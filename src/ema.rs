use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use std::fmt;
use ta::errors::{Result, TaError};
use ta::{Close, Next, Period, Reset};

/// An exponential moving average (EMA), also known as an exponentially weighted moving average
/// (EWMA).
///
/// It is a type of infinite impulse response filter that applies weighting factors which decrease exponentially.
/// The weighting for each older datum decreases exponentially, never reaching zero.
///
/// # Formula
///
/// ![EMA formula](https://wikimedia.org/api/rest_v1/media/math/render/svg/05d06bdbee2c14031fd91ead6f5f772aec1ec964)
///
/// Where:
///
/// * _EMA<sub>t</sub>_ - is the value of the EMA at any time period _t_.
/// * _EMA<sub>t-1</sub>_ - is the value of the EMA at the previous period _t-1_.
/// * _p<sub>t</sub>_ - is the input value at a time period t.
/// * _α_ - is the coefficient that represents the degree of weighting decrease, a constant smoothing factor between 0 and 1.
///
/// _α_ is calculated with the following formula:
///
/// ![alpha formula](https://wikimedia.org/api/rest_v1/media/math/render/svg/d9f6258e152db0644af548972bd6c50a8becf7ee)
///
/// Where:
///
/// * _period_ - number of periods
///
/// # Parameters
///
/// * _period_ - number of periods (integer greater than 0)
///
/// # Example
///
/// ```
/// use ta::indicators::ExponentialMovingAverage;
/// use ta::Next;
///
/// let mut ema = ExponentialMovingAverage::new(3).unwrap();
/// assert_eq!(ema.next(2.0), 2.0);
/// assert_eq!(ema.next(5.0), 3.5);
/// assert_eq!(ema.next(1.0), 2.25);
/// assert_eq!(ema.next(6.25), 4.25);
/// ```
///
/// # Links
///
/// * [Exponential moving average, Wikipedia](https://en.wikipedia.org/wiki/Moving_average#Exponential_moving_average)
///

#[doc(alias = "EMA")]
#[derive(Debug, Clone)]
pub struct ExponentialMovingAverage {
    period: usize,
    k: Decimal,
    current: Decimal,
    is_new: bool,
}

impl ExponentialMovingAverage {
    pub fn new(period: usize) -> Result<Self> {
        match period {
            0 => Err(TaError::InvalidParameter),
            num => Ok(Self {
                period,
                k: dec!(2.0) / (Decimal::from(num) + dec!(1)),
                current: dec!(0),
                is_new: true,
            }),
        }
    }
}

impl Period for ExponentialMovingAverage {
    fn period(&self) -> usize {
        self.period
    }
}

impl Next<Decimal> for ExponentialMovingAverage {
    type Output = Decimal;

    fn next(&mut self, input: Decimal) -> Self::Output {
        if self.is_new {
            self.is_new = false;
            self.current = input;
        } else {
            self.current = self.k * input + (dec!(1) - self.k) * self.current;
        }
        self.current
    }
}

impl<T: Close> Next<&T> for ExponentialMovingAverage {
    type Output = Decimal;

    fn next(&mut self, input: &T) -> Self::Output {
        self.next(Decimal::from_f64_retain(input.close()).unwrap())
    }
}

impl Reset for ExponentialMovingAverage {
    fn reset(&mut self) {
        self.current = dec!(0);
        self.is_new = true;
    }
}

impl Default for ExponentialMovingAverage {
    fn default() -> Self {
        Self::new(9).unwrap()
    }
}

impl fmt::Display for ExponentialMovingAverage {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "EMA({})", self.period)
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use ta::test_helper::*;
//
//     test_indicator!(ExponentialMovingAverage);
//
//     #[test]
//     fn test_new() {
//         assert!(ExponentialMovingAverage::new(0).is_err());
//         assert!(ExponentialMovingAverage::new(1).is_ok());
//     }
//
//     #[test]
//     fn test_next() {
//         let mut ema = ExponentialMovingAverage::new(3).unwrap();
//
//         assert_eq!(ema.next(2.0), 2.0);
//         assert_eq!(ema.next(5.0), 3.5);
//         assert_eq!(ema.next(1.0), 2.25);
//         assert_eq!(ema.next(6.25), 4.25);
//
//         let mut ema = ExponentialMovingAverage::new(3).unwrap();
//         let bar1 = Bar::new().close(2);
//         let bar2 = Bar::new().close(5);
//         assert_eq!(ema.next(&bar1), 2.0);
//         assert_eq!(ema.next(&bar2), 3.5);
//     }
//
//     #[test]
//     fn test_reset() {
//         let mut ema = ExponentialMovingAverage::new(5).unwrap();
//
//         assert_eq!(ema.next(4.0), 4.0);
//         ema.next(10.0);
//         ema.next(15.0);
//         ema.next(20.0);
//         assert_ne!(ema.next(4.0), 4.0);
//
//         ema.reset();
//         assert_eq!(ema.next(4.0), 4.0);
//     }
//
//     #[test]
//     fn test_default() {
//         ExponentialMovingAverage::default();
//     }
//
//     #[test]
//     fn test_display() {
//         let ema = ExponentialMovingAverage::new(7).unwrap();
//         assert_eq!(format!("{}", ema), "EMA(7)");
//     }
// }
