use crate::{OptionPricingModel,OptionType};

pub struct BaseModel {
    pub underlying: f64,
    pub strike: f64,
    pub option_type: OptionType,
}

impl OptionPricingModel for BaseModel {
    fn price(&self) -> f64 {
        (self.underlying - self.strike).abs()
    }
}
