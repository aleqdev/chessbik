macro_rules! derive_wrapper {
    ($i:item) => {
        #[derive(
            ::derive_more::Deref, ::derive_more::DerefMut, ::derive_more::From, ::derive_more::Into,
        )]
        $i
    };
}

pub(crate) use derive_wrapper;
