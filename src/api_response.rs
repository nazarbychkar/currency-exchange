/// APIResponse struct accepts the response from the API.
/// It has all the field of that from ExchangeRate-API.
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug, Default)]
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

#[cfg(test)]
mod tests {
    use crate::api_response::APIResponse;

    #[test]
    fn deserialize_api_response_valid_json() {
        let json_data = r#"
    {
        "result": "success",
        "documentation": "https://www.exchangerate-api.com/docs",
        "terms_of_use": "...",
        "time_last_update_unix": 1679173200,
        "time_last_update_utc": "2024-03-18T10:00:00Z",
        "time_next_update_unix": 1679259600,
        "time_next_update_utc": "2024-03-19T10:00:00Z",
        "base_code": "USD",
        "conversion_rates": {
        "EUR": 0.94,
        "JPY": 130.0
        }
    }
    "#;

        let response = serde_json::from_str(json_data);

        assert!(response.is_ok());
        let api_response: APIResponse = response.unwrap();
        assert_eq!(api_response.result, "success");
        assert_eq!(api_response.base_code, "USD");
    }
}
