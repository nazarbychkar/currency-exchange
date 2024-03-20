use reqwest::StatusCode;

/// Data struct is saving the essential data for the application.
/// Like your API key and cached data from your last request to the API.
use crate::api_response::APIResponse;

#[derive(Debug)]
pub struct Data {
    api_key: String,
    cache: Option<APIResponse>,
}

impl Data {
    /// Build function for creating a new instance of an Data struct.
    pub fn new(api_key: String, cache: Option<APIResponse>) -> Data {
        Data { api_key, cache }
    }

    pub fn set_api_key(&mut self, api_key: String) {
        self.api_key = api_key;
    }

    /// Set up function for the Data struct.
    /// If not cached, it will send a request for a particular currency code to an API
    /// and will return its response.
    pub async fn set_up(&mut self, currency: &str) -> Result<(), reqwest::Error> {
        if let Some(parse) = &self.cache {
            if parse.base_code == currency {
                return Ok(());
            }
        }

        let url = format!(
            "https://v6.exchangerate-api.com/v6/{}/latest/{}",
            self.api_key, currency
        );

        let response = reqwest::get(&url).await?;

        match response.status() {
            StatusCode::OK => {
                let parsed = response.json::<APIResponse>().await?;
                self.cache = Some(parsed);
            }
            StatusCode::UNAUTHORIZED => println!("Need to grab a new token."),
            _ => println!("Uh oh! Something unexpected happened."),
        }

        Ok(())
    }

    /// Function for "exchange" option in the application.
    /// It takes three arguments:
    /// 1. Code of source currency, that it will be fetching data from.
    /// 2. Code of target currency, that it will be exchanging to.
    /// 3. Amount of the source currency.
    /// Outputs String with data collected.
    pub async fn exchange(
        &mut self,
        source_currency: &str,
        target_currency: &str,
        amount: f32,
    ) -> String {
        let source_currency = source_currency.trim().to_uppercase();
        let target_currency = target_currency.trim().to_uppercase();

        if let Err(_) = self.set_up(&source_currency).await {
            return "There is no data on currencies. Refresh the data.".to_string();
        }

        if let Some(parse) = &self.cache {
            if let Some(value) = parse.conversion_rates.get(&target_currency) {
                return format!(
                    "\nResult:\n{} {} will be {} in {}.\nExchange rate used for the conversion: {}.",
                    amount,
                    source_currency,
                    amount * value,
                    target_currency,
                    value
                );
            }
        }

        "There is no such currency.".to_string()
    }

    /// Function for "view rates of currencies" option in the application.
    /// Outputs every currency to USD.
    pub async fn rates(&mut self) {
        if let Err(_) = self.set_up("USD").await {
            println!("There is no data on currencies. Refresh the data.");
            return;
        }

        if let Some(parse) = &self.cache {
            for (name, value) in &parse.conversion_rates {
                println!("{}: {}", name, value);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::{collections::HashMap, env};

    use crate::{api_response::APIResponse, data::Data};

    #[test]
    fn test_data_object_construction() {
        let api_key = env::var("YOUR_API_KEY")
            .expect("$YOUR_API_KEY is not set")
            .to_string();

        let data = Data::new(api_key.clone(), None);

        assert_eq!(data.api_key, api_key);
        assert!(data.cache.is_none());

        let mock_response = APIResponse {
            result: "success".to_string(),
            ..Default::default()
        };
        let data_with_cache = Data::new(api_key.clone(), Some(mock_response));

        assert_eq!(data_with_cache.api_key, api_key);
    }

    #[tokio::test]
    async fn test_data_cache_usage() {
        let api_key = env::var("YOUR_API_KEY")
            .expect("$YOUR_API_KEY is not set")
            .to_string();
        let currency_code = "EUR".to_string();

        let mut data = Data::new(api_key.clone(), None);

        let result = data.set_up(&currency_code.clone()).await;
        assert!(result.is_ok());
        assert!(data.cache.is_some());

        let second_result = data.set_up(&currency_code.clone()).await;
        assert!(second_result.is_ok());
    }

    #[tokio::test]
    async fn test_data_exchange_rate_calculation() {
        let api_key = env::var("YOUR_API_KEY")
            .expect("$YOUR_API_KEY is not set")
            .to_string();
        let mock_response = APIResponse {
            result: "success".to_string(),
            base_code: "USD".to_string(),
            conversion_rates: HashMap::from([
                ("EUR".to_string(), 0.94),
                ("JPY".to_string(), 130.0),
            ]),
            ..Default::default()
        };

        let mut data = Data::new(api_key.clone(), Some(mock_response));

        let source_currency = "USD".to_string();
        let target_currency = "EUR".to_string();
        let amount = 100.0;

        let exchange_result = data
            .exchange(&source_currency, &target_currency, amount)
            .await;

        assert!(exchange_result.contains(&format!(
            "{} {} will be {} in {}",
            amount,
            source_currency,
            amount * 0.94,
            target_currency
        )));
    }
}
