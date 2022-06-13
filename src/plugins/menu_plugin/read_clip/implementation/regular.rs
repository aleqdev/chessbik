use clipboard::{ClipboardContext, ClipboardProvider};

pub fn read_clip() -> String {
    let mut clip: ClipboardContext =
                ClipboardProvider::new().expect("failed to get clipboard");

    clip.get_contents().expect("failed to read clipboard contents")
}