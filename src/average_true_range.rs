use std::fmt;
use rust_decimal::Decimal;
use ta::errors::Result;
use crate::true_range::TrueRange;
use ta::{Close, High, Low, Next, Period, Reset};

use crate::rolling_moving_average::RollingMovingAverage;

/// Average true range (ATR).
///
/// A technical analysis volatility indicator, originally developed by J. Welles Wilder.
/// The average true range is an N-day smoothed moving average of the true range values.
/// This implementation uses exponential moving average.
///
/// # Formula
///
/// ATR(period)<sub>t</sub> = EMA(period) of TR<sub>t</sub>
///
/// Where:
///
/// * _EMA(period)_ - [exponential moving average](struct.ExponentialMovingAverage.html) with smoothing period
/// * _TR<sub>t</sub>_ - [true range](struct.TrueRange.html) for period _t_
///
/// # Parameters
///
/// * _period_ - smoothing period of EMA (integer greater than 0)
///
/// # Example
///
/// ```
/// extern crate ta;
/// #[macro_use] extern crate assert_approx_eq;
///
/// use ta::{Next, DataItem};
/// use ta::indicators::AverageTrueRange;
///
/// fn main() {
///     let data = vec![
///         // open, high, low, close, atr
///         (9.7   , 10.0, 9.0, 9.5  , 1.0),    // tr = high - low = 10.0 - 9.0 = 1.0
///         (9.9   , 10.4, 9.8, 10.2 , 0.95),   // tr = high - prev_close = 10.4 - 9.5 = 0.9
///         (10.1  , 10.7, 9.4, 9.7  , 1.125),  // tr = high - low = 10.7 - 9.4 = 1.3
///         (9.1   , 9.2 , 8.1, 8.4  , 1.3625), // tr = prev_close - low = 9.7 - 8.1 = 1.6
///     ];
///     let mut indicator = AverageTrueRange::new(3).unwrap();
///
///     for (open, high, low, close, atr) in data {
///         let di = DataItem::builder()
///             .high(high)
///             .low(low)
///             .close(close)
///             .open(open)
///             .volume(1000.0)
///             .build().unwrap();
///         assert_approx_eq!(indicator.next(&di), atr);
///     }
/// }
#[doc(alias = "ATR")]
#[derive(Debug, Clone)]
pub struct AverageTrueRange {
    true_range: TrueRange,
    rma: RollingMovingAverage,
}

impl AverageTrueRange {
    pub fn new(period: usize) -> Result<Self> {
        Ok(Self {
            true_range: TrueRange::new(),
            rma: RollingMovingAverage::new(period)?,
        })
    }
}

impl Period for AverageTrueRange {
    fn period(&self) -> usize {
        self.rma.period()
    }
}

impl Next<Decimal> for AverageTrueRange {
    type Output = Option<Decimal>;

    fn next(&mut self, input: Decimal) -> Self::Output {
        self.rma.next(Decimal::from(self.true_range.next(input)))
    }
}

impl<T: High + Low + Close> Next<&T> for AverageTrueRange {
    type Output = Option<Decimal>;

    fn next(&mut self, input: &T) -> Self::Output {
        self.rma.next(self.true_range.next(input))
    }
}

impl Reset for AverageTrueRange {
    fn reset(&mut self) {
        self.true_range.reset();
        self.rma.reset();
    }
}

impl Default for AverageTrueRange {
    fn default() -> Self {
        Self::new(14).unwrap()
    }
}

impl fmt::Display for AverageTrueRange {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ATR({})", self.rma.period())
    }
}

#[cfg(test)]
mod tests {
    use ta::DataItem;

    use super::*;

    #[test]
    fn test_new() {
        assert!(AverageTrueRange::new(0).is_err());
        assert!(AverageTrueRange::new(1).is_ok());
    }
    #[test]
    fn test_next() {
        let mut atr = AverageTrueRange::new(3).unwrap();

        let di1 = DataItem::builder()
            .close(5.3537490)
            .open(0.0984338)
            .high(8.2830820)
            .low(0.0831237)
            .volume(1.0)
            .build()
            .unwrap();

        let di2 = DataItem::builder()
            .close(4.0971314)
            .open(5.3537490)
            .high(16.9121161)
            .low(3.0669901)
            .volume(1.0)
            .build()
            .unwrap();

        let di3 = DataItem::builder()
            .close(1.5416779)
            .open(4.0971314)
            .high(4.3877257)
            .low(1.1157023)
            .volume(1.0)
            .build()
            .unwrap();

        let di4 = DataItem::builder()
            .close(0.8320898)
            .open(1.5416779)
            .high(2.3969461)
            .low(0.7421172)
            .volume(1.0)
            .build()
            .unwrap();

        let di5 = DataItem::builder()
            .close(0.3684033)
            .open(0.8320898)
            .high(1.2149281)
            .low(0.2187003)
            .volume(1.0)
            .build()
            .unwrap();

        let di6 = DataItem::builder()
            .close(0.8053857)
            .open(0.3684033)
            .high(1.7971078)
            .low(0.2623044)
            .volume(1.0)
            .build()
            .unwrap();

        let di7 = DataItem::builder()
            .close(0.3877222)
            .open(0.8053857)
            .high(0.9516707)
            .low(0.3649837)
            .volume(1.0)
            .build()
            .unwrap();

        println!("1) {:?}", atr.next(&di1));
        println!("2) {:?}", atr.next(&di2));
        println!("3) {:?}", atr.next(&di3));
        println!("4) {:?}", atr.next(&di4));
        println!("5) {:?}", atr.next(&di5));
        println!("6) {:?}", atr.next(&di6));
        println!("7) {:?}", atr.next(&di7));
    }
}
//     #[test]
//     fn test_reset() {
//         let mut atr = AverageTrueRange::new(9).unwrap();

//         let bar1 = Bar::new().high(10).low(7.5).close(9);
//         let bar2 = Bar::new().high(11).low(9).close(9.5);

//         atr.next(&bar1);
//         atr.next(&bar2);

//         atr.reset();
//         let bar3 = Bar::new().high(60).low(15).close(51);
//         assert_eq!(atr.next(&bar3), 45.0);
//     }

//     #[test]
//     fn test_default() {
//         AverageTrueRange::default();
//     }

//     #[test]
//     fn test_display() {
//         let indicator = AverageTrueRange::new(8).unwrap();
//         assert_eq!(format!("{}", indicator), "ATR(8)");
//     }
// }
