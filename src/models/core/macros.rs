use serde::{Deserialize, Deserializer};

#[allow(dead_code)]
pub fn empty_string_to_none<'de, D>(deserializer: D) -> Result<Option<String>, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    if s.is_empty() {
        Ok(None)
    } else {
        Ok(Some(s))
    }
}

macro_rules! model {
    ( :$struct_doc:literal $name:ident $( :$endpoint:literal )?; $( $( :$field_doc:literal )? $field_name:ident: $field_type:ty $( = $rename:literal )? $( => $deserialize_fn:literal )? ),* $(,)? ) => {
        #[derive(serde::Deserialize, Debug, PartialEq, PartialOrd, Clone)]
        #[serde(rename_all = "snake_case")]
        #[doc = $struct_doc]
        pub struct $name {
            $(
                $( #[doc = $field_doc] )?
                $( #[serde(rename = $rename)] )?
                $( #[serde(deserialize_with = $deserialize_fn)] )?
                pub $field_name: $field_type,
            )*
        }

        $(
            impl $crate::models::core::Endpoint for $name {
                fn endpoint() -> &'static str {
                    concat!("https://api.fluxpoint.dev", $endpoint)
                }
            }
        )?
    };
}

macro_rules! args_model {
    (
        $name:ident: $target:ident;
        $( $( :$field_doc:literal )?
        $field_name:ident: $field_type:ty $( = $serialized_name:literal )? ),* $(,)?
    ) => {
        #[derive(serde::Serialize, Debug, PartialEq, PartialOrd, Clone)]
        pub struct $name {
            $(
                $( #[doc = $field_doc] )?
                pub $field_name: $field_type,
            )*
        }

        crate::args_model!( @from_tuple_for_type $name: $target; $( $field_name: $field_type ),* );

        crate::args_model!( @impl_from_type $name: $target; $($field_name: $field_type $( = $serialized_name )? ),*);
        crate::args_model!( @impl_from_tuple $name: $target; $($field_name: $field_type $( = $serialized_name )? ),*);


    };

    // JSON -----------------------------------------------
    ( @json $field_name:ident ) => {
        stringify!($field_name)
    };

    ( @json $field_name:ident $serialized_name:literal ) => {
        stringify!($serialized_name)
    };

    // -----------------------------------------
    //  From Type -----------------------------------------
    // EMPTY FIELDS
    (
        @impl_from_type
        $name:ident: $target:ident;
    ) => {};

    // EXISTING FIELDS
    (
        @impl_from_type
        $name:ident: $target:ident;
        $( $field_name:ident: $field_type:ty $( = $serialized_name:literal )? ),*
    ) => {
        impl From<$name> for crate::models::core::RequestContext<$target> {
            fn from(value: $name) -> Self {
                let $name { $($field_name),+ } = value;
                crate::models::core::RequestContext::from((
                    reqwest::Body::from(
                        serde_json::json!(
                        {
                            $( crate::args_model!(@json $field_name $( $serialized_name )?): $field_name ),*
                        }
                        )
                        .to_string(),
                    ),
                    Default::default(),
                    Default::default(),
                ))
            }
        }
    };


    // -----------------------------------------
    // RequestContext from Tuple --------------------------
    // EMPTY FIELDS
    (
        @impl_from_tuple
        $name:ident: $target:ident;
    ) => {};

    // EXISTING FIELDS
    (
        @impl_from_tuple
        $name:ident: $target:ident;
        $( $field_name:ident: $field_type:ty $( = $serialized_name:literal )? ),*
    ) => {
        impl From<($($field_type),*)> for crate::models::core::RequestContext<$target> {
            fn from(value: ($($field_type),*)) -> Self {
                let ($($field_name),+) = value;
                crate::models::core::RequestContext::from((
                    reqwest::Body::from(
                        serde_json::json!(
                        {
                            $( crate::args_model!(@json $field_name $( $serialized_name )?): $field_name ),*
                        }
                        )
                        .to_string(),
                    ),
                    Default::default(),
                    Default::default(),
                ))
            }
        }
    };

    // -----------------------------------------
    //  From Tuple for Type -------------------------------
    // EMPTY FIELDS
    (
        @from_tuple_for_type
        $name:ident: $target:ident;
    ) => {
        impl From<crate::models::core::NoArgs> for $name {
            fn from(_val: crate::models::core::NoArgs) -> Self {
                Self {}
            }
        }
        impl From<crate::models::core::NoArgs> for crate::models::core::RequestContext<$target> {
            fn from(_val: crate::models::core::NoArgs) -> Self {
                crate::models::core::RequestContext::new()
            }
        }
    };

    // EXISTING FIELDS
    (
        @from_tuple_for_type
        $name:ident: $target:ident;
        $( $field_name:ident: $field_type:ty ),*
    ) => {
        impl From<($($field_type),*)> for $name {
            fn from(value: ($($field_type),*)) -> Self {
                let ($($field_name),+) = value;
                Self {
                    $( $field_name ),*
                }
            }
        }
    };

}

macro_rules! impl_noargs {
    ( $target:ident ) => {
        impl From<crate::models::core::NoArgs> for crate::models::core::RequestContext<$target> {
            fn from(_val: crate::models::core::NoArgs) -> Self {
                crate::models::core::RequestContext::new()
            }
        }
    };
}

pub(crate) use args_model;
pub(crate) use impl_noargs;
pub(crate) use model;
