/// APIResponse struct accepts the response from the API.
/// It has all the field of that from ExchangeRate-API.
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
pub struct APIResponse {
    pub result: String,
    pub documentation: String,
    pub terms_of_use: String,
    pub time_last_update_unix: u32,
    pub time_last_update_utc: String,
    pub time_next_update_unix: u32,
    pub time_next_update_utc: String,
    pub base_code: String,
    pub conversion_rates: HashMap<String, f32>,
}
