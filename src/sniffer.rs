pub struct Sniffer<TConnector> {
    private_client: std::sync::Arc<kuna_sdk::client::KunaClient<TConnector>>,
    public_client: std::sync::Arc<kuna_sdk::public::KunaPublicClient<TConnector>>,
}

impl<TConnector> agnostic::market::Sniffer for Sniffer<TConnector>
where
    TConnector: hyper::client::connect::Connect + Send + Sync + Clone + 'static,
{
    fn all_the_best_orders(
        &self,
        trading_pair: agnostic::trading_pair::TradingPair,
        count: u32,
    ) -> agnostic::market::Future<Result<Vec<agnostic::order::Order>, String>> {
        todo!()
    }

    fn the_best_order(&self, trading_pair: agnostic::trading_pair::TradingPair) -> agnostic::market::Future<Result<agnostic::order::Order, String>> {
        todo!()
    }

    fn get_my_orders(
        &self,
        trading_pair: agnostic::trading_pair::TradingPair,
    ) -> agnostic::market::Future<Result<Vec<agnostic::order::OrderWithId>, String>> {
        todo!()
    }
}
