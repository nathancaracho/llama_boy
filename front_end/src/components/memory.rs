use futures_util::StreamExt;
use gloo_net::websocket::futures::WebSocket;
use gloo_net::websocket::Message;
use serde_json::Value;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

use crate::components::json_tree::JsonTree;

#[function_component(Memory)]
pub fn memory() -> Html {
    let data = use_state(|| Value::Null);
    // flag para rodar só uma vez
    let initialized = use_mut_ref(|| false);

    {
        let data = data.clone();
        let initialized = initialized.clone();
        use_effect(move || {
            // só entra aqui na primeira renderização
            if !*initialized.borrow() {
                *initialized.borrow_mut() = true;

                // abre a WS uma única vez
                let ws = WebSocket::open("ws://localhost:8080/memory/ws")
                    .expect("failed to connect memory websocket");

                spawn_local(async move {
                    let (_write, mut read) = ws.split();
                    while let Some(Ok(Message::Text(txt))) = read.next().await {
                        if let Ok(json) = serde_json::from_str::<Value>(&txt) {
                            data.set(json);
                        }
                    }
                });
            }

            || ()
        });
    }

    html! {
        <div style="display: flex; flex-direction: column;max-height: 100vh;overflow-y: auto;">

                <h2>{"Memory Fields"}</h2>
                if let Value::Null = *data {
                    <p>{"Have no data..."}</p>
                } else {
                    <JsonTree value={(*data).clone()} />
                }
        </div>
    }
}
