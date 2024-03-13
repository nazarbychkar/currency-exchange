# Currency Exchange
Currency Exchange is a command-line tool written in Rust that facilitates currency conversion using real-time exchange rate data obtained from an API.

## How To Start
To build and run this application, follow these steps:
1. Clone this repository:
```
git clone https://github.com/nazarbychkar/currency-exchange.git
```
2. Build the application:
```
cargo build --release
```
3. Once built, you can find the executable in the target\release directory within the repository directory you cloned earlier. Run the program from there.

## Getting Started
To use the application, follow these steps:
1. Obtain an API Key from [ExchangeRate-API](https://www.exchangerate-api.com/).
2. Launch the application, and it will prompt you to enter your API Key.
3. Once the API Key is provided, you can choose from the available options provided by the program to perform currency conversion.

## Options
- **Exchange Currencies**: To convert currencies, provide the source currency code from which you will be exchanging, the target currency code to which you will be exchanging, and the amount of the source currency.

- **View Available Currencies and Their Current Exchange Rates (to USD)**: This option lists the current exchange rates of available currencies relative to the "international" currency, USD.

 - **Refresh the API Key**: Use this option to refresh your API key if there are any issues with it.

## Notes
- The program accepts currency codes in both lowercase and uppercase formats.
- Simply follow the prompts provided by the program to perform the desired currency conversions.