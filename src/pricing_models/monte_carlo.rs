use crate::OptionPricingModel;

pub struct MonteCarloModel {
    pub underlying: f64,
    pub strike: f64
}

impl OptionPricingModel for MonteCarloModel {
    fn price(&self) -> f64 {
        1.0 //TODO
    }
}