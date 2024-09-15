use crate::{OptionPricingModel, OptionType};

#[derive(Clone)]
pub struct BinomialModel {
    pub underlying: f64,
    pub strike: f64,
    pub maturity: f64,
    pub volatility: f64,
    pub risk_free_rate: f64,
    pub steps: u32,
}

impl BinomialModel {
    fn option_payoff(&self, asset_price: f64, option_type: OptionType) -> f64 {
        match option_type {
            OptionType::Call => f64::max(0.0, asset_price - self.strike),
            OptionType::Put => f64::max(0.0, self.strike - asset_price),
        }
    }
}

impl OptionPricingModel for BinomialModel {
    fn price(&self, option_type: OptionType) -> f64 {
        let dt = self.maturity / self.steps as f64; // Δt: Time step size
        let u = f64::exp(self.volatility * f64::sqrt(dt)); // Up factor: u = e^(σ√Δt)
        let d = 1.0 / u; // Down factor: d = 1 / u
        let p = (f64::exp(self.risk_free_rate * dt) - d) / (u - d); // Risk-neutral probability

        // Vector to store option values at each node
        let mut option_values: Vec<f64> = vec![0.0; (self.steps + 1) as usize];

        // Compute option values at maturity (step N)
        for i in 0..=self.steps {
            // Price of the underlying asset at node (N, i) is S * u^i * d^(N-i)
            let asset_price_at_maturity = self.underlying * u.powi(i as i32) * d.powi((self.steps - i) as i32);
            option_values[i as usize] = self.option_payoff(asset_price_at_maturity, option_type.clone());
        }

        // Traverse backward through the tree, starting from the last time step
        for step in (0..self.steps).rev() {
            for i in 0..=step {
                // Option value at node (step, i) is the discounted value at the next step
                option_values[i as usize] = (p * option_values[(i + 1) as usize] 
                                            + (1.0 - p) * option_values[i as usize])
                    * f64::exp(-self.risk_free_rate * dt);
            }
        }

        option_values[0]
    }

    fn delta(&self, option_type: OptionType) -> f64 {
        let epsilon = 0.01;
        let _price = self.price(option_type.clone());

        let mut model_up = self.clone();
        model_up.underlying += epsilon;
        let price_up = model_up.price(option_type.clone());

        let mut model_down = self.clone();
        model_down.underlying -= epsilon;
        let price_down = model_down.price(option_type.clone());

        (price_up - price_down) / (2.0 * epsilon)
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

    fn theta(&self, option_type: OptionType) -> f64 {
        let epsilon = 0.01;
        let price = self.price(option_type.clone());

        let mut model_up = self.clone();
        model_up.maturity -= epsilon;
        let price_up = model_up.price(option_type.clone());

        (price - price_up) / epsilon
    }

    fn vega(&self, option_type: OptionType) -> f64 {
        let epsilon = 0.01;
        let price = self.price(option_type.clone());

        let mut model_up = self.clone();
        model_up.volatility += epsilon;
        let price_up = model_up.price(option_type.clone());

        (price_up - price) / epsilon
    }
}
