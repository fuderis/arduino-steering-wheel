pub mod text;       pub(super) use text::Text;
pub mod number;     pub(super) use number::Number;
pub mod range;      pub(super) use range::Range;
pub mod select;     pub(super) use select::Select;
pub mod check;      pub(super) use check::Check;
pub mod switch;     pub(super) use switch::Switch;

pub mod form;       pub use form::Form;
pub mod field;      pub use field::{ Field, FieldKind, FieldValue };

use std::str::FromStr;
use num_traits::ToPrimitive;

// Rounds float number
pub(super) fn round_to_2<T>(value: T) -> T
where
    T: ToPrimitive + FromStr + Copy + ToString,
{
    if let Some(f) = value.to_f64() {
        let rounded = (f * 100.0).round() / 100.0;
        if let Ok(res) = T::from_str(&rounded.to_string()) {
            return res;
        }
    }

    value
}
