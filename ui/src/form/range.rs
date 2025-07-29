use crate::prelude::*;
use super::Number;
use std::{ ops::{ Add, Sub }, str::FromStr };
use num_traits::ToPrimitive;

/// The input range properties
#[derive(Properties, PartialEq)]
pub struct RangeProps<T>
where
    T: ToPrimitive + Copy + PartialOrd + Add<Output = T> + Sub<Output = T> + FromStr + ToString + 'static,
{
    pub label: String,
    pub name: String,
    pub min: T,
    pub max: T,
    pub step: T,
    pub value: T,
    pub oninput: Callback<T>,
}

/// The input range component
#[function_component(Range)]
pub fn range<T>(props: &RangeProps<T>) -> Html
where
    T: ToPrimitive + Copy + PartialOrd + Add<Output = T> + Sub<Output = T> + FromStr + ToString + 'static,
{
    let oninput = {
        let (min, max) = (props.min, props.max);
        let oninput = props.oninput.clone();

        Callback::from(move |e: InputEvent| {
            let input = e.target_unchecked_into::<web_sys::HtmlInputElement>();
            let val_str = input.value();

            if let Ok(val) = T::from_str(&val_str) {
                let clamped = if val < min {
                    min
                } else if val > max {
                    max
                } else {
                    val
                };

                oninput.emit(clamped);
            }
        })
    };

    let percent = (props.value.to_f64().unwrap_or_default() / props.max.to_f64().unwrap_or(1.0)) * 100.0;
    let style = fmt!("--percent:{percent}%;");

    let id = nanoid!();

    html! {
        <div class="field range" {style}>
            {
                if !props.label.is_empty() {
                    html! {
                        <label for={id.clone()} class="name">{str!(&props.label)}</label>
                    }
                } else {
                    html! {}
                }
            }
            <input
                id={id.clone()}
                type="range"
                name={str!(props.name)}
                min={str!(props.min)}
                max={str!(props.max)}
                step={str!(props.step)}
                value={str!(props.value)}
                {oninput}
            />
            <Number<T>
                label={str!()}
                name={str!()}
                min={props.min}
                max={props.max}
                step={props.step}
                value={props.value}
                oninput={props.oninput.clone()}
            />
        </div>
    }
}
