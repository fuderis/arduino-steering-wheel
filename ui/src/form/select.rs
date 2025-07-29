use crate::prelude::*;

/// Input select properties
#[derive(Properties, PartialEq)]
pub struct SelectProps {
    pub label: String,
    pub name: String,
    pub items: Vec<(String, String)>, // (value, text)
    pub active: String,
    pub oninput: Callback<String>,
}

/// Input select component
#[function_component(Select)]
pub fn select(props: &SelectProps) -> Html {
    let active = use_state(|| props.active.clone());
    let expanded = use_state(|| false);

    let onclick = {
        let expanded = expanded.clone();
        Callback::from(move |_| expanded.set(!*expanded))
    };

    let onselect = {
        let (active, expanded) = (active.clone(), expanded.clone());
        let oninput = props.oninput.clone();

        Callback::from(move |val: String| {
            active.set(val.clone());
            expanded.set(false);
            oninput.emit(val);
        })
    };

    let id = nanoid!();

    html! {
        <div class="field select">
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
                <div class="selector">
                    <label for={id.clone()} class="selected">
                        {
                            props.items.iter().find(|(value, _)| value == &*active).unwrap().1.clone()
                        }
                    </label>
                    <div class="items" style={format!("display: {}", if *expanded {"flex"} else {"none"})}>
                        {
                            for props.items.iter().map(|(value, text)| {
                                let value2 = value.clone();
                                let onselect_item = {
                                    let onselect = onselect.clone();
                                    Callback::from(move |_| onselect.emit(value2.clone()))
                                };

                                let mut classes = Classes::from("item");
                                if *active == *value {
                                    classes.push("active");
                                }

                                html! {
                                    <label class={classes}>
                                        <input
                                            type="radio"
                                            name={props.name.clone()}
                                            value={value.clone()}
                                            checked={*active == *value}
                                            onclick={onselect_item}
                                            style="display:none"
                                        />
                                        { text }
                                    </label>
                                }
                            })
                        }
                    </div>
                </div>
                <button
                    id={id.clone()}
                    type="button"
                    name="visible"
                    {onclick}
                ></button>
            </div>
        </div>
    }
}
