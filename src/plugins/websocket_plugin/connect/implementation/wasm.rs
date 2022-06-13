use bevy::prelude::*;

use crate::plugins::websocket_plugin::resources::{WebsocketReceiver, WebsocketSender};

pub fn connect_system(world: &mut World) {
    use js_sys::Array;
    use wasm_bindgen::{prelude::*, JsCast};
    use wasm_rs_shared_channel::spsc;
    use web_sys::{MessageEvent, WebSocket};

    let (recv_tx, recv_rx) = spsc::channel::<String>(2048).split();
    let (send_tx, send_rx) = spsc::channel::<String>(2048).split();

    world.insert_non_send_resource(WebsocketReceiver(recv_rx));
    world.insert_non_send_resource(WebsocketSender(send_tx));

    let ws = WebSocket::new(crate::consts::WS_URL).expect("failed to connect to webscoket");

    let onmessage_callback = Closure::wrap(Box::new(move |e: MessageEvent| {
        if let Ok(txt) = e.data().dyn_into::<js_sys::JsString>() {
            recv_tx
                .send(&txt.as_string().expect("failed to get JsString"))
                .expect("failed to send recv_tx");
        }
    }) as Box<dyn FnMut(MessageEvent)>);

    let cloned_ws = ws.clone();
    let h = Closure::wrap(Box::new(move || {
        while let Some(str) = send_rx.recv(None).expect("failed to read from send_rx") {
            cloned_ws
                .send_with_str(&str)
                .expect("failed to send websocket message");
        }
    }) as Box<dyn FnMut()>);

    let onopen_callback = Closure::wrap(Box::new(move |_| {
        web_sys::window()
            .expect("failed to get window")
            .set_interval_with_callback_and_timeout_and_arguments(
                h.as_ref().unchecked_ref(),
                200,
                &Array::new(),
            )
            .expect("failed to set websocket interval");
    }) as Box<dyn FnMut(JsValue)>);

    ws.set_onmessage(Some(onmessage_callback.as_ref().unchecked_ref()));
    ws.set_onopen(Some(onopen_callback.as_ref().unchecked_ref()));

    onmessage_callback.forget();
    onopen_callback.forget();
}
