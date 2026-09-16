use super::matrices::*;
use super::rgb_conversion::{linear_to_xyz, linear_to_xyz50, xyz_to_linear, xyz50_to_linear};

const WHITE_D50: Channels = [0.3457 / 0.3585, 1.0, (1.0 - 0.3457 - 0.3585) / 0.3585];

const EPSILON: f64 = 216.0 / 24389.0;

const KAPPA: f64 = 24389.0 / 27.0;

pub(super) fn linear_to_lab(v: Channels) -> Channels {
    let xyz = linear_to_xyz50(v);

    let f = std::array::from_fn::<_, 3, _>(|i| {
        let v = xyz[i] / WHITE_D50[i];

        if v > EPSILON {
            v.cbrt()
        } else {
            (KAPPA * v + 16.0) / 116.0
        }
    });

    [
        116.0 * f[1] - 16.0,
        500.0 * (f[0] - f[1]),
        200.0 * (f[1] - f[2]),
    ]
}

pub(super) fn lab_to_linear([l, a, b]: Channels) -> Channels {
    let fy = (l + 16.0) / 116.0;

    let f = [a / 500.0 + fy, fy, fy - b / 200.0];

    let xyz = std::array::from_fn(|i| {
        let cube = f[i].powi(3);

        let v = if cube > EPSILON {
            cube
        } else {
            (116.0 * f[i] - 16.0) / KAPPA
        };

        v * WHITE_D50[i]
    });

    xyz50_to_linear(xyz)
}

pub(super) fn linear_to_lms(v: Channels) -> Channels {
    multiply(XYZ_TO_LMS, linear_to_xyz(v))
}

pub(super) fn lms_to_linear(v: Channels) -> Channels {
    xyz_to_linear(multiply(LMS_TO_XYZ, v))
}

pub(super) fn linear_to_lms_prime(v: Channels) -> Channels {
    linear_to_lms(v).map(f64::cbrt)
}

pub(super) fn lms_prime_to_linear(v: Channels) -> Channels {
    lms_to_linear(v.map(|v| v.powi(3)))
}

pub(super) fn linear_to_oklab(v: Channels) -> Channels {
    multiply(LMS_PRIME_TO_OKLAB, linear_to_lms_prime(v))
}

pub(super) fn oklab_to_linear(v: Channels) -> Channels {
    lms_prime_to_linear(multiply(OKLAB_TO_LMS_PRIME, v))
}

fn to_polar([l, a, b]: Channels) -> Channels {
    let c = a.hypot(b);

    // Zero hue is the canonical finite representation of an achromatic color.
    let h = if c == 0.0 {
        0.0
    } else {
        b.atan2(a).to_degrees().rem_euclid(360.0)
    };

    [l, c, h]
}

fn from_polar([l, c, h]: Channels) -> Channels {
    let h = h.rem_euclid(360.0).to_radians();

    [l, c * h.cos(), c * h.sin()]
}

pub(super) fn linear_to_lch(v: Channels) -> Channels {
    to_polar(linear_to_lab(v))
}

pub(super) fn lch_to_linear(v: Channels) -> Channels {
    lab_to_linear(from_polar(v))
}

pub(super) fn linear_to_oklch(v: Channels) -> Channels {
    to_polar(linear_to_oklab(v))
}

pub(super) fn oklch_to_linear(v: Channels) -> Channels {
    oklab_to_linear(from_polar(v))
}
