use super::matrices::Channels;
use super::rgb_conversion::{decode_srgb, encode_srgb};

fn hue_rgb(hue: f64) -> Channels {

    let h = hue.rem_euclid(360.0) / 60.0;

    let x = 1.0 - (h.rem_euclid(2.0) - 1.0).abs();

    match h as u8 {
        0 => [1.0, x, 0.0],
        1 => [x, 1.0, 0.0],
        2 => [0.0, 1.0, x],
        3 => [0.0, x, 1.0],
        4 => [x, 0.0, 1.0],
        _ => [1.0, 0.0, x],
    }
}

fn hue_min_max([r, g, b]: Channels) -> (f64, f64, f64) {

    let min = r.min(g).min(b);

    let max = r.max(g).max(b);

    let delta = max - min;

    let hue = if delta == 0.0 {

        0.0
    } else if max == r {

        60.0 * ((g - b) / delta).rem_euclid(6.0)
    } else if max == g {

        60.0 * ((b - r) / delta + 2.0)
    } else {

        60.0 * ((r - g) / delta + 4.0)
    };

    (hue, min, max)
}

pub(super) fn hsl_to_linear([h, s, l]: Channels) -> Channels {

    let c = (1.0 - (2.0 * l - 1.0).abs()) * s;

    let m = l - c / 2.0;

    hue_rgb(h).map(|v| decode_srgb(v * c + m))
}

pub(super) fn linear_to_hsl(v: Channels) -> Channels {

    let (h, min, max) = hue_min_max(v.map(encode_srgb));

    let l = (min + max) / 2.0;

    let delta = max - min;

    let s = if delta == 0.0 {

        0.0
    } else {

        delta / (1.0 - (2.0 * l - 1.0).abs())
    };

    [h, s, l]
}

pub(super) fn hsv_to_linear([h, s, v]: Channels) -> Channels {

    let c = v * s;

    hue_rgb(h).map(|channel| decode_srgb(channel * c + v - c))
}

pub(super) fn linear_to_hsv(v: Channels) -> Channels {

    let (h, min, max) = hue_min_max(v.map(encode_srgb));

    let delta = max - min;

    let s = if delta == 0.0 { 0.0 } else { delta / max };

    [h, s, max]
}

pub(super) fn hwb_to_linear([h, w, b]: Channels) -> Channels {

    // CSS HWB normalizes overlapping whiteness and blackness to a gray.
    if w + b >= 1.0 {

        return [decode_srgb(w / (w + b)); 3];
    }

    hue_rgb(h).map(|v| decode_srgb(v * (1.0 - w - b) + w))
}

pub(super) fn linear_to_hwb(v: Channels) -> Channels {

    let (h, min, max) = hue_min_max(v.map(encode_srgb));

    [h, min, 1.0 - max]
}
