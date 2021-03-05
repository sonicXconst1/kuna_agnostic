pub struct Trader<TConnector> {
    private_client: std::sync::Arc<kuna_sdk::client::KunaClient<TConnector>>,
    public_client: std::sync::Arc<kuna_sdk::public::KunaPublicClient<TConnector>>,
}

impl<TConnector> agnostic::market::Trader for Trader<TConnector>
where
    TConnector: hyper::client::connect::Connect + Send + Sync + Clone + 'static,
{
    fn create_order(&self, order: agnostic::order::Order) -> agnostic::market::Future<Result<(), String>> {
        todo!()
    }

    fn delete_and_create(
        &self,
        id: &str,
        new_order: agnostic::order::Order,
    ) -> agnostic::market::Future<Result<String, String>> {
        todo!()
    }

    fn delete_order(&self, id: &str) -> agnostic::market::Future<Result<(), String>> {
        todo!()
    }

    fn create_trade_from_order(&self, order: agnostic::order::Order) -> agnostic::market::Future<Result<(), String>> {
        todo!()
    }
}
