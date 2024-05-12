macro_rules! state {
    ( $( $name:ty );+ $(;)? ) => {
        paste::paste! {
            $(
                #[derive(Default, Clone)]
                pub struct [<__No$name>];
                #[derive(Default, Clone)]
                pub struct [<__$name>]($name);
            )+
        }
    };
}

pub(super) use state;
