use crate::{OptionPricingModel,OptionType};
use rand_distr::{Normal, Distribution};
use rand::Rng; 

pub struct MonteCarloModel {
    pub underlying: f64,
    pub strike: f64,
    pub maturity: f64,
    pub volatility: f64,
    pub risk_free_rate: f64, 
    pub option_type: OptionType,
    pub simulations: u32, 
    pub steps: u32,
}

impl MonteCarloModel {
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
    fn calculate_price<F>(&self, payoff_func: F) -> f64
    where
        F: Fn(f64) -> f64,
    {
        let mut rng = rand::thread_rng();
        let mut total_payoff = 0.0;

        for _ in 0..self.simulations {
            let final_price = self.simulate_path(&mut rng);
            total_payoff += payoff_func(final_price);
        }

        let average_payoff = total_payoff / self.simulations as f64;
        average_payoff * f64::exp(-self.risk_free_rate * self.maturity)
    }
}

// Implement the OptionPricingModel trait for MonteCarloModel
impl OptionPricingModel for MonteCarloModel {
    fn price(&self) -> f64 {
        if self.option_type == OptionType::Call {
            self.calculate_price(|final_price| self.call_payoff(final_price))
        } else {
            self.calculate_price(|final_price| self.put_payoff(final_price))
        }
        
    }
}

