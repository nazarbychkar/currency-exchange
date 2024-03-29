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

## Testing

### Casual testing
1. **Set Up API Key**: Before running tests, create an environment variable named YOUR_API_KEY and assign your API key from [ExchangeRate-API](https://www.exchangerate-api.com/). Instructions for creating environment variables vary by operating system [(Windows, Linux, or Mac)](https://www3.ntu.edu.sg/home/ehchua/programming/howto/Environment_Variables.html).
2. **Run Unit Tests**: Use the ```cargo test``` command to execute your project's unit tests.

### Testing with docker
1. **Choose Your Approach**:

- Build Your Own Image:
Install [Docker](https://docs.docker.com/engine/install/) on your system.
Run the following command to build a Docker image from your current directory, replacing ```<image-name>``` with a descriptive name for your image.
```
docker build -t <image-name> .
```

- Use a Pre-Built Image:
Alternatively, you can use the pre-built image chypa17/currency_exchange from [Docker Hub](https://hub.docker.com/r/chypa17/currency_exchange).

2. **Run the Image**:

- Interactive Testing: Use the following command to start the image in an interactive container. This allows you to run commands within the container, including ```cargo test```.
```
docker run -it <image-name> /bin/bash
```
- Non-Interactive Testing: Use the following command to run the image as main procces, just the programm, nothing else.

```
docker run -i <image-name>
```

## Notes
- The program accepts currency codes in both lowercase and uppercase formats.
- Simply follow the prompts provided by the program to perform the desired currency conversions.