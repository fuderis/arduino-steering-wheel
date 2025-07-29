use crate::prelude::*;

/// The input text properties
#[derive(Properties, PartialEq)]
pub struct TextProps {
    pub label: String,
    pub name: String,
    pub value: String,
    pub oninput: Callback<String>,
}

/// The input text component
#[function_component(Text)]
pub fn text(props: &TextProps) -> Html {
    let oninput = {
        let oninput = props.oninput.clone();

        Callback::from(move |e: InputEvent| {
            let input = e.target_unchecked_into::<web_sys::HtmlInputElement>();
            let val_str = input.value();
            
            oninput.emit(val_str);
        })
    };

    let id = nanoid!();

    html! {
        <div class="field text">
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
                    type="text"
                    name={str!(props.name)}
                    value={str!(props.value)}
                    {oninput}
                />
            </div>
        </div>
    }
}
