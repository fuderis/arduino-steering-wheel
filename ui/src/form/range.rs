use crate::prelude::*;
use super::Number;

#[derive(Properties, PartialEq)]
pub struct RangeProps {
    pub name: String,
    pub min: i32,
    pub max: i32,
    pub step: i32,
    pub value: i32,
    pub oninput: Callback<i32>,
}

#[function_component(Range)]
pub fn range(props: &RangeProps) -> Html {
    let oninput = {
        let oninput = props.oninput.clone();
        Callback::from(move |e: InputEvent| {
            let input = e.target_unchecked_into::<web_sys::HtmlInputElement>();
            oninput.emit(input.value_as_number() as i32);
        })
    };

    let percent = (props.value as f64 / props.max as f64) * 100.0;
    let style = fmt!("--percent:{percent}%;", );

    html! {
        <range {style}>
            <input
                type="range"
                name={str!(props.name)}
                min={str!(props.min)}
                max={str!(props.max)}
                step={str!(props.step)}
                value={str!(props.value)}
                {oninput}
            />
            <Number
                name={str!()}
                min={props.min}
                max={props.max}
                step={props.step}
                value={props.value}
                oninput={props.oninput.clone()}
            />
        </range>
    }
}
