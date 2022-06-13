use std::{collections::VecDeque, sync::{Arc, RwLock}};

use bevy::prelude::*;
use js_sys::Promise;
use wasm_bindgen::{prelude::{wasm_bindgen, Closure}, JsValue};

use crate::{events::{UiJoinGameEvent, JoinGameFromClipboardEvent}};

pub struct ReadClipQueue(pub Arc<RwLock<VecDeque<String>>>);

#[wasm_bindgen(module = "/js/read_clip.js")]
extern "C" {
    pub fn read_clip() -> Promise;
}


pub fn startup_system(
    mut commands: Commands
) {
    commands.insert_resource(ReadClipQueue(Arc::new(RwLock::new(VecDeque::new()))));
}


pub fn system(
    mut ui_reader: EventReader<UiJoinGameEvent>,
    queue: ResMut<ReadClipQueue>,
    mut writer: EventWriter<JoinGameFromClipboardEvent>
) {

    for _ in ui_reader.iter() {
        let cloned_queue = queue.0.clone();

        let cb = Closure::wrap(Box::new(move |e: JsValue| {
            if let Some(string) = e.as_string() {
                cloned_queue.write()
                    .expect("failed to get write lock of queue")
                    .push_back(string);
            }
        }) as Box<dyn FnMut(JsValue)>);

        wasm_bindgen_futures::spawn_local(async move {
            wasm_bindgen_futures::JsFuture::from(
                read_clip()
                .then(&cb)
            ).await.expect("failure of clipboard future");
        });
    }

    let mut queue = queue.0.write().expect("failed to get write lock of queue");

    while let Some(s) = queue.pop_back() {
        writer.send(JoinGameFromClipboardEvent(s));
    }
}
