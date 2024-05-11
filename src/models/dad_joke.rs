use super::{
    base::{NoArgs, RequestContext},
    macros::model,
};

model! {
    DadJoke: "/dadjoke";
    joke: String,
}

impl From<NoArgs> for RequestContext<DadJoke> {
    fn from(_val: NoArgs) -> Self {
        RequestContext::new()
    }
}
