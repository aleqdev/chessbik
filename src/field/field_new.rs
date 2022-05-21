pub trait FieldNew {
    fn new() -> super::Field;
}

impl FieldNew for super::Field {
    fn new() -> Self {
        Self {
            cells: [
                crate::cell::BPAWN,
                crate::cell::BPAWN,
                crate::cell::BPAWN,
                crate::cell::NONE,
                crate::cell::NONE,
                crate::cell::NONE,
                crate::cell::WPAWN,
                crate::cell::WPAWN,
                crate::cell::WPAWN,
                crate::cell::BPAWN,
                crate::cell::NONE,
                crate::cell::WPAWN,
                crate::cell::BPAWN,
                crate::cell::NONE,
                crate::cell::WPAWN,
                crate::cell::BPAWN,
                crate::cell::NONE,
                crate::cell::WPAWN,
                crate::cell::WKNIGHT,
                crate::cell::WQUEEN,
                crate::cell::WKNIGHT,
                crate::cell::WROOK,
                crate::cell::WKING,
                crate::cell::WROOK,
                crate::cell::WBISHOP,
                crate::cell::WMAGE,
                crate::cell::WBISHOP,
                crate::cell::WPAWN,
                crate::cell::NONE,
                crate::cell::BPAWN,
                crate::cell::WPAWN,
                crate::cell::NONE,
                crate::cell::BPAWN,
                crate::cell::WPAWN,
                crate::cell::NONE,
                crate::cell::BPAWN,
                crate::cell::BKNIGHT,
                crate::cell::BQUEEN,
                crate::cell::BKNIGHT,
                crate::cell::BROOK,
                crate::cell::BKING,
                crate::cell::BROOK,
                crate::cell::BBISHOP,
                crate::cell::BMAGE,
                crate::cell::BBISHOP,
                crate::cell::WPAWN,
                crate::cell::WPAWN,
                crate::cell::WPAWN,
                crate::cell::NONE,
                crate::cell::NONE,
                crate::cell::NONE,
                crate::cell::BPAWN,
                crate::cell::BPAWN,
                crate::cell::BPAWN,
            ],
        }
    }
}