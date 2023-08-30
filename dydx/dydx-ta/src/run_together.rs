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

use yata::core::{Action, Error, IndicatorResult, MovingAverageConstructor, OHLCV, PeriodType, Source, ValueType};
use yata::helpers::MA;
use yata::methods::Cross;
use yata::prelude::*;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct RunTogether<M: MovingAverageConstructor = MA> {
    pub ma1: M,
    pub ma2: M,

    price: ValueType,
    period: PeriodType,
    source: Source,
}

impl<M: MovingAverageConstructor> IndicatorConfig for RunTogether<M> {
    type Instance = RunTogetherInstance<M>;

    const NAME: &'static str = "RunTogether";

    fn validate(&self) -> bool {
        self.ma1.ma_period() < self.ma2.ma_period()
            && self.ma1.ma_period() > 1
            && self.price > 0.0
    }

    fn set(&mut self, name: &str, value: String) -> Result<(), Error> {
        match name {
            "ma1" => match value.parse() {
                Err(_) => return Err(Error::ParameterParse(name.to_string(), value.to_string())),
                Ok(value) => self.ma1 = value,
            },
            "ma2" => match value.parse() {
                Err(_) => return Err(Error::ParameterParse(name.to_string(), value.to_string())),
                Ok(value) => self.ma2 = value,
            },
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

    fn size(&self) -> (u8, u8) {
        (1, 2)
    }

    fn init<T: OHLCV>(self, candle: &T) -> Result<Self::Instance, Error> {
        if self.validate() {
            let cfg = self;
            let src = candle.source(cfg.source);
            Ok(Self::Instance {
                ma1: cfg.ma1.init(src)?,
                ma2: cfg.ma2.init(src)?,
                cross: Cross::default(),
                last_signal: Action::None,
                last_signal_position: 0,
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
            ma1: MA::EMA(12),
            ma2: MA::EMA(26),
            price: 2.0,
            period: 3,
            source: Source::Close,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct RunTogetherInstance<M: MovingAverageConstructor> {
    cfg: RunTogether<M>,
    ma1: M::Instance,
    ma2: M::Instance,
    cross: Cross,
    last_signal: Action,
    last_signal_position: PeriodType,
}

impl<M: MovingAverageConstructor> IndicatorInstance for RunTogetherInstance<M> {
    type Config = RunTogether<M>;

    fn config(&self) -> &Self::Config {
        &self.cfg
    }

    fn next<T: OHLCV>(&mut self, candle: &T) -> IndicatorResult {
        let src = &candle.source(self.cfg.source);
        let ema1 = self.ma1.next(src);
        let ema2 = self.ma2.next(src);

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