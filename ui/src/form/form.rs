use crate::prelude::*;
use super::*;
use web_sys::SubmitEvent;

#[derive(Properties, PartialEq)]
pub struct FormProps {
    pub name: String,
    pub title: String,
    pub fields: Vec<Field>,
    pub button: String,
    pub oninput: Callback<(String, Vec<Field>)>,
    pub onsubmit: Callback<(String, Vec<Field>)>,
}

#[function_component(Form)]
pub fn form(props: &FormProps) -> Html {
    let fields_state = use_state(|| props.fields.clone());

    let oninput_field = {
        let form_name = props.name.clone();
        let fields_state = fields_state.clone();
        let oninput = props.oninput.clone();

        Callback::from(move |(name, value): (String, FieldValue)| {
            let form_name = form_name.clone();
            let mut new_fields = (*fields_state).clone();

            for field in new_fields.iter_mut() {
                if field.name == name {
                    field.value = value.clone();
                }
            }

            fields_state.set(new_fields.clone());
            oninput.emit((form_name, new_fields));
        })
    };
    
    let onsubmit = {
        let form_name = props.name.clone();
        let fields_state = fields_state.clone();
        let onsubmit = props.onsubmit.clone();

        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            let form_name = form_name.clone();
            onsubmit.emit((form_name, (*fields_state).clone()));
        })
    };

    html! {
        <form name={props.name.clone()} {onsubmit}>
            <h4 class="title">{ &props.title }</h4>
            {
                for fields_state.iter().map(|Field { name, label, kind, value }| {
                    let name = name.clone();
                    let label = label.clone();
                    let value = value.clone();
                    let oninput = oninput_field.clone();
                    
                    match &kind {
                        FieldKind::Number { min, max, step } => {
                            let value = match value {
                                FieldValue::Int(v) => v,
                                _ => *min,
                            };
                            let oninput = {
                                let name = name.clone();
                                let oninput = oninput.clone();

                                Callback::from(move |v: i32| {
                                    oninput.emit((name.clone(), FieldValue::Int(v)));
                                })
                            };

                            html! {
                                <Number<i32>
                                    label={label.clone()}
                                    name={name.clone()}
                                    min={min}
                                    max={max}
                                    step={step}
                                    value={value}
                                    oninput={oninput}
                                />
                            }
                        }

                        FieldKind::Select { items } => {
                            let value = match &value {
                                FieldValue::Str(s) => s.clone(),
                                _ => "".into(),
                            };
                            let oninput = {
                                let name = name.clone();
                                let oninput = oninput.clone();

                                Callback::from(move |v: String| {
                                    oninput.emit((name.clone(), FieldValue::Str(v)));
                                })
                            };

                            html! {
                                <Select
                                    label={label.clone()}
                                    name={name.clone()}
                                    items={items.clone()}
                                    active={value}
                                    oninput={oninput}
                                />
                            }
                        }

                        FieldKind::Range { min, max, step } => {
                            let value = match value {
                                FieldValue::Int(v) => v,
                                _ => *min,
                            };
                            let oninput = {
                                let name = name.clone();
                                let oninput = oninput.clone();

                                Callback::from(move |v: i32| {
                                    oninput.emit((name.clone(), FieldValue::Int(v)));
                                })
                            };

                            html! {
                                <Range<i32>
                                    label={label.clone()}
                                    name={name.clone()}
                                    min={min}
                                    max={max}
                                    step={step}
                                    value={value}
                                    oninput={oninput}
                                />
                            }
                        }
                    }
                })
            }
            {
                if !props.button.is_empty() {
                    html! {
                        <button type="submit">{str!(&props.button)}</button>
                    }
                } else {
                    html! {}
                }
            }
        </form>
    }
}
