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
    ( $name:ident $( :$endpoint:literal )?; $( $( :$field_doc:literal )? $field_name:ident: $field_type:ty $( = $rename:literal )? $( => $deserialize_fn:literal )? ),* $(,)? ) => {
        #[derive(serde::Deserialize, Debug, PartialEq, PartialOrd, Clone)]
        #[serde(rename_all = "snake_case")]
        pub struct $name {
            $(
                $( #[doc = $field_doc] )?
                $( #[serde(rename = $rename)] )?
                $( #[serde(deserialize_with = $deserialize_fn)] )?
                pub $field_name: $field_type,
            )*
        }

        $(
            impl $crate::models::base::Endpoint for $name {
                fn endpoint() -> &'static str {
                    concat!("https://api.fluxpoint.dev", $endpoint)
                }
            }
        )?
    };
}

macro_rules! args_model {
    ( $name:ident: $target:ident; $( $( :$field_doc:literal )? $field_name:ident: $field_type:ty $( = $serialized_name:literal )? ),* $(,)? ) => {
        #[derive(serde::Serialize, Debug, PartialEq, PartialOrd, Clone)]
        pub struct $name {
            $(
                $( #[doc = $field_doc] )?
                pub $field_name: $field_type,
            )*
        }

        impl From<($($field_type),*)> for $name {
            fn from(value: ($($field_type),*)) -> Self {
                let ($($field_name),+) = value;
                Self {
                    $( $field_name ),*
                }
            }
        }

        impl From<$name> for RequestContext<$target> {
            fn from(value: $name) -> Self {
                let $name { $($field_name),+ } = value;
                RequestContext::from((
                    Body::from(
                        json!(
                        {
                            $( crate::args_model!(@json $field_name $( $serialized_name )?): $field_name ),*
                            // $( stringify!($field_name): $field_name ),*Y

                        }
                        )
                        .to_string(),
                    ),
                    Default::default(),
                ))
            }
        }

        impl From<($($field_type),*)> for RequestContext<$target> {
            fn from(value: ($($field_type),*)) -> Self {
                let ($($field_name),+) = value;
                RequestContext::from((
                    Body::from(
                        json!(
                        {
                            $( crate::args_model!(@json $field_name $( $serialized_name )?): $field_name ),*
                            // $( stringify!($field_name): $field_name ),*Y

                        }
                        )
                        .to_string(),
                    ),
                    Default::default(),
                ))
            }
        }
    };

    ( @json $field_name:ident ) => {
        stringify!($field_name)
    };

    ( @json $field_name:ident $serialized_name:literal ) => {
        stringify!($serialized_name)
    };

}

pub(crate) use args_model;
pub(crate) use model;
