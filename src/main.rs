//! Currency Exchange is a command-line tool written in Rust that 
//! facilitates currency conversion using real-time exchange rate 
//! data obtained from an API.
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, io::stdin};

/// Basic input. Creates String and put there read lines.
fn input() -> String {
    let mut input = String::new();
    stdin().read_line(&mut input).expect("Incorrect value");
    input
}

/// Data struct is saving the essential data for the application.
/// Like your API key and cached data from your last request to the API.
#[derive(Debug)]
struct Data {
    api_key: String,
    cache: Option<APIResponse>,
}

impl Data {
    /// Set up function for the Data struct.
    /// If not cached, it will send a request for a particular currency code to an API 
    /// and will return its response.
    #[tokio::main]
    async fn set_up(&mut self, currency: &str) {
        if let Some(parse) = &self.cache {
            if parse.base_code == currency {
                return;
            }
        }

        let url = format!(
            "https://v6.exchangerate-api.com/v6/{}/latest/{}",
            self.api_key, currency
        );

        let client = reqwest::Client::new();
        let response = client.get(url).send().await.unwrap();

        match response.status() {
            reqwest::StatusCode::OK => match response.json::<APIResponse>().await {
                Ok(parsed) => {
                    self.cache = Some(parsed);
                }
                Err(_) => println!("Hm, the response didn't match the shape we expected"),
            },
            reqwest::StatusCode::UNAUTHORIZED => println!("Need to grab a new token."),
            _ => println!("Uh oh! Something unexpected happened."),
        }
    }

    /// Function for "exchange" option in the application.
    /// It takes three arguments: 
    /// 1. Code of source currency, that it will be fetching data from.
    /// 2. Code of target currency, that it will be exchanging to.
    /// 3. Amount of the source currency.
    /// Outputs String with data collected.
    fn exchange(
        &mut self,
        source_currency: String,
        target_currency: String,
        amount: f32,
    ) -> String {
        let binding = source_currency.trim().to_uppercase();
        let source_currency = binding.as_str();
        let binding = target_currency.trim().to_uppercase();
        let target_currency = binding.as_str();
        self.set_up(source_currency);

        match &self.cache {
            Some(parse) => match parse.conversion_rates.get(target_currency) {
                Some(value) => {
                    let result = format!("\nResult:\n{} {} will be {} in {}.\nExchange rate used for the conversion: {}.", amount, source_currency, amount as f32*value, target_currency, value);
                    result
                }
                None => "There is no such currency.".to_string(),
            },
            None => "There is no data on currencies. Refresh the data.".to_string(),
        }
    }
    /// Function for "view rates of currencies" option in the application.
    /// Outputs every currency to USD.
    fn rates(&mut self) {
        self.set_up("USD");

        match &self.cache {
            Some(parse) => {
                for (name, value) in &parse.conversion_rates {
                    println!("{name}: {value}");
                }
            }
            None => println!("There is no data on currencies. Refresh the data."),
        }
    }
}

/// APIResponse struct accepts the response from the API.
/// It has all the field of that from ExchangeRate-API.
#[derive(Serialize, Deserialize, Debug)]
struct APIResponse {
    result: String,
    documentation: String,
    terms_of_use: String,
    time_last_update_unix: u32,
    time_last_update_utc: String,
    time_next_update_unix: u32,
    time_next_update_utc: String,
    base_code: String,
    conversion_rates: HashMap<String, f32>,
}

/// Main function.
/// Firstly, programm asks for your API key.
/// Then, it shows you options for you to choose from.
/// It all simple.
fn main() {
    println!(
        "\nHello, this is currency convertion CLI programm!
Enter your API Key:"
    );
    let api_key_input = input().trim().to_string();
    let mut data = Data {
        api_key: api_key_input,
        cache: None,
    };

    loop {
        println!(
            "\nPut:\n1 - to exchange currencies.
2 - to view available currencies and their current exchange rates(to USD).
3 - to refresh the API Key.
0 - to exit."
        ); //TODO 4- to refresh current currency data.
        let start_input: u16 = input().trim().parse().expect("Not an int.");
        match start_input {
            1 => {
                println!("\nYou can put currencies in their internation codes, like \"USD\" or just \"usd\" or even \"UsD\" :)\n\nPut the \"source currency\", that is from which you\'ll be fetching data:");
                let source_input = input().trim().to_string();
                println!("Put the \"target currency\", that is to what you'll be converting:");
                let target_input = input().trim().to_string();
                println!("Put an amount of the source currency:");
                let amount_input: f32 = input().trim().parse().expect("Not an number.");

                let result = data.exchange(source_input, target_input, amount_input);
                println!("{result}");
            }
            2 => {
                data.rates();
            }
            3 => {
                println!("Enter your API Key:");
                let api_key_input = input().trim().to_string();
                data.api_key = api_key_input;
            },
            0 => std::process::exit(0),
            _ => println!("There is no such option."),
        }
    }
}
