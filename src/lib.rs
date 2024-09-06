pub trait OptionPricingModel {
    fn price(&self) -> f64;
}

pub mod models {
    pub mod base;
    pub mod black_scholes;
    pub mod binomial;
    pub mod monte_carlo;
}

pub use models::base::BaseModel;
pub use models::black_scholes::BlackScholesModel;
pub use models::binomial::BinomialModel;
pub use models::monte_carlo::MonteCarloModel;