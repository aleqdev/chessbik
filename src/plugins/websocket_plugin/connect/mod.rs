#[cfg(target_arch = "wasm32")]
mod implementation {
    mod wasm;
    pub use wasm::connect_system;
}

#[cfg(not(target_arch = "wasm32"))]
mod implementation {
    mod regular;
    pub use regular::connect_system;
}

pub use implementation::*;
