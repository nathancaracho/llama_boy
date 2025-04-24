use futures_util::StreamExt;
use gloo_net::websocket::futures::WebSocket;
use gloo_net::websocket::Message;
use serde_json::Value;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

use crate::components::deck_view::DeckView;
use crate::components::json_tree::JsonTree;
use crate::components::pokemon::{Pokemon, PokemonCard};

#[function_component(Memory)]
pub fn memory() -> Html {
    let data = use_state(|| Value::Null);
    let initialized = use_mut_ref(|| false);
    let expanded = use_state(|| false);

    let toggle = {
        let expanded = expanded.clone();
        Callback::from(move |_e: MouseEvent| expanded.set(!*expanded))
    };
    {
        let data = data.clone();
        let initialized = initialized.clone();
        use_effect(move || {
            if !*initialized.borrow() {
                *initialized.borrow_mut() = true;

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
        <div>
            // <h2>{"Memory Fields"}</h2>
            // if let Value::Null = *data {
            //     <p>{"Have no data..."}</p>
            // } else {
            //     <JsonTree value={(*data).clone()} />
            // }

            <div class="row">
                <div class="col s6">
                    <h5>{"Enemy Party"}</h5>
                    {
                        if let Some(enemies) = data.get("enemy_party").and_then(|v| v.as_array()) {
                            let enimies_cards: Vec<Pokemon> = enemies.iter()
                                .take(5)
                                .filter_map(|poke_json| serde_json::from_value::<Pokemon>(poke_json.clone()).ok())
                                .collect();

                            html! { <DeckView pokemons={enimies_cards} />}
                        } else {
                            html! { <p>{ "No enemy party data." }</p> }
                        }
                    }

                </div>
                <div class="col s6">
                <h5>{"Player Party"}</h5>
                <div >
                    {
                        if let Some(players) = data.get("player_party").and_then(|v| v.as_array()) {
                            let player_cards: Vec<Pokemon> = players.iter()
                                .take(5)
                                .filter_map(|poke_json| serde_json::from_value::<Pokemon>(poke_json.clone()).ok())
                                .collect();

                            html! { <DeckView pokemons={player_cards} />}
                        } else {
                            html! { <p>{ "No player party data." }</p> }
                        }
                    }
                </div>
                </div>
            </div>



        </div>
    }
}
