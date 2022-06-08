#[cfg(target_arch = "wasm32")]
mod implementation {
    pub mod wasm;
    pub use wasm::{WebsocketReceiver, WebsocketSender};
}

#[cfg(not(target_arch = "wasm32"))]
mod implementation {
    pub mod regular;
    pub use regular::{WebsocketReceiver, WebsocketSender};
}

pub use implementation::*;
