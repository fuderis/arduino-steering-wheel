use crate::prelude::*;

/// Input checkbox (switcher style) properties
#[derive(Properties, PartialEq)]
pub struct SwitchProps {
    pub label: String,
    pub name: String,
    pub enabled: String,
    pub disabled: String,
    pub checked: bool,
    pub oninput: Callback<bool>,
}

/// Input checkbox (switcher style) component
#[function_component(Switch)]
pub fn switch(props: &SwitchProps) -> Html {
    let oninput = {
        let oninput = props.oninput.clone();

        Callback::from(move |e: InputEvent| {
            let input = e.target_unchecked_into::<web_sys::HtmlInputElement>();
            let is_checked = input.checked();

            oninput.emit(is_checked);
        })
    };

    let id = nanoid!();

    html! {
        <div class="field switch">
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
                    type="checkbox"
                    name={str!(props.name)}
                    checked={props.checked}
                    {oninput}
                />
                {
                    if props.checked && !props.enabled.is_empty() {
                        html! {
                            <label class="status enabled" for={id.clone()}>
                                {&props.enabled}
                            </label>
                        }
                    } else if !props.checked && !props.disabled.is_empty() {
                        html! {
                            <label class="status disabled" for={id.clone()}>
                                {&props.disabled}
                            </label>
                        }
                    } else {
                        html! {}
                    }
                }
                <label class="checkbox" for={id.clone()}></label>
            </div>
        </div>
    }
}
