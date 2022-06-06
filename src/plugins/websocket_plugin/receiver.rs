use wasm_rs_shared_channel::spsc::{Sender, Receiver};

pub struct WebsocketReceiver(pub Receiver<String>);

pub struct WebsocketSender(pub Sender<String>);
