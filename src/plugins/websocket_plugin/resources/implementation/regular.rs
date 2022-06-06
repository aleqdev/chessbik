use crossbeam_channel::{Receiver, Sender};

chessbik_commons::derive_wrapper!(
    pub struct WebsocketReceiver(pub Receiver<String>);
);

chessbik_commons::derive_wrapper!(
    pub struct WebsocketSender(pub Sender<String>);
);
