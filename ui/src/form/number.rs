use crate::prelude::*;

#[derive(Properties, PartialEq)]
pub struct NumberProps {
    pub name: Option<String>,
    pub name_id: String,
    pub min: i32,
    pub max: i32,
    pub step: i32,
    pub value: i32,
    pub oninput: Callback<i32>,
}

#[function_component(Number)]
pub fn number(props: &NumberProps) -> Html {
    let oninput = {
        let (min, max) = (props.min.clone(), props.max.clone());
        let oninput = props.oninput.clone();
        Callback::from(move |e: InputEvent| {
            let input = e.target_unchecked_into::<web_sys::HtmlInputElement>();
            oninput.emit((input.value_as_number() as i32).clamp(min, max));
        })
    };

    let onclick_plus = {
        let (min, max, step) = (props.min.clone(), props.max.clone(), props.step.clone());
        let value = props.value.clone();
        let oninput = props.oninput.clone();
        Callback::from(move |_| {
            oninput.emit((value + step).clamp(min, max));
        })
    };

    let onclick_minus = {
        let (min, max, step) = (props.min.clone(), props.max.clone(), props.step.clone());
        let value = props.value.clone();
        let oninput = props.oninput.clone();
        Callback::from(move |_| {
            oninput.emit((value - step).clamp(min, max));
        })
    };

    html! {
        <number>
            {
                if let Some(name) = &props.name {
                    html!{ <span class="name">{name}</span> }
                } else {
                    html!{}
                }
            }
            <div class="container">
                <input
                    type="number"
                    name={props.name_id.to_string()}
                    value={props.value.to_string()}
                    {oninput}
                />
                <div class="buttons">
                    <button name="plus" type="button" onclick={onclick_plus}></button>
                    <button name="minus" type="button" onclick={onclick_minus}></button>
                </div>
            </div>
        </number>
    }
}
