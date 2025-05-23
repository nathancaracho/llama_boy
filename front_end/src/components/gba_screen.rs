use base64::decode;
use futures_util::StreamExt;
use gloo_net::websocket::{futures::WebSocket, Message};
use web_sys::wasm_bindgen::JsCast;
use web_sys::wasm_bindgen::{self, Clamped};
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, ImageData};
use yew::platform::spawn_local;
use yew::prelude::*;

#[function_component(GbaScreen)]
pub fn gba_screen() -> Html {
    let canvas_ref = use_node_ref();

    {
        let canvas_ref = canvas_ref.clone();

        use_effect(move || {
            spawn_local({
                let canvas_ref = canvas_ref.clone();
                async move {
                    let ws = WebSocket::open("ws://localhost:8080/video/ws")
                        .expect("failed to connect to WebSocket");
                    let mut read = ws.split().1;

                    while let Some(Ok(Message::Text(base64_data))) = read.next().await {
                        if let Ok(decoded) = decode(&base64_data) {
                            if let Some(canvas) = canvas_ref.cast::<HtmlCanvasElement>() {
                                if let Ok(Some(context)) = canvas.get_context("2d").map(|ctx| {
                                    ctx.map(|c| c.dyn_into::<CanvasRenderingContext2d>().unwrap())
                                }) {
                                    let width = 240;
                                    let height = 160;
                                    let clamped = Clamped(decoded.as_slice());
                                    let image_data = ImageData::new_with_u8_clamped_array_and_sh(
                                        clamped, width, height,
                                    )
                                    .expect("failed to create ImageData");

                                    context
                                        .put_image_data(&image_data, 0.0, 0.0)
                                        .expect("failed to draw frame");
                                }
                            }
                        }
                    }
                }
            });

            || ()
        });
    }

    html! {
        <div>
            <h2>{ "GBA Screen" }</h2>
            <canvas class="z-depth-3 " ref={canvas_ref} width="240" height="160" style="width: 480px; filter: contrast(1.2) brightness(0.9) saturate(1.3); image-rendering: pixelated;" />
        </div>
    }
}
