use crate::accountant;
use crate::sniffer;
use crate::trader;
use std::sync::Arc;

pub struct Merchant<TConnector> {
    accountant: Arc<accountant::Accountant<TConnector>>,
    sniffer: Arc<sniffer::Sniffer<TConnector>>,
    trader: Arc<trader::Trader<TConnector>>,
}

impl<TConnector> agnostic::merchant::Merchant for Merchant<TConnector>
where
    TConnector: hyper::client::connect::Connect + Send + Sync + Clone + 'static,
{
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
