use app::{ prelude::*, form::{ Range, Number, List } };

/// The application component
#[function_component(App)]
fn app() -> Html {
    let com_port = use_state(|| 6);
    let oninput_com_port = {
        let com_port = com_port.clone();
        Callback::from(move |value: i32| com_port.set(value))
    };

    let baud_rate = use_state(|| 115200);
    let oninput_baud_rate = {
        let baud_rate = baud_rate.clone();
        Callback::from(move |value: String| baud_rate.set(value.parse::<i32>().unwrap()))
    };

    let dead_zone = use_state(|| 4);
    let oninput_dead_zone = {
        let dead_zone = dead_zone.clone();
        Callback::from(move |value: i32| dead_zone.set(value))
    };
    
    html! {
        <>
        <header>
            <h1 class="title">{"Steering Wheel (Arduino -> Xbox360)"}</h1>
        </header>
        
        <main>
            <div id="settings">
                <form id="com-port-settings">
                    <h4 class="title">{"COM Port:"}</h4>
                    <div class="field">
                        <span class="name">{"COM Port"}</span>
                        <Number
                            name={"com_port"}
                            min={0}
                            max={9999}
                            step={1}
                            value={*com_port}
                            oninput={oninput_com_port}
                        />
                    </div>
                    <div class="field">
                        <span class="name">{"Baud rate"}</span>
                        <List
                            name={"baud_rate"}
                            items={vec![ 
                                (str!("9600"), str!("9600 bps")),
                                (str!("115200"), str!("115200 bps")),
                            ]}
                            active={str!(baud_rate)}
                            oninput={oninput_baud_rate}
                        />
                    </div>
                </form>
                <form id="wheel-settings">
                    <h4 class="title">{"Wheel Settings:"}</h4>
                    <div class="field merged">
                        <span class="name">{"Dead zone"}</span>
                        <Range
                            name={"dead_zone"}
                            min={0}
                            max={510}
                            step={1}
                            value={*dead_zone}
                            oninput={oninput_dead_zone}
                        />
                    </div>
                </form>
            </div>
        </main>
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
