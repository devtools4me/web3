// This is an "Source Change" indicator
//
// It has a **Configuration** with parameters `ma`, `source` and `k`.
//
// Algorithm:
//   Enter deal: Use a few indicators which shows unusual market activity:
// 	 Source value makes increases > k times more than average.

use yata::core::{Action, Error, IndicatorResult, MovingAverageConstructor, OHLCV, PeriodType, Source, ValueType};
use yata::helpers::MA;
use yata::indicators::RSI;
use yata::methods::Cross;
use yata::prelude::*;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct SourceChange<M: MovingAverageConstructor = MA> {
    pub ma: M,

    source: Source,
    k: ValueType,
}

impl<M: MovingAverageConstructor> IndicatorConfig for SourceChange<M> {
    type Instance = SourceChangeInstance<M>;

    const NAME: &'static str = "SourceChange";

    fn validate(&self) -> bool {
        self.ma.ma_period() > 1 && self.k > 0.0
    }

    fn set(&mut self, name: &str, value: String) -> Result<(), Error> {
        match name {
            "ma" => match value.parse() {
                Err(_) => return Err(Error::ParameterParse(name.to_string(), value.to_string())),
                Ok(value) => self.ma = value,
            },
            "k" => match value.parse() {
                Err(_) => return Err(Error::ParameterParse(name.to_string(), value.to_string())),
                Ok(value) => self.k = value,
            },
            _ => {
                return Err(Error::ParameterParse(name.to_string(), value));
            }
        };

        Ok(())
    }

    fn size(&self) -> (u8, u8) {
        (1, 1)
    }

    fn init<T: OHLCV>(self, candle: &T) -> Result<Self::Instance, Error> {
        if self.validate() {
            let cfg = self;
            let src = candle.source(cfg.source);
            Ok(Self::Instance {
                ma: cfg.ma.init(src)?,
                cfg,
            })
        } else {
            return Err(Error::WrongConfig);
        }
    }
}

impl Default for SourceChange {
    fn default() -> Self {
        Self {
            ma: MA::EMA(20),
            source: Source::Close,
            k: 2.0
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct SourceChangeInstance<M: MovingAverageConstructor> {
    cfg: SourceChange<M>,
    ma: M::Instance
}

impl<M: MovingAverageConstructor> IndicatorInstance for SourceChangeInstance<M> {
    type Config = SourceChange<M>;

    fn config(&self) -> &Self::Config {
        &self.cfg
    }

    fn next<T: OHLCV>(&mut self, candle: &T) -> IndicatorResult {
        let src = &candle.source(self.cfg.source);
        let average_value: ValueType = self.ma.next(src);
        let signal = if src / average_value > self.cfg.k {
            Action::Buy(10)
        } else {
            Action::None
        };

        IndicatorResult::new(&[src.clone()], &[signal])
    }
}