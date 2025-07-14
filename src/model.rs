#[derive(Debug, serde::Deserialize, serde::Serialize, Clone, PartialEq)]
pub struct ADX {
    pub adx_opt: Option<f64>,
    pub di_plus_opt: Option<f64>,
    pub di_minus_opt: Option<f64>,
}
