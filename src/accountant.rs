use agnostic::trading_pair::TradingPairConverter;
use crate::convert;

pub struct Accountant<TConnector> {
    private_client: std::sync::Arc<kuna_sdk::client::KunaClient<TConnector>>,
    price_epsilon: f64,
}

impl<TConnector> Accountant<TConnector> 
where
    TConnector: hyper::client::connect::Connect + Send + Sync + Clone + 'static,
{
    pub fn new(
        private_client: std::sync::Arc<kuna_sdk::client::KunaClient<TConnector>>,
    ) -> Accountant<TConnector> {
        Accountant {
            private_client,
            price_epsilon: 0.0001,
        }
    }
}

impl<TConnector> agnostic::market::Accountant for Accountant<TConnector>
where
    TConnector: hyper::client::connect::Connect + Send + Sync + Clone + 'static,
{
    fn ask(
        &self,
        coin: agnostic::trading_pair::Coin,
    ) -> agnostic::market::Future<Result<agnostic::currency::Currency, String>> {
        let client = self.private_client.clone();
        let future = async move {
            let balance = match client.get_balance().await {
                Ok(balance) => balance,
                Err(error) => return Err(error),
            };
            let converter = convert::CoinConverter::default();
            let kuna_coin = converter.from_agnostic_coin(coin.clone());
            match balance.into_iter()
                .find(|currency| currency.coin == kuna_coin) {
                    Some(currency) => Ok(agnostic::currency::Currency {
                        coin,
                        amount: currency.available,
                        held: currency.full - currency.available,
                    }),
                    None => Err("Failed to find currency in balance".to_owned()),
            }
        };
        Box::pin(future)
    }

    fn ask_both(
        &self,
        first_coin: agnostic::trading_pair::Coin,
        second_coin: agnostic::trading_pair::Coin,
    ) -> agnostic::market::Future<
        Result<(agnostic::currency::Currency, agnostic::currency::Currency), String>,
    > {
        let client = self.private_client.clone();
        let future = async move {
            let balance = match client.get_balance().await {
                Ok(balance) => balance,
                Err(error) => return Err(error),
            };
            let converter = convert::CoinConverter::default();
            let kuna_first_coin = converter.from_agnostic_coin(first_coin.clone());
            let first_currency = match balance.iter()
                .find(|currency| currency.coin == kuna_first_coin) {
                    Some(currency) => agnostic::currency::Currency {
                        coin: first_coin,
                        amount: currency.available,
                        held: currency.full - currency.available,
                    },
                    None => return Err("Failed to find currency in balance".to_owned()),
            };
            let kuna_second_coin = converter.from_agnostic_coin(second_coin.clone());
            let second_currency = match balance.iter()
                .find(|currency| currency.coin == kuna_second_coin) {
                    Some(currency) => agnostic::currency::Currency {
                        coin: second_coin,
                        amount: currency.available,
                        held: currency.full - currency.available,
                    },
                    None => return Err("Failed to find currency in balance".to_owned()),
            };
            Ok((first_currency, second_currency))
        };
        Box::pin(future)
    }

    fn calculate_volume(
        &self,
        _trading_pair: agnostic::trading_pair::TradingPair,
        price: f64,
        amount: f64,
    ) -> f64 {
        price * amount
    }

    fn nearest_price(
        &self,
        trading_pair: agnostic::trading_pair::TradingPair,
        price: f64
    ) -> f64 {
        match trading_pair.side {
            agnostic::trading_pair::Side::Buy => price + self.price_epsilon,
            agnostic::trading_pair::Side::Sell => price - self.price_epsilon,
        }
    }
}
