use bevy::{prelude::*, tasks::AsyncComputeTaskPool};

use crate::plugins::websocket_plugin::resources::{WebsocketReceiver, WebsocketSender};

pub fn connect_system(mut commands: Commands, pool: Res<AsyncComputeTaskPool>) {
    let (recv_tx, recv_rx) = crossbeam_channel::unbounded::<String>();
    let (send_tx, send_rx) = crossbeam_channel::unbounded::<String>();

    commands.insert_resource(WebsocketReceiver(recv_rx));

    commands.insert_resource(WebsocketSender(send_tx));

    pool.spawn(async {
        use async_tungstenite::{async_std::connect_async, tungstenite::Message};
        use futures::StreamExt;
        use futures_util::SinkExt;
        use std::time::Duration;

        let (stream, _) = connect_async("wss://chessbik.herokuapp.com/ws")
            .await
            .expect("failed to connect to webscoket");

        let (mut write, mut read) = stream.split();

        let read_future = async move {
            while let Some(message) = read.next().await {
                let message = message.expect("failed to receive message from websocket");
                match message {
                    Message::Text(message) => {
                        recv_tx.send(message).expect("failed to send recv_tx");
                    }
                    _ => {}
                }
            }
        };

        let write_future = async move {
            loop {
                for i in send_rx.try_iter() {
                    write
                        .send(Message::Text(i))
                        .await
                        .expect("failed to send websocket message");
                }

                async_std::task::sleep(Duration::from_secs_f32(0.06)).await;
            }
        };

        futures::join!(read_future, write_future);
    })
    .detach();
}
