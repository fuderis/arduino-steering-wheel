use crate::prelude::*;

/// The input checkbox (switcher style) properties
#[derive(Properties, PartialEq)]
pub struct SwitchProps {
    pub label: String,
    pub name: String,
    pub checked: bool,
    pub oninput: Callback<bool>,
}

/// The input checkbox (switcher style) component
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
                        <label for={id.clone()} class="name">{str!(&props.label)}</label>
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
                <label for={id.clone()}></label>
            </div>
        </div>
    }
}
