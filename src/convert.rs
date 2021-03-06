use kuna_sdk::coin;
use agnostic::trading_pair;

#[derive(Default, Debug)] 
pub struct CoinConverter {
}

impl agnostic::trading_pair::TradingPairConverter for CoinConverter {
    type Pair = coin::Coins;
    type Coin = coin::Coin;

    fn to_string(&self, trading_pair: agnostic::trading_pair::TradingPair) -> String {
        self.to_pair(trading_pair).to_string()
    }

    fn to_pair(&self, trading_pair: agnostic::trading_pair::TradingPair) -> Self::Pair {
        match trading_pair.coins {
            trading_pair::Coins::TonUsdt => coin::Coins::TonUsdt,
        }
    }

    fn from_agnostic_coin(&self, coin: agnostic::trading_pair::Coin) -> Self::Coin {
        match coin {
            trading_pair::Coin::TON => coin::Coin::TON,
            trading_pair::Coin::USDT => coin::Coin::USDT,
        }
    }

    fn to_agnostic_coin(&self, coin: Self::Coin) -> Option<agnostic::trading_pair::Coin> {
        match coin {
            coin::Coin::TON => Some(trading_pair::Coin::TON),
            coin::Coin::USDT => Some(trading_pair::Coin::USDT),
            _ => None,
        }
    }
}
