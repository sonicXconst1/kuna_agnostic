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
    ) -> agnostic::market::Future<Result<(), String>> {
        let future = create_order(self.private_client.clone(), order);
        Box::pin(future)
    }

    fn delete_and_create(
        &self,
        id: &str,
        new_order: agnostic::order::Order,
    ) -> agnostic::market::Future<Result<String, String>> {
        let private_client = self.private_client.clone();
        let id = id.to_owned();
        let future = async move {
            match delete_order(private_client.clone(), id).await {
                Ok(_) => match create_order(private_client.clone(), new_order).await {
                    Ok(_) => Ok("delete and create finished".to_owned()),
                    Err(error) => Err(error),
                },
                Err(error) => return Err(error),
            }
        };
        Box::pin(future)
    }

    fn delete_order(&self, id: &str) -> agnostic::market::Future<Result<(), String>> {
        let future = delete_order(self.private_client.clone(), id.to_owned());
        Box::pin(future)
    }

    fn create_trade_from_order(&self, order: agnostic::order::Order) -> agnostic::market::Future<Result<(), String>> {
        let future = create_order(self.private_client.clone(), order);
        Box::pin(future)
    }
}

async fn create_order<TConnector>(
    private_client: std::sync::Arc<kuna_sdk::client::KunaClient<TConnector>>,
    order: agnostic::order::Order,
) -> Result<(), String> 
where
    TConnector: hyper::client::connect::Connect + Send + Sync + Clone + 'static
{
    use crate::convert;
    use agnostic::trading_pair::TradingPairConverter;
    use agnostic::trading_pair::Target;
    let converter = convert::CoinConverter::default();
    let kuna_symbol = converter.to_pair(order.trading_pair.clone());
    let target = match order.trading_pair.target {
        Target::Limit => kuna_sdk::base::Target::Limit,
        Target::Market => kuna_sdk::base::Target::Market,
    };
    let create_order = kuna_sdk::models::CreateOrder {
        symbol: kuna_symbol.to_string(),
        amount: order.amount,
        price: order.price,
        order_type: target.to_string(),
    };
    match private_client.create_order(create_order).await {
        Ok(result) => {
            log::debug!("{}", result);
            Ok(())
        },
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
