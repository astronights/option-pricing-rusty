use crate::OptionPricingModel;

pub struct BinomialModel {
    pub underlying: f64,
    pub strike: f64,
    pub maturity: f64,
    pub volatility: f64,
    pub risk_free_rate: f64,
    pub steps: u32,
}

impl OptionPricingModel for BinomialModel {
    fn price(&self) -> f64 {
        1.0 //TODO
    }
}