use std::fmt;

use ta::errors::{Result, TaError};
use ta::indicators::SimpleMovingAverage;
use ta::{Next, Period, Reset};

#[derive(Debug, Clone)]
pub struct RollingMovingAverage {
    period: usize,
    opt_current: Option<f64>,
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

impl Next<f64> for RollingMovingAverage {
    type Output = Option<f64>;

    fn next(&mut self, input: f64) -> Self::Output {
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
                .map(|current| (current * (self.period as f64 - 1.0) + input) / self.period as f64);
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
    use super::*;

    #[test]
    fn test_new() {
        assert!(RollingMovingAverage::new(0).is_err());
        assert!(RollingMovingAverage::new(1).is_ok());
    }

    #[test]
    fn test_next() {
        let mut rma = RollingMovingAverage::new(14).unwrap();

        assert_eq!(rma.next(100.0), None);
        assert_eq!(rma.next(102.0), None);
        assert_eq!(rma.next(101.0), None);
        assert_eq!(rma.next(103.0), None);
        assert_eq!(rma.next(102.0), None);
        assert_eq!(rma.next(104.0), None);
        assert_eq!(rma.next(105.0), None);
        assert_eq!(rma.next(106.0), None);
        assert_eq!(rma.next(108.0), None);
        assert_eq!(rma.next(107.0), None);
        assert_eq!(rma.next(109.0), None);
        assert_eq!(rma.next(110.0), None);
        assert_eq!(rma.next(111.0), None);
        assert_eq!(rma.next(113.0), Some(105.78571428571429));
        assert_eq!(rma.next(115.0), Some(106.4438775510204));
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
