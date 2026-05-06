use leptos::prelude::*;
use wasm_bindgen::prelude::*;

#[component]
fn App() -> impl IntoView {
    let (count, set_count) = signal(0);

    view! {
        <div>
            <h2>"My Extension"</h2>
            <p>"Count: " {count}</p>
            <button on:click=move |_| set_count.update(|n| *n += 1)>
                "Click me"
            </button>
        </div>
    }
}

#[wasm_bindgen(start)]
pub fn run() {
    leptos::mount::mount_to_body(App);
}