#[derive(PartialEq,Debug,Clone,Copy)]
pub enum OptionType {
    Call,
    Put,
}

pub trait OptionPricingModel {
    fn price(&self, option_type: OptionType) -> f64;
}

pub mod pricing_models {
    pub mod base;
    pub mod black_scholes;
    pub mod binomial;
    pub mod monte_carlo;
}

pub use pricing_models::base::BaseModel;
pub use pricing_models::black_scholes::BlackScholesModel;
pub use pricing_models::binomial::BinomialModel;
pub use pricing_models::monte_carlo::MonteCarloModel;