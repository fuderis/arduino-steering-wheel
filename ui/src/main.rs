use app::{ prelude::*, form::{ Range, Number } };

/// The application component
#[function_component(App)]
fn app() -> Html {
    let com_port = use_state(|| 6);
    let oninput_com_port = {
        let com_port = com_port.clone();
        Callback::from(move |val: i32| com_port.set(val))
    };

    let dead_zone = use_state(|| 4);
    let oninput_dead_zone = {
        let dead_zone = dead_zone.clone();
        Callback::from(move |val: i32| dead_zone.set(val))
    };
    
    html! {
        <main id="main">
            <h1 id="title">{"Steering Wheel (Arduino -> Xbox360)"}</h1>

            <div id="forms-container">
                <form id="com-port-settings">
                    <Number
                        name={Some("COM port identifier")}
                        name_id={"com_port"}
                        min={0}
                        max={9999}
                        step={1}
                        value={*com_port}
                        oninput={oninput_com_port}
                    />
                </form>
                <form id="wheel-settings">
                    <Range
                        name={"Dead zone"}
                        name_id={"dead_zone"}
                        min={0}
                        max={510}
                        step={1}
                        value={*dead_zone}
                        oninput={oninput_dead_zone}
                    />
                </form>
            </div>
        </main>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
