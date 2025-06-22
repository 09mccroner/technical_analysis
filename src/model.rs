#[derive(Debug, serde::Deserialize, serde::Serialize, Clone)]
pub struct ADX {
    pub adx: f64,
    pub di_plus: f64,
    pub di_minus: f64,
}
