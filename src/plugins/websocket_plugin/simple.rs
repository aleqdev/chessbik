use bevy::prelude::*;

use crate::events::{WebsocketReceiveEvent, WebsocketSendEvent};

pub fn read_system(mut reader: EventReader<WebsocketReceiveEvent>) {
    for i in reader.iter() {
        println!("received: {}", i.0);
    }
}

pub fn write_system(mut writer: EventWriter<WebsocketSendEvent>) {
    writer.send("123 write".to_string().into());
}
