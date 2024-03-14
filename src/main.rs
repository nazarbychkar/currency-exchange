//! Currency Exchange is a command-line tool written in Rust that
//! facilitates currency conversion using real-time exchange rate
//! data obtained from an API.

mod api_response;
mod data;
mod service;
use data::Data;
use service::input;

/// Main function.
/// Firstly, programm asks for your API key.
/// Then, it shows you options for you to choose from.
/// It all simple.
#[tokio::main]
async fn main() {
    println!(
        "\nHello, this is currency convertion CLI programm!
Enter your API Key:"
    );
    let api_key_input = input().trim().to_string();
    let mut data = Data::new(api_key_input, None);
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
                
                println!("{}", source_input);

                let result = data.exchange(&source_input, &target_input, amount_input).await;
                println!("{result}");
            }
            2 => {
                data.rates().await;
            }
            3 => {
                println!("Enter your API Key:");
                let api_key_input = input().trim().to_string();
                data.set_api_key(api_key_input);
            }
            0 => std::process::exit(0),
            _ => println!("There is no such option. Choose from the provided options."),
        }
    }
}
