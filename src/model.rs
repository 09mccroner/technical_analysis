use rust_decimal::Decimal;

#[derive(Debug, serde::Deserialize, serde::Serialize, Clone, PartialEq)]
pub struct ADX {
    pub adx_opt: Option<Decimal>,
    pub di_plus_opt: Option<Decimal>,
    pub di_minus_opt: Option<Decimal>,
}
