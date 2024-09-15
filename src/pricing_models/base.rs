use crate::{OptionPricingModel,OptionType};

#[derive(Clone)]
pub struct BaseModel {
    pub underlying: f64,
    pub strike: f64,
}

impl OptionPricingModel for BaseModel {
    fn price(&self, option_type: OptionType) -> f64 {
        let is_itm = match option_type {
            OptionType::Call => self.underlying > self.strike,
            OptionType::Put => self.underlying < self.strike,
        };

        if is_itm {
            (self.underlying - self.strike).abs()
        } else {
            0.01 * (self.underlying - self.strike).abs()
        }
    }

    fn delta(&self, option_type: OptionType) -> f64 {
        let epsilon = 0.01;
        let price_up = self.price(option_type.clone());

        let mut model_up = self.clone();
        model_up.underlying += epsilon;
        let price_up_epsilon = model_up.price(option_type.clone());

        (price_up_epsilon - price_up) / epsilon
    }

    fn gamma(&self, option_type: OptionType) -> f64 {
        let epsilon = 0.01;
        let price = self.price(option_type.clone());

        let mut model_up = self.clone();
        model_up.underlying += epsilon;
        let price_up = model_up.price(option_type.clone());

        let mut model_down = self.clone();
        model_down.underlying -= epsilon;
        let price_down = model_down.price(option_type.clone());

        (price_up - 2.0 * price + price_down) / (epsilon * epsilon)
    }

    fn theta(&self, _option_type: OptionType) -> f64 {
        print!(" (No time to maturity) ");
        0.0
    }

    fn vega(&self, _option_type: OptionType) -> f64 {
        print!(" (No volatility) ");
        0.0
    }
}
