use bevy::prelude::*;

use super::receiver::{WebsocketReceiver, WebsocketSender};

pub fn system(
    mut commands: Commands
) {
    use web_sys::{MessageEvent, WebSocket};
    use js_sys::Array;
    use wasm_bindgen::{JsCast, prelude::*};

    let (recv_tx, recv_rx) = crossbeam_channel::unbounded::<String>();
    let (send_tx, send_rx) = crossbeam_channel::unbounded::<String>();

    commands
        .insert_resource(WebsocketReceiver(recv_rx));

    commands
        .insert_resource(WebsocketSender(send_tx));

    let ws = WebSocket::new("ws://0.0.0.0:3000/ws")
        .expect("failed to connect to webscoket");

    let onmessage_callback = Closure::wrap(Box::new(move |e: MessageEvent| {
        if let Ok(txt) = e.data().dyn_into::<js_sys::JsString>() {
            recv_tx.send(txt.as_string().expect("failed to get JsString"))
                .expect("failed to send recv_tx");
        }
    }) as Box<dyn FnMut(MessageEvent)>);

    let cloned_ws = ws.clone();
    let h = move || {
        for e in send_rx.iter() {
            cloned_ws.send_with_str(&e)
                .expect("failed to send websocket message");
        }
    };
    let h = Closure::wrap(Box::new(h) as Box<dyn FnMut()>);

    let onopen_callback = Closure::wrap(Box::new(move |_| {
        web_sys::window()
        .expect("failed to get window")
        .set_interval_with_callback_and_timeout_and_arguments( 
            h.as_ref().unchecked_ref(),
            60,
            &Array::new()
        ).expect("failed to set websocket interval");
    }) as Box<dyn FnMut(JsValue)>);

    ws.set_onmessage(Some(onmessage_callback.as_ref().unchecked_ref()));
    ws.set_onopen(Some(onopen_callback.as_ref().unchecked_ref()));

    onmessage_callback.forget();
    onopen_callback.forget();
}