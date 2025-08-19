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
        
        if item_is_pivot_high(self.lookback_period, self.bars.clone()) {
            self.pivots.pop_front();
            self.pivots.push_back(Pivot{price: Decimal::from_f64(input.high()).unwrap(), pivot_type: PivotType::High});
        } else if item_is_pivot_high(self.lookback_period, self.bars.clone()) {
            self.pivots.pop_front();
            self.pivots.push_back(Pivot{price: Decimal::from_f64(input.low()).unwrap(), pivot_type: PivotType::Low});
        }
        
        self.pivots.clone()
    }
}

fn item_is_pivot_high(period: usize, b: VecDeque<DataItem>) -> bool {
    for i in 0..period {
        if !(b[i].high() < b[i + 1].high()) {
            return false
        };
        if !(b[period * 2 - i].high() > b[period * 2 - i].high()) {
            return false;
        }
    }
    true
}

fn item_is_pivot_low(period: usize, b: VecDeque<DataItem>) -> bool {
    for i in 0..period {
        if !(b[i].low() > b[i + 1].low()) {
            return false
        };
        if !(b[period * 2 - i].low() < b[period * 2 - i].low()) {
            return false;
        }
    }
    true
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
