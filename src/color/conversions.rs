use crate::color::matrices::Channels;
use crate::color::{ColorError, ColorResult};

pub(super) fn checked(values: Channels, names: [&'static str; 3]) -> ColorResult<[f32; 3]> {
    let values = values.map(|v| v as f32);
    for (value, name) in values.into_iter().zip(names) {
        if !value.is_finite() {
            return Err(ColorError::InvalidColorChannel(name, value));
        }
    }
    Ok(values)
}

// Validate both public input fields and f64 results narrowed to f32. All trait
// entry points share these checks, including the explicitly unclamped raw methods.
macro_rules! impl_colorspace {
    (infallible $ty:ty, [$a:ident, $b:ident, $c:ident], $to:path, $from:path $(, $normalize:ident)?) => {
        crate::color::conversions::impl_colorspace!(@common $ty, [$a, $b, $c], $to $(, $normalize)?);

        impl From<super::LinearSrgb> for $ty {
            fn from(color: super::LinearSrgb) -> Self {
                // These transforms keep every finite f32 input representable.
                // See finite_linear_inputs_have_infallible_named_conversions.
                let input = [color.r(), color.g(), color.b()].map(f64::from);
                let [$a, $b, $c] = $from(input).map(|value| value as f32);
                Self::new($a, $b, $c)
            }
        }
    };
    ($ty:ty, [$a:ident, $b:ident, $c:ident], $to:path, $from:path $(, $normalize:ident)?) => {
        crate::color::conversions::impl_colorspace!(@common $ty, [$a, $b, $c], $to $(, $normalize)?);

        impl TryFrom<super::LinearSrgb> for $ty {
            type Error = crate::color::ColorError;
            fn try_from(color: super::LinearSrgb) -> Result<Self, Self::Error> {
                let input = [color.r(), color.g(), color.b()].map(f64::from);
                let [$a, $b, $c] = crate::color::conversions::checked(
                    $from(input),
                    [stringify!($a), stringify!($b), stringify!($c)],
                )?;
                Ok(Self::new($a, $b, $c))
            }
        }
    };
    (@common $ty:ty, [$a:ident, $b:ident, $c:ident], $to:path $(, $normalize:ident)?) => {
        impl super::ColorSpace for $ty {
            fn try_into_linear_srgb_raw(self) -> crate::color::ColorResult<super::LinearSrgb> {
                let input = [*self.$a, *self.$b, *self.$c].map(f64::from);
                crate::color::conversions::checked(
                    input,
                    [stringify!($a), stringify!($b), stringify!($c)],
                )?;
                let [r, g, b] = crate::color::conversions::checked($to(input), ["r", "g", "b"])?;
                super::LinearSrgb::new(r, g, b)
            }

            fn try_from_linear_srgb_raw(
                color: super::LinearSrgb,
            ) -> crate::color::ColorResult<Self> {
                Self::try_from(color).map_err(Into::into)
            }
        }

        impl TryFrom<$ty> for super::LinearSrgb {
            type Error = crate::color::ColorError;
            fn try_from(color: $ty) -> Result<Self, Self::Error> {
                <$ty as super::ColorSpace>::try_into_linear_srgb_raw(color)
            }
        }

        impl crate::color::Clamp for $ty {
            fn clamp(mut self) -> Self {
                self.$a = self.$a.clamp();
                self.$b = self.$b.clamp();
                self.$c = self.$c.clamp();
                $(self.$normalize();)?
                self
            }
        }
    };
}

pub(super) use impl_colorspace;
