export async function invoke_tauri(cmd, args) {
    return await window.__TAURI__.core.invoke(cmd, args);
}
