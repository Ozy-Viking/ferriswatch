use super::matrices::*;

pub(super) fn decode_srgb(v: f64) -> f64 {

    if v.abs() <= 0.04045 {

        v / 12.92
    } else {

        v.signum() * ((v.abs() + 0.055) / 1.055).powf(2.4)
    }
}

pub(super) fn encode_srgb(v: f64) -> f64 {

    if v.abs() <= 0.0031308 {

        12.92 * v
    } else {

        v.signum() * (1.055 * v.abs().powf(1.0 / 2.4) - 0.055)
    }
}

fn signed_power(v: f64, exponent: f64) -> f64 {

    v.signum() * v.abs().powf(exponent)
}

pub(super) fn xyz_to_linear(v: Channels) -> Channels {

    multiply(XYZ_TO_SRGB, v)
}

pub(super) fn linear_to_xyz(v: Channels) -> Channels {

    multiply(SRGB_TO_XYZ, v)
}

pub(super) fn xyz50_to_linear(v: Channels) -> Channels {

    xyz_to_linear(multiply(D50_TO_D65, v))
}

pub(super) fn linear_to_xyz50(v: Channels) -> Channels {

    multiply(D65_TO_D50, linear_to_xyz(v))
}

pub(super) fn p3_to_linear(v: Channels) -> Channels {

    xyz_to_linear(multiply(P3_TO_XYZ, v.map(decode_srgb)))
}

pub(super) fn linear_to_p3(v: Channels) -> Channels {

    multiply(XYZ_TO_P3, linear_to_xyz(v)).map(encode_srgb)
}

pub(super) fn a98_to_linear(v: Channels) -> Channels {

    xyz_to_linear(multiply(
        A98_TO_XYZ,
        v.map(|v| signed_power(v, 563.0 / 256.0)),
    ))
}

pub(super) fn linear_to_a98(v: Channels) -> Channels {

    multiply(XYZ_TO_A98, linear_to_xyz(v)).map(|v| signed_power(v, 256.0 / 563.0))
}

// CSS Color 4 uses the BT.1886 reference EOTF for rec2020, gamma 2.4.
pub(super) fn rec2020_to_linear(v: Channels) -> Channels {

    xyz_to_linear(multiply(REC2020_TO_XYZ, v.map(|v| signed_power(v, 2.4))))
}

pub(super) fn linear_to_rec2020(v: Channels) -> Channels {

    multiply(XYZ_TO_REC2020, linear_to_xyz(v)).map(|v| signed_power(v, 1.0 / 2.4))
}

fn decode_prophoto(v: f64) -> f64 {

    if v.abs() <= 16.0 / 512.0 {

        v / 16.0
    } else {

        signed_power(v, 1.8)
    }
}

fn encode_prophoto(v: f64) -> f64 {

    if v.abs() < 1.0 / 512.0 {

        v * 16.0
    } else {

        signed_power(v, 1.0 / 1.8)
    }
}

pub(super) fn prophoto_to_linear(v: Channels) -> Channels {

    xyz50_to_linear(multiply(PROPHOTO_TO_XYZ, v.map(decode_prophoto)))
}

pub(super) fn linear_to_prophoto(v: Channels) -> Channels {

    multiply(XYZ_TO_PROPHOTO, linear_to_xyz50(v)).map(encode_prophoto)
}
