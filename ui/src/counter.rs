use crate::prelude::*;

/// The counter component
#[function_component(Counter)]
pub fn counter() -> Html {
    let count = use_state(|| 0u32);

    // get start counter value:
    {
        let count = count.clone();
        use_effect_with((), move |_| {
            spawn_local(async move {
                match invoke_handler::<u32>("get_count", json!({})).await {
                    Ok(current) => count.set(current),
                    _ => {}
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
                match invoke_handler::<u32>("plus_count", json!({ "step": 1 })).await {
                    Ok(updated) => count.set(updated),
                    _ => {}
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
