use std::fmt;
use rust_decimal::Decimal;
use rust_decimal::prelude::FromPrimitive;
use rust_decimal_macros::dec;
use ta::errors::{Result, TaError};
use ta::{DataItem, High, Low, Next, Period, Reset};

use crate::average_true_range::AverageTrueRange;
use crate::model::ADX;
use crate::rolling_moving_average::RollingMovingAverage;

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
        let adx = get_adx_indicator(
            &di,
            self.atr.next(di),
            &Decimal::from_f64(self.current_di.low()).unwrap(),
            &Decimal::from_f64(self.current_di.high()).unwrap(),
            &mut self.dmi_plus,
            &mut self.dmi_minus,
            &mut self.adx,
            self.is_new,
        );

        if self.is_new {
            self.is_new = false;
        }

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
    atr_opt: Option<Decimal>,
    prev_low: &Decimal,
    prev_high: &Decimal,
    ema_di_plus: &mut RollingMovingAverage,
    ema_di_minus: &mut RollingMovingAverage,
    ema_di_adx: &mut RollingMovingAverage,
    is_new: bool,
) -> ADX {
    if is_new {
        ADX {
            adx_opt: None,
            di_plus_opt: None,
            di_minus_opt: None,
        }
    } else {
        let up_move = Decimal::from_f64(data_item.high()).unwrap() - prev_high;

        let down_move = prev_low - Decimal::from_f64(data_item.low()).unwrap();

        let (dm_plus, dm_minus) = if up_move > down_move && up_move > dec!(0) {
            (up_move, dec!(0))
        } else if down_move > up_move && down_move > dec!(0) {
            (dec!(0), down_move)
        } else {
            (dec!(0), dec!(0))
        };

        let atr_output = atr_opt.unwrap_or_else(|| dec!(1));

        println!(
            "dm_plus: {}, dm_minus: {}, atr: {}",
            dm_plus, dm_minus, atr_output
        );

        let di_plus_opt = ema_di_plus.next(dm_plus).map(|f| (f / atr_output) * dec!(100));
        let di_minus_opt = ema_di_minus
            .next(dm_minus)
            .map(|f| (f / atr_output) * dec!(100));

        let adx_temp_opt = match (di_plus_opt, di_minus_opt) {
            (Some(di_plus), Some(di_minus)) => {
                Some(((di_plus - di_minus) / (di_plus + di_minus)).abs())
            }
            _ => None,
        };

        // TODO: No unwrap
        let adx_opt = match adx_temp_opt {
            Some(adx_temp) => ema_di_adx.next(adx_temp).map(|adx| adx * dec!(100)),
            _ => None,
        };

        ADX {
            adx_opt,
            di_plus_opt,
            di_minus_opt,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        assert!(DirectionalMovementIndex::new(0).is_err());
        assert!(DirectionalMovementIndex::new(1).is_ok());
    }

    #[test]
    fn test_next() {
        let mut dmi = DirectionalMovementIndex::new(3).unwrap();

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

        println!("1) {:?}", dmi.next(&di1).di_plus_opt);
        println!("2) {:?}", dmi.next(&di2).di_plus_opt);
        println!("3) {:?}", dmi.next(&di3).di_plus_opt);
        println!("4) {:?}", dmi.next(&di4).di_plus_opt);
        println!("5) {:?}", dmi.next(&di5).di_plus_opt);
        println!("6) {:?}", dmi.next(&di6).di_plus_opt);
        println!("7) {:?}", dmi.next(&di7).di_plus_opt);

        // assert_eq!(ema.next(5.0), 3.5);
        // assert_eq!(ema.next(1.0), 2.25);
        // assert_eq!(ema.next(6.25), 4.25);
    }

    // #[test]
    // fn test_reset() {
    //     let mut ema = DirectionalMovementIndex::new(5).unwrap();

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
    //     DirectionalMovementIndex::default();
    // }

    // #[test]
    // fn test_display() {
    //     let ema = DirectionalMovementIndex::new(7).unwrap();
    //     assert_eq!(format!("{}", ema), "EMA(7)");
    // }
}
