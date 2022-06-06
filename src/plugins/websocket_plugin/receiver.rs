use wasm_rs_shared_channel::spsc::{Receiver, Sender};

pub struct WebsocketReceiver(pub Receiver<String>);

pub struct WebsocketSender(pub Sender<String>);
