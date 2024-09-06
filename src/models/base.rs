use crate::OptionPricingModel;

pub struct BaseModel {
    pub underlying: f64;
    pub strike: f64;
}

impl OptionPricingModel for BaseModel {
    fn price(&self) -> f64 {
        (self.underlying - self.strike).abs()
    }
}