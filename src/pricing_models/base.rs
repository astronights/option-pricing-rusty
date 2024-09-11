use crate::{OptionPricingModel,OptionType};

pub struct BaseModel {
    pub underlying: f64,
    pub strike: f64,
}

impl OptionPricingModel for BaseModel {
    fn price(&self, _option_type: OptionType) -> f64 {
        (self.underlying - self.strike).abs()
    }
}
