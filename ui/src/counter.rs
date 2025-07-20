use prelude::*;

/// The counter component
#[function_component(Counter)]
pub fn counter() -> Html {
    let count = use_state(|| 0u32);

    // get start counter value:
    {
        let count = count.clone();
        use_effect_with((), move |_| {
            spawn_local(async move {
                match invoke_handler("get_count", to_value(&serde_json::json!({})).unwrap()).await {
                    Ok(js_val) => {
                        if let Ok(current) = from_value::<u32>(js_val) {
                            count.set(current);
                        }
                    }
                    Err(e) => {
                        web_sys::console::error_1(&e);
                    }
                }
            });
            || ()
        });
    }

    // gen button onclick:
    let onclick = {
        let count = count.clone();
        Callback::from(move |_event: MouseEvent| {
            let count = count.clone();
            spawn_local(async move {
                match invoke_handler("plus_count", to_value(&serde_json::json!({"step": 1})).unwrap()).await {
                    Ok(js_val) => {
                        if let Ok(updated) = from_value::<u32>(js_val) {
                            count.set(updated);
                        }
                    }
                    Err(e) => {
                        web_sys::console::error_1(&e);
                    }
                }
            });
        })
    };

    html! {
        <div id="counter">
            <p>{"Counter: "}<span id="count">{*count}</span></p>
            <button id="counter-button" {onclick}>{"Click me!"}</button>
        </div>
    }
}
