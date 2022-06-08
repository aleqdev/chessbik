use crossbeam_channel::{Receiver, Sender};

chessbik_derive_wrapper::derive_wrapper!(
    pub struct WebsocketReceiver(pub Receiver<String>);
);

chessbik_derive_wrapper::derive_wrapper!(
    pub struct WebsocketSender(pub Sender<String>);
);
