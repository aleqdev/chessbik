pub use super::*;

#[cfg(target_arch = "wasm32")]
mod implementation {
    mod wasm;
    pub use wasm::{receive_system, send_system};
}

#[cfg(not(target_arch = "wasm32"))]
mod implementation {
    mod regular;
    pub use regular::{receive_system, send_system};
}

pub use implementation::*;
