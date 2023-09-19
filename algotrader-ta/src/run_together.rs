// This is an "Run Together" indicator
//
// It has a **Configuration** with parameters `price`, `period` and `source`.
//
// Algorithm:
// a.	Enter deal: Use a few indicators which shows unusual market activity:
// i.	Price makes a new high and volume increases > 2 times more than average.
//      Confirm price and volumes on 3 timeframes: 1min, 1h, 1d.
//      Average comparison should be to the same period during last 3 months
//      (i.e. 1 min Fri 9 am should compare with previous Fridays 9am)
// ii.	Test and add other filters: Confirm strong RSI > 40 on all these periods, confirm strong momentum etc.
// iii.	Later we will add more filters with L2 data (i.e. number of buy/sell orders)
// b.	Take profit: 3 pips exit – open limit order immediately, linked to filled open.
//      Also test option with 4 pips with trailing stop 1 pip.
// c.	Stop loss: 2 pips or 1 min exit.

use yata::core::{Action, Error, IndicatorResult, OHLCV};
use yata::prelude::*;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use crate::source_change::SourceChange;

#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct RunTogether {
    pub i1: SourceChange,
    pub i2: SourceChange,
}

impl IndicatorConfig for RunTogether {
    type Instance = RunTogetherInstance;

    const NAME: &'static str = "RunTogether";

    fn validate(&self) -> bool {
        self.i1.validate() && self.i2.validate()
    }

    fn set(&mut self, name: &str, value: String) -> Result<(), Error> {
        match name {
            "i1" => SourceChange::default(),
                // match value.parse() {
                //     Err(_) => return Err(Error::ParameterParse(name.to_string(), value.to_string())),
                //     Ok(value) => self.i1 = value,
                // },
            "i2" => SourceChange::default(),
                // match value.parse() {
                //     Err(_) => return Err(Error::ParameterParse(name.to_string(), value.to_string())),
                //     Ok(value) => self.i2 = value,
                // },
            _ => {
                return Err(Error::ParameterParse(name.to_string(), value));
            }
        };

        Ok(())
    }

    fn size(&self) -> (u8, u8) {
        (2, 1)
    }

    fn init<T: OHLCV>(self, candle: &T) -> Result<Self::Instance, Error> {
        if self.validate() {
            let cfg = self;
            Ok(Self::Instance {
                i1: cfg.i1.init(candle)?,
                i2: cfg.i2.init(candle)?,
                cfg,
            })
        } else {
            return Err(Error::WrongConfig);
        }
    }
}

impl Default for RunTogether {
    fn default() -> Self {
        Self {
            i1: SourceChange::default(),
            i2: SourceChange::default(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct RunTogetherInstance {
    pub cfg: RunTogether,
    pub i1: <SourceChange as IndicatorConfig>::Instance,
    pub i2: <SourceChange as IndicatorConfig>::Instance,
}

impl IndicatorInstance for RunTogetherInstance {
    type Config = RunTogether;

    fn config(&self) -> &Self::Config {
        &self.cfg
    }

    fn next<T: OHLCV>(&mut self, candle: &T) -> IndicatorResult {
        let res1 = self.i1.next(candle);
        let res2 = self.i2.next(candle);
        let v1 = res1.values().get(0).unwrap();
        let sig1 = res1.signals().get(0).unwrap();
        let v2 = res2.values().get(0).unwrap();
        let sig2 = res2.signals().get(0).unwrap();

        let signal = match (sig1, sig2) {
            (Action::Buy(_), Action::Buy(_)) => Action::Buy(1),
            (_, _) => Action::None
        };

        IndicatorResult::new(&[v1.clone(), v2.clone()], &[signal])
    }
}