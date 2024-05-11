use reqwest::Body;
use serde_json::json;

use super::{
    base::RequestContext,
    cs_datetime_format::CSDateTimeFormat,
    macros::{args_model, model},
};

model! {
    SnowflakeDate: "/utility/snowflake-date";
    date_string: String = "content",
}

args_model! {
    SnowflakeDateArgs: SnowflakeDate;
    snowflake: u64,
    format: CSDateTimeFormat,
}
