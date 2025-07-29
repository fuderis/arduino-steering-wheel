use crate::prelude::*;
use std::{ ops::{ Add, Sub }, str::FromStr };
use num_traits::ToPrimitive;

/// Input number properties
#[derive(Properties, PartialEq)]
pub struct NumberProps<T>
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

/// Input number component
#[function_component(Number)]
pub fn number<T>(props: &NumberProps<T>) -> Html
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

    let onclick_plus = {
        let (max, step) = (props.max, props.step);
        let value = props.value;
        let oninput = props.oninput.clone();

        Callback::from(move |_| {
            let next = value + step;
            let clamped = if next > max { max } else { next };
            
            oninput.emit(clamped);
        })
    };

    let onclick_minus = {
        let (min, step) = (props.min, props.step);
        let value = props.value;
        let oninput = props.oninput.clone();

        Callback::from(move |_| {
            let prev = value - step;
            let clamped = if prev < min { min } else { prev };
            
            oninput.emit(clamped);
        })
    };

    let id = nanoid!();

    html! {
        <div class="field number">
            {
                if !props.label.is_empty() {
                    html! {
                        <label class="name" for={id.clone()}>{str!(&props.label)}</label>
                    }
                } else {
                    html! {}
                }
            }
            <div class="container">
                <input
                    id={id.clone()}
                    type="number"
                    name={str!(props.name)}
                    value={str!(props.value)}
                    {oninput}
                />
                <div class="buttons">
                    <button name="plus" type="button" onclick={onclick_plus}></button>
                    <button name="minus" type="button" onclick={onclick_minus}></button>
                </div>
            </div>
        </div>
    }
}
