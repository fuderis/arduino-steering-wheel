export async function invoke_tauri(cmd, args) {
    return await window.__TAURI__.invoke(cmd, args);
}
