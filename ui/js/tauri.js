export async function js_invoke_handler(cmd, args) {
    return await window.__TAURI__.core.invoke(cmd, args);
}

export async function js_listen_event(name, handler) {
    return await window.__TAURI__.event.listen(name, handler);
}
