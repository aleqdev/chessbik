use bevy::prelude::Component;
use crossbeam_channel::{Receiver, Sender};

#[derive(Component)]
pub struct WebsocketReceiver(pub Receiver<String>);

#[derive(Component)]
pub struct WebsocketSender(pub Sender<String>);
