pub struct Accountant<TConnector> {
    private_client: std::sync::Arc<kuna_sdk::client::KunaClient<TConnector>>,
    public_client: std::sync::Arc<kuna_sdk::public::KunaPublicClient<TConnector>>,
}

impl<TConnector> agnostic::market::Accountant for Accountant<TConnector>
where
    TConnector: hyper::client::connect::Connect + Send + Sync + Clone + 'static,
{
    fn ask(
        &self,
        coin: agnostic::trading_pair::Coin,
    ) -> agnostic::market::Future<Result<agnostic::currency::Currency, String>> {
        todo!()
    }

    fn ask_both(
        &self,
        first_coin: agnostic::trading_pair::Coin,
        second_coin: agnostic::trading_pair::Coin,
    ) -> agnostic::market::Future<
        Result<(agnostic::currency::Currency, agnostic::currency::Currency), String>,
    > {
        todo!()
    }

    fn calculate_volume(
        &self,
        trading_pair: agnostic::trading_pair::TradingPair,
        price: f64,
        amount: f64,
    ) -> f64 {
        todo!()
    }

    fn nearest_price(&self, trading_pair: agnostic::trading_pair::TradingPair, price: f64) -> f64 {
        todo!()
    }
}
