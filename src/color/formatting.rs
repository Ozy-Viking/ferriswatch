use std::fmt;

// All formatters write the opening function and channels before this helper adds alpha.
pub(super) fn finish(f: &mut fmt::Formatter<'_>, alpha: Option<f32>) -> fmt::Result {
    if let Some(alpha) = alpha {
        f.write_str(" / ")?;

        channel(f, f64::from(alpha), "")?;
    }

    f.write_str(")")
}

pub(super) fn channel(f: &mut fmt::Formatter<'_>, value: f64, unit: &str) -> fmt::Result {
    if !value.is_finite() {
        return f.write_str("none");
    }

    let precision = f.precision().unwrap_or(3);

    write!(f, "{value:.precision$}{unit}")
}

macro_rules! impl_display {
    ($ty:ty, $prefix:literal, |$color:ident| [$($value:expr => $unit:literal),+ $(,)?]) => {
        impl $ty {
            fn fmt_color(
                &self,
                f: &mut std::fmt::Formatter<'_>,
                alpha: Option<f32>,
            ) -> std::fmt::Result {
                let $color = self;
                f.write_str($prefix)?;
                let mut separator = "";
                $(
                    f.write_str(separator)?;
                    crate::color::formatting::channel(f, ($value) as f64, $unit)?;
                    separator = " ";
                )+
                let _ = separator;
                crate::color::formatting::finish(f, alpha)
            }
        }

        impl std::fmt::Display for $ty {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                self.fmt_color(f, None)
            }
        }

        impl std::fmt::Display for crate::color::Alpha<$ty> {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                self.color().fmt_color(f, Some(self.alpha()))
            }
        }
    };
}

pub(super) use impl_display;
