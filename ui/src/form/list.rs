use crate::prelude::*;

#[derive(Properties, PartialEq)]
pub struct ListProps {
    pub name: String,
    pub items: Vec<(String, String)>, // (value, text)
    pub active: String,
    pub oninput: Callback<String>,
}

#[function_component(List)]
pub fn list(props: &ListProps) -> Html {
    let active = use_state(|| props.active.clone());
    let expanded = use_state(|| false);

    let onclick = {
        let expanded = expanded.clone();
        Callback::from(move |_| expanded.set(!*expanded))
    };

    let onselect = {
        let active = active.clone();
        let expanded = expanded.clone();
        let oninput = props.oninput.clone();
        Callback::from(move |val: String| {
            active.set(val.clone());
            expanded.set(false);
            oninput.emit(val);
        })
    };

    html! {
        <list tabindex="0">
            <div class="selected" {onclick}>
                {
                    props.items.iter().find(|(value, _)| value == &*active).unwrap().1.clone()
                }
                <span class="icon"></span>
            </div>
            <div class="container" style={format!("display: {}", if *expanded {"flex"} else {"none"})}>
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
                                    name={props.name.clone()}
                                    value={value.clone()}
                                    type="radio"
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
        </list>
    }
}
