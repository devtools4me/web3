// This is an "Sell Volatility" indicator
//
// It has a **Configuration** with parameters `price`, `period` and `source`.
//
// Algorithm:
// Enter deal:
//   Sell SPX put, ATM (at the money), every 3 months (most liquid one, 3d Fri every 3 month, starting from September 23, 2023).
//   Test with position size 1.
// Interim Hedge:
//   Sell MES SEP future if SPX drops 2.5% (later level could be adjusted based on Option premium size).
//   Position size 2 (due to lot size differences)
// Stop loss:
//   if interim hedge is executed and SPX price goes up by 1% compared to future position opening price (Strike * 0.985 * 1.01),
//   close future position and put trailing take profit to option position with 1% window
// Exit:
//   either by stop loss or nothing (option and future will expire by maturity)

use yata::core::{Action, Error, IndicatorResult, MovingAverageConstructor, OHLCV, PeriodType, Source, ValueType};
use yata::helpers::MA;
use yata::methods::Cross;
use yata::prelude::*;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct SellVolatility<M: MovingAverageConstructor = MA> {
    pub ma1: M,
    pub ma2: M,

    price: ValueType,
    period: PeriodType,
    source: Source,
}

impl<M: MovingAverageConstructor> IndicatorConfig for SellVolatility<M> {
    type Instance = SellVolatilityInstance<M>;

    const NAME: &'static str = "SellVolatility";

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

impl Default for SellVolatility {
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
pub struct SellVolatilityInstance<M: MovingAverageConstructor> {
    cfg: SellVolatility<M>,
    ma1: M::Instance,
    ma2: M::Instance,
    cross: Cross,
    last_signal: Action,
    last_signal_position: PeriodType,
}

impl<M: MovingAverageConstructor> IndicatorInstance for SellVolatilityInstance<M> {
    type Config = SellVolatility<M>;

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