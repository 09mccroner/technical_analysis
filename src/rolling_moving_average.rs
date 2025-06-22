use std::fmt;

use ta::errors::{Result, TaError};
use ta::{indicators, Close, Next, Period, Reset};

#[derive(Debug, Clone)]
pub struct RollingMovingAverage {
    period: usize,
    alpha: f64,
    current: f64,
    is_new: bool,
}

impl RollingMovingAverage {
    pub fn new(period: usize) -> Result<Self> {
        match period {
            0 => Err(TaError::InvalidParameter),
            _ => Ok(Self {
                period,
                alpha: 1.0 / (period as f64),
                current: 0.0,
                is_new: true,
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
    type Output = f64;

    fn next(&mut self, input: f64) -> Self::Output {
        if self.is_new {
            self.is_new = false;
            let mut sma = indicators::SimpleMovingAverage::new(self.period).unwrap();
            self.current = sma.next(input);
        } else {
            self.current = self.alpha * input + (1.0 - self.alpha) * self.current;
        }
        self.current
    }
}

impl<T: Close> Next<&T> for RollingMovingAverage {
    type Output = f64;

    fn next(&mut self, input: &T) -> Self::Output {
        self.next(input.close())
    }
}

impl Reset for RollingMovingAverage {
    fn reset(&mut self) {
        self.current = 0.0;
        self.is_new = true;
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

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use crate::test_helper::*;

//     test_indicator!(ExponentialMovingAverage);

//     #[test]
//     fn test_new() {
//         assert!(ExponentialMovingAverage::new(0).is_err());
//         assert!(ExponentialMovingAverage::new(1).is_ok());
//     }

//     #[test]
//     fn test_next() {
//         let mut ema = ExponentialMovingAverage::new(3).unwrap();

//         assert_eq!(ema.next(2.0), 2.0);
//         assert_eq!(ema.next(5.0), 3.5);
//         assert_eq!(ema.next(1.0), 2.25);
//         assert_eq!(ema.next(6.25), 4.25);

//         let mut ema = ExponentialMovingAverage::new(3).unwrap();
//         let bar1 = Bar::new().close(2);
//         let bar2 = Bar::new().close(5);
//         assert_eq!(ema.next(&bar1), 2.0);
//         assert_eq!(ema.next(&bar2), 3.5);
//     }

//     #[test]
//     fn test_reset() {
//         let mut ema = ExponentialMovingAverage::new(5).unwrap();

//         assert_eq!(ema.next(4.0), 4.0);
//         ema.next(10.0);
//         ema.next(15.0);
//         ema.next(20.0);
//         assert_ne!(ema.next(4.0), 4.0);

//         ema.reset();
//         assert_eq!(ema.next(4.0), 4.0);
//     }

//     #[test]
//     fn test_default() {
//         ExponentialMovingAverage::default();
//     }

//     #[test]
//     fn test_display() {
//         let ema = ExponentialMovingAverage::new(7).unwrap();
//         assert_eq!(format!("{}", ema), "EMA(7)");
//     }
// }
