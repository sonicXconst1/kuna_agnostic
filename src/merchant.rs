use crate::accountant;
use crate::sniffer;
use crate::trader;
use std::sync::Arc;

pub struct Merchant<TConnector> {
    accountant: Arc<accountant::Accountant<TConnector>>,
    sniffer: Arc<sniffer::Sniffer<TConnector>>,
    trader: Arc<trader::Trader<TConnector>>,
}

impl<TConnector> Merchant<TConnector>
where
    TConnector: hyper::client::connect::Connect + Send + Sync + Clone + 'static
{
    pub fn new(
        client: std::sync::Arc<hyper::Client<TConnector>>,
        private_key: String,
        public_key: String,
        base_url: url::Url,
    ) ->  Merchant<TConnector> {
        let auth_context = std::sync::Arc::new(kuna_sdk::context::AuthContext::new(
            private_key,
            public_key,
            base_url));
        let private_client = std::sync::Arc::new(kuna_sdk::client::KunaClient::new(
            client.clone(),
            auth_context.clone()));
        let public_client = std::sync::Arc::new(kuna_sdk::public::KunaPublicClient::new(
            client,
            auth_context.base_url.clone()));
        let accountant = accountant::Accountant::new(private_client.clone());
        let trader = trader::Trader::new(private_client.clone());
        let sniffer = sniffer::Sniffer::new(
            private_client.clone(),
            public_client.clone());
        Merchant {
            accountant: std::sync::Arc::new(accountant),
            sniffer: std::sync::Arc::new(sniffer),
            trader: std::sync::Arc::new(trader),
        }
    }
}

impl<TConnector> agnostic::merchant::Merchant for Merchant<TConnector>
where
    TConnector: hyper::client::connect::Connect + Send + Sync + Clone + 'static,
{
    const ID: u8 = 3;

    fn accountant(&self) -> std::sync::Arc<dyn agnostic::market::Accountant> {
        self.accountant.clone()
    }

    fn trader(&self) -> std::sync::Arc<dyn agnostic::market::Trader> {
        self.trader.clone()
    }

    fn sniffer(&self) -> std::sync::Arc<dyn agnostic::market::Sniffer> {
        self.sniffer.clone()
    }
}
