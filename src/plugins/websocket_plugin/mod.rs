pub mod connect;
pub mod convertion;
pub mod resources;
mod simple;

use bevy::{core::FixedTimestep, prelude::*};

use crate::events::{WebsocketReceiveEvent, WebsocketSendEvent};

pub struct WebsocketPlugin;

impl Plugin for WebsocketPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(connect::connect_system.exclusive_system());
        app.add_system(convertion::send_system);
        app.add_system(convertion::receive_system);
        app.add_event::<WebsocketSendEvent>();
        app.add_event::<WebsocketReceiveEvent>();
        app.add_startup_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::steps_per_second(1.))
                .with_system(simple::read_system)
                .with_system(simple::write_system),
        );
    }
}
