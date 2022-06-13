#[cfg(target_arch = "wasm32")]
mod implementation {
    mod wasm;
    pub use wasm::read_clip;
}

#[cfg(not(target_arch = "wasm32"))]
mod implementation {
    mod regular;
    pub use regular::read_clip;
}

pub use implementation::read_clip;
