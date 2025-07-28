use app::{ prelude::*, form::{ Form, Field, FieldKind, FieldValue } };
use yew::Renderer;

/// The application component
#[function_component(App)]
fn app() -> Html {
    let oninput = Callback::from(|(form_name, fields): (String, Vec<Field>)| {
        for field in fields {
            web_sys::console::log_1(&format!("{form_name}: {} = {:?}", field.name, field.value).into());
        }
    });

    let onsubmit = Callback::from(|(form_name, fields): (String, Vec<Field>)| {
        for field in fields {
            web_sys::console::log_1(&format!("{form_name}: {} = {:?}", field.name, field.value).into());
        }
    });
    
    html! {
        <>
        <header>
            <h1 class="title">{"Steering Wheel (Arduino -> Xbox360)"}</h1>
        </header>
        
        <main>
            <div id="settings">
                <Form
                    name="com-port-settings"
                    title={"COM Port:"}
                    fields={vec![
                        Field {
                            name: str!("com_port"),
                            label: str!("COM Port"),
                            kind: FieldKind::Number {
                                min: 0,
                                max: 9999,
                                step: 1,
                            },
                            value: FieldValue::Int(6),
                        },
                        Field {
                            name: str!("baud_rate"),
                            label: str!("Baud rate"),
                            kind: FieldKind::Select {
                                items: vec![ 9600, 14400, 19200, 38400, 57600, 115200, 128000 ].into_iter()
                                    .map(|rate| (rate.to_string(), fmt!("{rate} bps")))
                                    .collect::<Vec<_>>(),
                            },
                            value: FieldValue::Str(str!("115200")),
                        },
                    ]}
                    button="Save"
                    oninput={&oninput}
                    onsubmit={&onsubmit}
                />

                <Form
                    name="wheel-settings"
                    title={"Wheel Settings:"}
                    fields={vec![
                        Field {
                            name: str!("dead_zone"),
                            label: str!("Dead zone"),
                            kind: FieldKind::Range {
                                min: 0,
                                max: 255,
                                step: 5,
                            },
                            value: FieldValue::Int(5),
                        },
                    ]}
                    button="Save"
                    oninput={&oninput}
                    onsubmit={&onsubmit}
                />
            </div>
        </main>
        </>
    }
}

fn main() {
    Renderer::<App>::new().render();
}
