macro_rules! state {
    ( $( $name:ty $(: $renamed:ident )? );+ $(;)? ) => {

            $(
                crate::state!{ @struct_def $name $(: $renamed )? }
            )+

    };

    ( @struct_def $name:ty ) => {
        paste::paste! {
            #[derive(Default, Clone)]
            pub struct [<__No$name>];
            #[derive(Default, Clone)]
            pub struct [<__$name>]($name);
        }
    };
    ( @struct_def $name:ty : $renamed:ident ) => {
        paste::paste! {
            #[derive(Default, Clone)]
            pub struct [<__No$renamed>];
            #[derive(Default, Clone)]
            pub struct [<__$renamed>]($name);
        }
    };
}

pub(crate) use state;
