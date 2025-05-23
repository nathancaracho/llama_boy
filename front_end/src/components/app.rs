use futures_util::SinkExt;
use gloo_net::websocket::{futures::WebSocket, Message};
use wasm_bindgen_futures::spawn_local;
use web_sys::{
    wasm_bindgen::{self, prelude::wasm_bindgen},
    KeyboardEvent,
};
use yew::{platform::time::sleep, prelude::*};

use crate::components::memory::Memory;

use super::gba_screen::GbaScreen;

#[wasm_bindgen(inline_js = "export function init_materialize() { M.AutoInit(); }")]
extern "C" {
    pub fn init_materialize();
}
#[function_component(App)]
pub fn app() -> Html {
    let ws = use_mut_ref(|| {
        WebSocket::open("ws://localhost:8080/command/ws").expect("Failed to connect")
    });

    let on_keydown = {
        let ws = ws.clone();
        Callback::from(move |e: KeyboardEvent| {
            let command = match e.key().as_str() {
                "ArrowUp" => "up",
                "ArrowDown" => "down",
                "ArrowLeft" => "left",
                "ArrowRight" => "right",
                "Enter" => "start",
                " " => "select",
                "a" => "a",
                "s" => "b",
                "z" => "l",
                "x" => "r",
                _ => return,
            }
            .to_string();

            let ws = ws.clone();
            spawn_local(async move {
                let mut socket = ws.borrow_mut();
                let _ = socket.send(Message::Text(command)).await;
            });
        })
    };

    html! {
        <div
        class="container"
            tabindex="0"
            onkeydown={on_keydown}
        >
            <div class="row">
                <div class="col s12"><GbaScreen /></div>
            </div>
            <div class="row">
                <div class="col s12"><Memory /></div>
            </div>
        </div>
    }
}
