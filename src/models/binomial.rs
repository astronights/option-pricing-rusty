use crate::OptionPricingModel;

pub struct BinomialModel {
    pub underlying: f64,
    pub strike: f64
}

impl OptionPricingModel for BinomialModel {
    fn price(&self) -> f64 {
        1.0 //TODO
    }
}