export async function get_count() {
    return await window.__TAURI__.invoke("get_count");
}

export async function plus_count(step) {
    return await window.__TAURI__.invoke("plus_count", { step });
}
