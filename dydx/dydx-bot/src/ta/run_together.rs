//! This is an "Run Together" indicator
//!
//! It has a **Configuration** with parameters `price`, `period` and `source`.
//!
//! Algorithm:
//! a.	Enter deal: Use a few indicators which shows unusual market activity:
// i.	Price makes a new high and volume increases > 2 times more than average.
//      Confirm price and volumes on 3 timeframes: 1min, 1h, 1d.
//      Average comparison should be to the same period during last 3 months
//      (i.e. 1 min Fri 9 am should compare with previous Fridays 9am)
// ii.	Test and add other filters: Confirm strong RSI > 40 on all these periods, confirm strong momentum etc.
// iii.	Later we will add more filters with L2 data (i.e. number of buy/sell orders)
// b.	Take profit: 3 pips exit – open limit order immediately, linked to filled open.
//      Also test option with 4 pips with trailing stop 1 pip.
// c.	Stop loss: 2 pips or 1 min exit.

use yata::core::{Action, Error, IndicatorResult, PeriodType, Source, ValueType, OHLCV};
use yata::prelude::*;

use yata::methods::Cross;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct RunTogether {
    price: ValueType,
    period: PeriodType,
    source: Source,
}

impl IndicatorConfig for RunTogether {
    type Instance = RunTogetherInstance;

    const NAME: &'static str = "RunTogether";

    fn init<T: OHLCV>(self, _candle: &T) -> Result<Self::Instance, Error> {
        if !self.validate() {
            return Err(Error::WrongConfig);
        }

        let cfg = self;
        Ok(Self::Instance {
            cross: Cross::default(),
            last_signal: Action::None,
            last_signal_position: 0,
            cfg,
        })
    }

    /// Validates config values to be consistent
    fn validate(&self) -> bool {
        self.price > 0.0
    }

    /// Sets attributes of config by given name and value by `String`
    fn set(&mut self, name: &str, value: String) -> Result<(), Error> {
        match name {
            "price" => match value.parse() {
                Err(_) => return Err(Error::ParameterParse(name.to_string(), value.to_string())),
                Ok(value) => self.price = value,
            },

            _ => {
                return Err(Error::ParameterParse(name.to_string(), value));
            }
        };

        Ok(())
    }

    /// Our indicator will return single raw value and two signals
    fn size(&self) -> (u8, u8) {
        (1, 2)
    }
}

impl Default for RunTogether {
    fn default() -> Self {
        Self {
            price: 2.0,
            period: 3,
            source: Source::Close,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct RunTogetherInstance {
    cfg: RunTogether,

    cross: Cross,
    last_signal: Action,
    last_signal_position: PeriodType,
}

impl IndicatorInstance for RunTogetherInstance {
    type Config = RunTogether;

    fn config(&self) -> &Self::Config {
        &self.cfg
    }

    /// Calculates next value by giving [`OHLCV`](crate::core::OHLCV)-object
    fn next<T: OHLCV>(&mut self, candle: &T) -> IndicatorResult {
        let new_signal = self.cross.next(&(candle.close(), self.cfg.price));

        let signal = if new_signal == Action::None {
            self.last_signal = new_signal;
            self.last_signal_position = 0;
            new_signal
        } else {
            if Action::None != self.last_signal {
                self.last_signal_position += 1;
                if self.last_signal_position > self.cfg.period {
                    self.last_signal = Action::None;
                }
            }

            self.last_signal
        };

        let some_other_signal = Action::from(0.5);

        IndicatorResult::new(&[candle.close()], &[signal, some_other_signal])
    }
}