use bevy::prelude::*;

use super::receiver::{WebsocketReceiver, WebsocketSender};

pub fn system(
    world: &mut World
) {
    use web_sys::{MessageEvent, WebSocket};
    use js_sys::Array;
    use wasm_bindgen::{JsCast, prelude::*};
    use wasm_rs_shared_channel::spsc;

    let (recv_tx, recv_rx) = spsc::channel::<String>(128).split();
    let (send_tx, send_rx) = spsc::channel::<String>(128).split();

    world.insert_non_send_resource(WebsocketReceiver(recv_rx));
    world.insert_non_send_resource(WebsocketSender(send_tx));

    let ws = WebSocket::new("ws://0.0.0.0:3000/ws")
        .expect("failed to connect to webscoket");

    let onmessage_callback = Closure::wrap(Box::new(move |e: MessageEvent| {
        if let Ok(txt) = e.data().dyn_into::<js_sys::JsString>() {
            recv_tx.send(&txt.as_string().expect("failed to get JsString"))
                .expect("failed to send recv_tx");
        }
    }) as Box<dyn FnMut(MessageEvent)>);

    let cloned_ws = ws.clone();
    let h = move || {
        loop {
            while let Some(str) = send_rx.recv(None).expect("failed to read from send_rx") {
                cloned_ws.send_with_str(&str)
                    .expect("failed to send websocket message");
            }
        }
    };
    let h = Closure::wrap(Box::new(h) as Box<dyn FnMut()>);

    let onopen_callback = Closure::wrap(Box::new(move |_| {
        web_sys::window()
        .expect("failed to get window")
        .set_interval_with_callback_and_timeout_and_arguments( 
            h.as_ref().unchecked_ref(),
            600,
            &Array::new()
        ).expect("failed to set websocket interval");
    }) as Box<dyn FnMut(JsValue)>);

    ws.set_onmessage(Some(onmessage_callback.as_ref().unchecked_ref()));
    ws.set_onopen(Some(onopen_callback.as_ref().unchecked_ref()));

    onmessage_callback.forget();
    onopen_callback.forget();
}