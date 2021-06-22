use agnostic::trading_pair::TradingPairConverter;
use agnostic::trading_pair;
use crate::convert;

pub struct Sniffer<TConnector> {
    private_client: std::sync::Arc<kuna_sdk::client::KunaClient<TConnector>>,
    public_client: std::sync::Arc<kuna_sdk::public::KunaPublicClient<TConnector>>,
}

impl<TConnector> Sniffer<TConnector>
where
    TConnector: hyper::client::connect::Connect + Send + Sync + 'static
{
    pub fn new(
        private_client: std::sync::Arc<kuna_sdk::client::KunaClient<TConnector>>,
        public_client: std::sync::Arc<kuna_sdk::public::KunaPublicClient<TConnector>>,
    ) -> Sniffer<TConnector> {
        Sniffer {
            private_client,
            public_client,
        }
    }
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
        let future = get_all_orders(self.public_client.clone(), trading_pair, count);
        Box::pin(future)
    }

    fn get_my_orders(
        &self,
        trading_pair: agnostic::trading_pair::TradingPair,
    ) -> agnostic::market::Future<Result<Vec<agnostic::order::OrderWithId>, String>> {
        let private_client = self.private_client.clone();
        let future = async move {
            let converter = convert::CoinConverter::default();
            let kuna_pair = converter.to_pair(trading_pair.clone());
            let raw_orders = private_client.get_my_orders(kuna_pair).await?;
            Ok(raw_orders.into_iter()
                .map(|order| agnostic::order::OrderWithId {
                    id: order.id.to_string(),
                    trading_pair: trading_pair.clone(),
                    price: order.price.expect("Price could not be null here!"),
                    amount: order.executed_amount.abs(),
                })
                .collect())
        };
        Box::pin(future)
    }
}

async fn get_all_orders<TConnector>(
    public_client: std::sync::Arc<kuna_sdk::public::KunaPublicClient<TConnector>>,
    trading_pair: trading_pair::TradingPair,
    count: u32,
) -> Result<Vec<agnostic::order::Order>, String> 
where 
    TConnector: hyper::client::connect::Connect + Send + Sync + Clone + 'static
{
    let converter = convert::CoinConverter::default();
    let pair = converter.to_pair(trading_pair.clone());
    let order_book = match public_client.get_orderbook(pair).await {
        Ok(order_book) => order_book,
        Err(error) => return Err(error),
    };
    let orders = match trading_pair.target {
        trading_pair::Target::Market => {
            match trading_pair.side {
                trading_pair::Side::Buy => {
                    order_book.asks
                },
                trading_pair::Side::Sell => {
                    order_book.bids
                },
            }
        },
        trading_pair::Target::Limit => {
            match trading_pair.side {
                trading_pair::Side::Buy => {
                    order_book.bids
                },
                trading_pair::Side::Sell => {
                    order_book.asks
                },
            }
        },
    };
    let orders = orders.into_iter()
        .take(count as usize)
        .map(|order| agnostic::order::Order {
            trading_pair: trading_pair.clone(),
            price: order.price,
            amount: order.amount,
        })
        .collect();
    Ok(orders)
}

