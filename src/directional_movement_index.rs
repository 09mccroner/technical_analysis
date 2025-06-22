use std::fmt;

use ta::errors::{Result, TaError};
use ta::indicators::AverageTrueRange;
use ta::{DataItem, High, Low, Next, Period, Reset};

use crate::rolling_moving_average::RollingMovingAverage;
use crate::model::ADX;

#[derive(Debug, Clone)]
pub struct DirectionalMovementIndex {
    period: usize,
    dmi_plus: RollingMovingAverage,
    dmi_minus: RollingMovingAverage,
    adx: RollingMovingAverage,
    atr: AverageTrueRange,
    current_di: DataItem,
    is_new: bool,
}

impl DirectionalMovementIndex {
    pub fn new(period: usize) -> Result<Self> {
        match period {
            0 => Err(TaError::InvalidParameter),
            _ => Ok(Self {
                period,
                dmi_plus: RollingMovingAverage::new(period)?,
                dmi_minus: RollingMovingAverage::new(period)?,
                adx: RollingMovingAverage::new(period)?,
                atr: AverageTrueRange::new(period)?,
                current_di: empty_di()?,
                is_new: true,
            }),
        }
    }
}

impl Period for DirectionalMovementIndex {
    fn period(&self) -> usize {
        self.period
    }
}

impl Next<&DataItem> for DirectionalMovementIndex {
    type Output = ADX;

    fn next(&mut self, di: &DataItem) -> Self::Output {

        if self.is_new {
            self.is_new = false;
        }

        let adx = get_adx_indicator(
            &di, 
            self.atr.next(di), 
            &self.current_di.low(), 
            &self.current_di.high(), 
            &mut self.dmi_plus,
            &mut self.dmi_minus,
            &mut self.adx,
        );

        self.current_di = di.clone();
        adx
    }
}

impl Reset for DirectionalMovementIndex {
    fn reset(&mut self) {
        self.adx.reset();
        self.dmi_plus.reset();
        self.dmi_minus.reset();
        self.is_new = true;
    }
}

impl Default for DirectionalMovementIndex {
    fn default() -> Self {
        Self::new(14).unwrap()
    }
}

impl fmt::Display for DirectionalMovementIndex {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "RMA({})", self.period)
    }
}

fn empty_di() -> Result<DataItem> {
    DataItem::builder()
        .close(0.0)
        .high(0.0)
        .low(0.0)
        .open(0.0)
        .volume(0.0)
        .build()
}

pub fn get_adx_indicator(
    data_item: &DataItem, 
    atr_output: f64, 
    prev_low: &f64, 
    prev_high: &f64, 
    ema_di_plus: &mut RollingMovingAverage, 
    ema_di_minus: &mut RollingMovingAverage,
    ema_di_adx: &mut RollingMovingAverage,
) -> ADX {
    let up_move = data_item.high() - prev_high;

    let down_move = prev_low - data_item.low();

    let (dm_plus, dm_minus) = if up_move > down_move && up_move > 0.0 {
        (up_move, 0.0)
    } else if down_move > up_move && down_move > 0.0 {
        (0.0, down_move)
    } else {
        (0.0, 0.0)
    };

    if atr_output != 0.0 {
        let dm_temp_plus = dm_plus / atr_output;
        let dm_temp_minus = dm_minus / atr_output;
        let di_plus = 100.0 * ema_di_plus.next(dm_temp_plus);
        let di_minus = 100.0 * ema_di_minus.next(dm_temp_minus);

        let adx_temp = ((di_plus - di_minus) / (di_plus + di_minus)).abs();

        let adx = 100.0 * ema_di_adx.next(adx_temp);

        ADX{adx, di_plus, di_minus}
    } else {
        ADX{adx: 0.0, di_plus: 0.0, di_minus: 0.0}
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
