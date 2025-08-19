use std::collections::VecDeque;
use rust_decimal::Decimal;
use rust_decimal::prelude::FromPrimitive;
use rust_decimal_macros::dec;
use ta::errors::{Result, TaError};
use ta::{DataItem, High, Low, Next};
use crate::pivot::pivot_points::PivotType::Unknown;

#[derive(Debug, Clone)]
pub struct PivotPoints {
    lookback_period: usize,
    num_pivots: usize,
    pivots: VecDeque<Pivot>,
    bars: VecDeque<DataItem>,
}

#[derive(Debug, Clone)]
pub struct Pivot {
    price: Decimal,
    pivot_type: PivotType
}

#[derive(Debug, Clone)]
#[derive(PartialEq)]
pub enum PivotType {
    High,
    Low,
    Unknown,
}

impl PivotPoints {
    pub fn new(lookback_period: usize, num_pivots: usize) -> Result<Self> {
        match (lookback_period, num_pivots) {
            (0, 0) => Err(TaError::InvalidParameter),
            _ => Ok(Self {
                lookback_period,
                num_pivots,
                pivots: VecDeque::from(vec![Pivot{price: dec!(0), pivot_type: Unknown}; num_pivots]),
                bars: VecDeque::from(vec![default_bar()?; lookback_period * 2 + 1]),
            }),
        }
    }
}

impl Next<&DataItem> for PivotPoints {
    type Output = VecDeque<Pivot>;

    fn next(&mut self, input: &DataItem) -> Self::Output {
        self.bars.pop_front();
        self.bars.push_back(input.clone());

        if let Some(ph) = find_pivot_high(self.lookback_period, self.bars.clone()) {
            self.pivots.pop_front();
            self.pivots.push_back(Pivot{price: Decimal::from_f64(ph).unwrap(), pivot_type: PivotType::High});
        }

        if let Some(pl) = find_pivot_low(self.lookback_period, self.bars.clone()) {
            self.pivots.pop_front();
            self.pivots.push_back(Pivot{price: Decimal::from_f64(pl).unwrap(), pivot_type: PivotType::Low});
        }
        
        self.pivots.clone()
    }
}

fn find_pivot_high(period: usize, b: VecDeque<DataItem>) -> Option<f64> {
    for i in 0..2 * period {
        if i >= period {
            if b[i].high() <= b[i+1].high() {
                return None;
            };
        } else {
            if b[i].high() >= b[i+1].high() {
                return None;
            };
        }
    }
    Some(b[period].high())
}

fn find_pivot_low(period: usize, b: VecDeque<DataItem>) -> Option<f64> {
    for i in 0..2 * period {
        if i >= period {
            if b[i].low() >= b[i+1].low() {
                return None;
            };
        } else {
            if b[i].low() <= b[i+1].low() {
                return None;
            };
        }
    }
    Some(b[period].low())
}

impl Default for PivotPoints {
    fn default() -> Self {
        Self::new(3, 5).unwrap()
    }
}

fn default_bar() -> Result<DataItem> {
    DataItem::builder()
        .open(0.0)
        .close(0.0)
        .high(0.0)
        .low(0.0)
        .volume(0.0)
        .build()
}

#[cfg(test)]
mod tests {
    use super::*;
    use ta::Next;
    use crate::PivotType::{High, Low};

    fn setup_di_highs(high: f64) -> DataItem {
        DataItem::builder()
            .open(high)
            .close(high)
            .high(high)
            .low(high)
            .volume(high)
            .build()
            .unwrap()
    }

    fn setup_di_lows(low: f64) -> DataItem {
        DataItem::builder()
            .open(low)
            .close(low)
            .high(low)
            .low(low)
            .volume(low)
            .build()
            .unwrap()
    }

    #[test]
    fn test_pivot_points_high() {
        let mut pp = PivotPoints::new(2, 3).unwrap();

        pp.next(&setup_di_highs(0.1));
        pp.next(&setup_di_highs(0.2));
        pp.next(&setup_di_highs(0.3));
        pp.next(&setup_di_highs(0.2));
        let out = pp.next(&setup_di_highs(0.1));

        println!("{:?}", out);
        println!("{:?}", pp);

        assert_eq!(out.back().unwrap().price, dec!(0.3));
        assert_eq!(out.back().unwrap().pivot_type, High);
    }

    #[test]
    fn test_pivot_points_low() {
        let mut pp = PivotPoints::new(2, 3).unwrap();

        pp.next(&setup_di_lows(0.3));
        pp.next(&setup_di_lows(0.2));
        pp.next(&setup_di_lows(0.1));
        pp.next(&setup_di_lows(0.2));
        let out = pp.next(&setup_di_lows(0.3));

        assert_eq!(out.back().unwrap().price, dec!(0.1));
        assert_eq!(out.back().unwrap().pivot_type, Low);
    }
}