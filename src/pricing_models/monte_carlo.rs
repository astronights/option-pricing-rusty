use crate::{OptionPricingModel, OptionType};
use rand_distr::{Normal, Distribution};
use rand::Rng;

#[derive(Clone)]
pub struct MonteCarloModel {
    pub underlying: f64,
    pub strike: f64,
    pub maturity: f64,
    pub volatility: f64,
    pub risk_free_rate: f64,
    pub simulations: u32,
    pub steps: u32,
}

impl MonteCarloModel {

    fn cap_value(&self, value: f64, min: f64, max: f64) -> f64 {
        value.clamp(min, max)
    }

    // Function to simulate a single path of the underlying asset price
    fn simulate_path(&self, rng: &mut impl Rng) -> f64 {
        let dt = self.maturity / self.steps as f64;
        let normal_dist = Normal::new(0.0, 1.0).unwrap();

        // Start at the current underlying price
        let mut price = self.underlying;

        // Simulate the price evolution over each time step
        for _ in 0..self.steps {
            let z: f64 = normal_dist.sample(rng);
            price *= f64::exp((self.risk_free_rate - 0.5 * self.volatility.powi(2)) * dt
                + self.volatility * f64::sqrt(dt) * z);
        }

        price
    }

    // Function to calculate the option payoff for a call option
    fn call_payoff(&self, final_price: f64) -> f64 {
        f64::max(0.0, final_price - self.strike)
    }

    // Function to calculate the option payoff for a put option
    fn put_payoff(&self, final_price: f64) -> f64 {
        f64::max(0.0, self.strike - final_price)
    }

    // Function to calculate the discounted average payoff across multiple simulations
    fn calculate_price<F>(&self, payoff_func: F, rng: &mut impl Rng) -> f64
    where
        F: Fn(f64) -> f64,
    {
        let mut total_payoff = 0.0;

        for _ in 0..self.simulations {
            let final_price = self.simulate_path(rng);
            total_payoff += payoff_func(final_price);
        }

        let average_payoff = total_payoff / self.simulations as f64;
        average_payoff * f64::exp(-self.risk_free_rate * self.maturity)
    }
}

// Implement the OptionPricingModel trait for MonteCarloModel
impl OptionPricingModel for MonteCarloModel {
    fn price(&self, option_type: OptionType) -> f64 {
        let mut rng = rand::thread_rng();
        if option_type == OptionType::Call {
            self.calculate_price(|final_price| self.call_payoff(final_price), &mut rng)
        } else {
            self.calculate_price(|final_price| self.put_payoff(final_price), &mut rng)
        }
    }

    fn delta(&self, option_type: OptionType) -> f64 {
        let epsilon = 0.01; // Epsilon value for finite difference
        let mut rng = rand::thread_rng();

        let price = self.price(option_type.clone());
        let mut model_up = self.clone();
        model_up.underlying += epsilon;
        let price_up = model_up.calculate_price(|final_price| {
            if option_type == OptionType::Call {
                model_up.call_payoff(final_price)
            } else {
                model_up.put_payoff(final_price)
            }
        }, &mut rng);

        let delta = (price_up - price) / epsilon;

        self.cap_value(delta, -2.0, 2.0)
    }

    fn gamma(&self, option_type: OptionType) -> f64 {
        let epsilon = 0.01;
        let mut rng = rand::thread_rng();

        let price = self.price(option_type.clone());
        let mut model_up = self.clone();
        model_up.underlying += epsilon;
        let price_up = model_up.calculate_price(|final_price| {
            if option_type == OptionType::Call {
                model_up.call_payoff(final_price)
            } else {
                model_up.put_payoff(final_price)
            }
        }, &mut rng);

        let mut model_down = self.clone();
        model_down.underlying -= epsilon;
        let price_down = model_down.calculate_price(|final_price| {
            if option_type == OptionType::Call {
                model_down.call_payoff(final_price)
            } else {
                model_down.put_payoff(final_price)
            }
        }, &mut rng);

        let gamma = (price_up - 2.0 * price + price_down) / (epsilon * epsilon);

        self.cap_value(gamma, -5.0, 5.0)
    }

    fn theta(&self, option_type: OptionType) -> f64 {
        let epsilon = 0.01;
        let mut rng = rand::thread_rng();

        let price = self.price(option_type.clone());
        let mut model_up = self.clone();
        model_up.maturity -= epsilon; // Adjusting maturity to simulate theta
        let price_up = model_up.calculate_price(|final_price| {
            if option_type == OptionType::Call {
                model_up.call_payoff(final_price)
            } else {
                model_up.put_payoff(final_price)
            }
        }, &mut rng);

        let theta = (price_up - price) / epsilon;

        self.cap_value(theta, -10.0, 10.0)
    }

    fn vega(&self, option_type: OptionType) -> f64 {
        let epsilon = 0.01;
        let mut rng = rand::thread_rng();

        let price = self.price(option_type.clone());
        let mut model_up = self.clone();
        model_up.volatility += epsilon;
        let price_up = model_up.calculate_price(|final_price| {
            if option_type == OptionType::Call {
                model_up.call_payoff(final_price)
            } else {
                model_up.put_payoff(final_price)
            }
        }, &mut rng);

        let vega = (price_up - price) / epsilon;

        self.cap_value(vega, -50.0, 50.0)
    }

    fn rho(&self, option_type: OptionType) -> f64 {
        let epsilon = 0.01; // Small change in the risk-free rate
        let mut model_up = self.clone();
        model_up.risk_free_rate += epsilon; // Increment the risk-free rate
        let price_up = model_up.price(option_type.clone());
        let price = self.price(option_type);

        let rho = (price_up - price) / epsilon;

        self.cap_value(rho, -100.0, 100.0)
    }
}
