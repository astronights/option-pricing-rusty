use crate::OptionPricingModel;

pub struct BlackScholesModel {
    pub underlying: f64,
    pub strike: f64
}

impl OptionPricingModel for BlackScholesModel {
    fn price(&self) -> f64 {
        1.0 //TODO
    }
}