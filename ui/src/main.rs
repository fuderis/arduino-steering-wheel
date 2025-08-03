use app::{ prelude::*, form::{ Form, Field, FieldKind, FieldValue }, config::* };
use yew::Renderer;

/// The application component
#[function_component(App)]
fn app() -> Html {
    let config = use_state(|| Config::default());
    let is_loaded = use_state(|| false);

    // get config data:
    {
        let config = config.clone();
        let is_loaded = is_loaded.clone();
        use_effect_with((), move |_| {
            spawn_local(async move {
                let fetched_config = invoke_handler::<Config>("get_config", json!({})).await
                    .expect("Failed to get config");

                config.set(fetched_config);
                is_loaded.set(true);
            });
            || ()
        });
    }
    
    let oninput = Callback::from(|(form_name, fields): (String, Vec<Field>)| {
        let json_str = str!("{{ {} }}",
            fields.into_iter()
                .map(|field| {
                    let value = match field.name.as_ref() {
                        "baud_rate" => if let FieldValue::Str(s) = field.value {
                            FieldValue::Int(s.parse::<i32>().unwrap())
                        } else { panic!() },
                        _ => field.value
                    };

                    fmt!("\"{}\":{}", field.name, serde_json::to_string(&value).expect("Failed to parse form"))
                })
                .collect::<Vec<_>>()
                .join(",")
        );
        
        // web_sys::console::log_1(&fmt!("{json_str}").into());  // DEBUG: Input form
        
        spawn_local(async move {
            let _ = invoke_handler::<()>("update_config_part", json!({
                "name": form_name,
                "json": json_str
            })).await;
        });
    });

    let onsubmit = Callback::from(|(_, _): (String, Vec<Field>)| {});

    if !*is_loaded {
        return html! {
            <div>{ "Loading wheel configuration.." }</div>
        };
    }
    
    html! {
        <>
        <header>
            <h1 class="title">{"Steering Wheel (Arduino -> Xbox360)"}</h1>
        </header>
        
        <main>
            <div id="settings">
                <Form
                    name="comport-settings"
                    title="COM Port:"
                    fields={vec![
                        Field {
                            name: str!("com_port"),
                            label: str!("COM Port"),
                            kind: FieldKind::Number {
                                min: 0,
                                max: 9999,
                                step: 1,
                            },
                            value: FieldValue::Int(config.comport.com_port),
                        },
                        Field {
                            name: str!("baud_rate"),
                            label: str!("Baud rate"),
                            kind: FieldKind::Select {
                                items: vec![ 9600, 14400, 19200, 38400, 57600, 115200, 128000 ].into_iter()
                                    .map(|rate| (rate.to_string(), fmt!("{rate} bps")))
                                    .collect::<Vec<_>>(),
                            },
                            value: FieldValue::Str(config.comport.baud_rate.to_string()),
                        },
                    ]}
                    button=""
                    oninput={&oninput}
                    onsubmit={&onsubmit}
                />

                <Form
                    name="wheel-settings"
                    title="Wheel Settings:"
                    fields={vec![
                        Field {
                            name: str!("wheel_bias"),
                            label: str!("Center bias"),
                            kind: FieldKind::Range {
                                min: -255,
                                max: 255,
                                step: 1,
                            },
                            value: FieldValue::Int(config.wheel.wheel_bias),
                        },
                        Field {
                            name: str!("wheel_dead_zone"),
                            label: str!("Dead zone"),
                            kind: FieldKind::Range {
                                min: 0,
                                max: 255,
                                step: 1,
                            },
                            value: FieldValue::Int(config.wheel.wheel_dead_zone),
                        },
                        Field {
                            name: str!("wheel_degs_limit"),
                            label: str!("Degs limit"),
                            kind: FieldKind::Range {
                                min: 90,
                                max: 1080,
                                step: 30,
                            },
                            value: FieldValue::Int(config.wheel.wheel_degs_limit),
                        },
                        Field {
                            name: str!("wheel_degs_max_possible"),
                            label: str!("Degs (max possible)"),
                            kind: FieldKind::Range {
                                min: 180,
                                max: 1980,
                                step: 30,
                            },
                            value: FieldValue::Int(config.wheel.wheel_degs_max_possible),
                        },
                        Field {
                            name: str!("wheel_smooth_rate"),
                            label: str!("Smooth rate"),
                            kind: FieldKind::RangeFloat {
                                min: 0.0,
                                max: 0.95,
                                step: 0.05,
                            },
                            value: FieldValue::Float(config.wheel.wheel_smooth_rate),
                        },
                        Field {
                            name: str!("wheel_reverse_direction"),
                            label: str!("Reverse direction"),
                            kind: FieldKind::Check {
                                enabled: str!("Enabled"),
                                disabled: str!("Disabled"),
                            },
                            value: FieldValue::Bool(config.wheel.wheel_reverse_direction),
                        },
                    ]}
                    button=""
                    oninput={&oninput}
                    onsubmit={&onsubmit}
                />

                <Form
                    name="feedback-settings"
                    title="Feedback Settings:"
                    fields={vec![
                        Field {
                            name: str!("feedback_dead_zone"),
                            label: str!("Dead zone"),
                            kind: FieldKind::Range {
                                min: 0,
                                max: 255,
                                step: 1,
                            },
                            value: FieldValue::Int(config.feedback.feedback_dead_zone),
                        },
                        Field {
                            name: str!("feedback_min_power"),
                            label: str!("Min power"),
                            kind: FieldKind::Range {
                                min: 0,
                                max: 799,
                                step: 5,
                            },
                            value: FieldValue::Int(config.feedback.feedback_min_power),
                        },
                        Field {
                            name: str!("feedback_max_power"),
                            label: str!("Max power"),
                            kind: FieldKind::Range {
                                min: 0,
                                max: 799,
                                step: 5,
                            },
                            value: FieldValue::Int(config.feedback.feedback_max_power),
                        },
                        Field {
                            name: str!("feedback_exponent"),
                            label: str!("Exponent rate"),
                            kind: FieldKind::RangeFloat {
                                min: 1.0,
                                max: 5.0,
                                step: 0.05,
                            },
                            value: FieldValue::Float(config.feedback.feedback_exponent),
                        },
                    ]}
                    button=""
                    oninput={&oninput}
                    onsubmit={&onsubmit}
                />

                <Form
                    name="pedals-settings"
                    title="Pedals Settings:"
                    fields={vec![
                        // gas:
                        Field {
                            name: str!("gas_dead_zone"),
                            label: str!("Gas dead zone"),
                            kind: FieldKind::Range {
                                min: 0,
                                max: 255,
                                step: 1,
                            },
                            value: FieldValue::Int(config.pedals.gas_dead_zone),
                        },
                        Field {
                            name: str!("gas_value_limit"),
                            label: str!("Gas value limit"),
                            kind: FieldKind::Range {
                                min: 0,
                                max: 255,
                                step: 1,
                            },
                            value: FieldValue::Int(config.pedals.gas_value_limit),
                        },
                        Field {
                            name: str!("gas_smooth_rate"),
                            label: str!("Gas smooth rate"),
                            kind: FieldKind::RangeFloat {
                                min: 0.0,
                                max: 0.95,
                                step: 0.05,
                            },
                            value: FieldValue::Float(config.pedals.gas_smooth_rate),
                        },
                        // brake:
                        Field {
                            name: str!("brake_dead_zone"),
                            label: str!("Brake dead zone"),
                            kind: FieldKind::Range {
                                min: 0,
                                max: 255,
                                step: 1,
                            },
                            value: FieldValue::Int(config.pedals.brake_dead_zone),
                        },
                        Field {
                            name: str!("brake_value_limit"),
                            label: str!("Brake value limit"),
                            kind: FieldKind::Range {
                                min: 0,
                                max: 255,
                                step: 1,
                            },
                            value: FieldValue::Int(config.pedals.brake_value_limit),
                        },
                        Field {
                            name: str!("brake_smooth_rate"),
                            label: str!("Brake smooth rate"),
                            kind: FieldKind::RangeFloat {
                                min: 0.0,
                                max: 0.95,
                                step: 0.05,
                            },
                            value: FieldValue::Float(config.pedals.brake_smooth_rate),
                        },
                        // clutch:
                        Field {
                            name: str!("clutch_dead_zone"),
                            label: str!("Clutch dead zone"),
                            kind: FieldKind::Range {
                                min: 0,
                                max: 255,
                                step: 1,
                            },
                            value: FieldValue::Int(config.pedals.clutch_dead_zone),
                        },
                        Field {
                            name: str!("clutch_value_limit"),
                            label: str!("Clutch value limit"),
                            kind: FieldKind::Range {
                                min: 0,
                                max: 255,
                                step: 1,
                            },
                            value: FieldValue::Int(config.pedals.clutch_value_limit),
                        },
                        Field {
                            name: str!("clutch_smooth_rate"),
                            label: str!("Clutch smooth rate"),
                            kind: FieldKind::RangeFloat {
                                min: 0.0,
                                max: 0.95,
                                step: 0.05,
                            },
                            value: FieldValue::Float(config.pedals.clutch_smooth_rate),
                        },
                    ]}
                    button=""
                    oninput={&oninput}
                    onsubmit={&onsubmit}
                />
            </div>
        </main>

        <footer>
            <a class="link" href="https://github.com/fuderis/steering-wheel.git" target="_blank">
                {"Open Project Repository"}
            </a>
        </footer>
        </>
    }
}

fn main() {
    Renderer::<App>::new().render();
}
