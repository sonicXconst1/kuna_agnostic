use agnostic::trade::{Trade, TradeResult};
use agnostic::order::OrderWithId;

pub struct Trader<TConnector> {
    private_client: std::sync::Arc<kuna_sdk::client::KunaClient<TConnector>>,
}

impl<TConnector> Trader<TConnector> {
    pub fn new(
        private_client: std::sync::Arc<kuna_sdk::client::KunaClient<TConnector>>,
    ) -> Trader<TConnector> {
        Trader {
            private_client,
        }
    }
}

impl<TConnector> agnostic::market::Trader for Trader<TConnector>
where
    TConnector: hyper::client::connect::Connect + Send + Sync + Clone + 'static,
{
    fn create_order(
        &self,
        order: agnostic::order::Order
    ) -> agnostic::market::Future<Result<Trade, String>> {
        let future = create_order(self.private_client.clone(), order);
        Box::pin(future)
    }

    fn delete_order(&self, id: &str) -> agnostic::market::Future<Result<(), String>> {
        let future = delete_order(self.private_client.clone(), id.to_owned());
        Box::pin(future)
    }
}

async fn create_order<TConnector>(
    private_client: std::sync::Arc<kuna_sdk::client::KunaClient<TConnector>>,
    order: agnostic::order::Order,
) -> Result<Trade, String> 
where
    TConnector: hyper::client::connect::Connect + Send + Sync + Clone + 'static
{
    use crate::convert;
    use agnostic::trading_pair::TradingPairConverter;
    let converter = convert::CoinConverter::default();
    let trading_pair = order.trading_pair.clone();
    let kuna_symbol = converter.to_pair(order.trading_pair.clone());
    use agnostic::trading_pair::Target;
    let target = match order.trading_pair.target {
        Target::Limit => kuna_sdk::base::Target::Limit,
        Target::Market => kuna_sdk::base::Target::Market,
    };
    use agnostic::trading_pair::Side;
    let create_order = kuna_sdk::models::CreateOrder {
        symbol: kuna_symbol.to_string(),
        amount: match order.trading_pair.side {
            Side::Buy => order.amount,
            Side::Sell => -order.amount,
        },
        price: order.price,
        order_type: target.to_string(),
    };
    let price = order.price;
    let amount = order.amount;
    match private_client.create_order(create_order).await {
        Ok(result) => Ok(match order.trading_pair.target {
            Target::Market => Trade::Market(TradeResult {
                id: result.id.to_string(),
                trading_pair,
                price: result.price.map_or(price, |price| price),
                amount,
            }),
            Target::Limit => Trade::Limit(OrderWithId {
                id: result.id.to_string(),
                trading_pair,
                price,
                amount,
            }),
        }),
        Err(error) => Err(error),
    }
}

async fn delete_order<TConnector>(
    private_client: std::sync::Arc<kuna_sdk::client::KunaClient<TConnector>>,
    id: String,
) -> Result<(), String> 
where
    TConnector: hyper::client::connect::Connect + Send + Sync + Clone + 'static
{
    use std::str::FromStr;
    let id = match i32::from_str(id.as_ref()) {
        Ok(id) => id,
        Err(error) => return Err(format!("Failed to convert id to i32: {}. {}", id, error)),
    };
    let cancel_order = kuna_sdk::models::CancelOrderRequest {
        order_id: id,
    };
    match private_client.delete_order(cancel_order).await {
        Ok(_) => Ok(()),
        Err(error) => Err(error),
    }
}
