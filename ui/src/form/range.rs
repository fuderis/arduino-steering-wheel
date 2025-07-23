use crate::prelude::*;
use super::Number;

#[derive(Properties, PartialEq)]
pub struct RangeProps {
    pub name: String,
    pub name_id: String,
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
                name={props.name_id.to_string()}
                min={props.min.to_string()}
                max={props.max.to_string()}
                step={props.step.to_string()}
                value={props.value.to_string()}
                {oninput}
            />
            <span class="name">{&props.name}</span>
            <Number
                name={None::<String>}
                name_id={props.name_id.clone()}
                min={props.min}
                max={props.max}
                step={5}
                value={props.value}
                oninput={props.oninput.clone()}
            />
        </range>
    }
}
