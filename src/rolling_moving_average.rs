use std::fmt;
use rust_decimal::Decimal;
use ta::errors::{Result, TaError};
use crate::simple_moving_average::SimpleMovingAverage;
use ta::{Next, Period, Reset};

#[derive(Debug, Clone)]
pub struct RollingMovingAverage {
    period: usize,
    opt_current: Option<Decimal>,
    sma: SimpleMovingAverage,
    no_invokes: usize,
}

impl RollingMovingAverage {
    pub fn new(period: usize) -> Result<Self> {
        match period {
            0 => Err(TaError::InvalidParameter),
            _ => Ok(Self {
                period,
                opt_current: None,
                sma: SimpleMovingAverage::new(period)?,
                no_invokes: 0,
            }),
        }
    }
}

impl Period for RollingMovingAverage {
    fn period(&self) -> usize {
        self.period
    }
}

impl Next<Decimal> for RollingMovingAverage {
    type Output = Option<Decimal>;

    fn next(&mut self, input: Decimal) -> Self::Output {
        if self.no_invokes < self.period - 1 {
            self.no_invokes += 1;
            self.sma.next(input);
        } else if self.no_invokes == self.period - 1 {
            self.no_invokes += 1;
            self.opt_current = Some(self.sma.next(input));
        } else {
            // self.opt_current = Some(self.alpha * input + (1.0 - self.alpha) * self.opt_current);
            self.opt_current = self
                .opt_current
                .map(|current| (current * Decimal::from(self.period - 1) + input) / Decimal::from(self.period));
        }
        self.opt_current
    }
}

impl Reset for RollingMovingAverage {
    fn reset(&mut self) {
        self.opt_current = None;
        self.no_invokes = 0;
        self.sma = SimpleMovingAverage::new(self.period).unwrap();
    }
}

impl Default for RollingMovingAverage {
    fn default() -> Self {
        Self::new(14).unwrap()
    }
}

impl fmt::Display for RollingMovingAverage {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "RMA({})", self.period)
    }
}

#[cfg(test)]
mod tests {
    use rust_decimal_macros::dec;
    use super::*;

    #[test]
    fn test_new() {
        assert!(RollingMovingAverage::new(0).is_err());
        assert!(RollingMovingAverage::new(1).is_ok());
    }

    #[test]
    fn test_next() {
        let mut rma = RollingMovingAverage::new(14).unwrap();

        assert_eq!(rma.next(dec!(100.0)), None);
        assert_eq!(rma.next(dec!(102.0)), None);
        assert_eq!(rma.next(dec!(101.0)), None);
        assert_eq!(rma.next(dec!(103.0)), None);
        assert_eq!(rma.next(dec!(102.0)), None);
        assert_eq!(rma.next(dec!(104.0)), None);
        assert_eq!(rma.next(dec!(105.0)), None);
        assert_eq!(rma.next(dec!(106.0)), None);
        assert_eq!(rma.next(dec!(108.0)), None);
        assert_eq!(rma.next(dec!(107.0)), None);
        assert_eq!(rma.next(dec!(109.0)), None);
        assert_eq!(rma.next(dec!(110.0)), None);
        assert_eq!(rma.next(dec!(111.0)), None);
        assert_eq!(rma.next(dec!(113.0)).unwrap().round_dp(4), dec!(105.7857));
        assert_eq!(rma.next(dec!(115.0)).unwrap().round_dp(4), dec!(106.4439));
    }

    // #[test]
    // fn test_reset() {
    //     let mut ema = ExponentialMovingAverage::new(5).unwrap();

    //     assert_eq!(ema.next(4.0), 4.0);
    //     ema.next(10.0);
    //     ema.next(15.0);
    //     ema.next(20.0);
    //     assert_ne!(ema.next(4.0), 4.0);

    //     ema.reset();
    //     assert_eq!(ema.next(4.0), 4.0);
    // }

    // #[test]
    // fn test_default() {
    //     ExponentialMovingAverage::default();
    // }

    // #[test]
    // fn test_display() {
    //     let ema = ExponentialMovingAverage::new(7).unwrap();
    //     assert_eq!(format!("{}", ema), "EMA(7)");
    // }
}
