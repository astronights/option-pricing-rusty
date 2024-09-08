use crate::OptionPricingModel;
use std::f64::consts::E;

pub struct BlackScholesModel {
    pub underlying: f64,
    pub strike: f64,
    pub maturity: f64,
    pub volatility: f64,
    pub risk_free_rate: f64,
}

impl BlackScholesModel {
    // Calculate d1 and d2 for the Black-Scholes formula
    fn calculate_d1_d2(&self) -> (f64, f64) {
        let d1 = (f64::ln(self.underlying / self.strike)
            + (self.risk_free_rate + 0.5 * self.volatility.powi(2)) * self.maturity)
            / (self.volatility * f64::sqrt(self.maturity));
        let d2 = d1 - self.volatility * f64::sqrt(self.maturity);
        (d1, d2)
    }

    // Approximate the normal CDF using erf
    fn normal_cdf(x: f64) -> f64 {
        (1.0 + (x / f64::sqrt(2.0)).erf()) / 2.0
    }

    // Call option price calculation
    fn call_price(&self) -> f64 {
        let (d1, d2) = self.calculate_d1_d2();
        let nd1 = Self::normal_cdf(d1);
        let nd2 = Self::normal_cdf(d2);

        // Call option price: S * N(d1) - K * e^(-rT) * N(d2)
        self.underlying * nd1 - self.strike * f64::exp(-self.risk_free_rate * self.maturity) * nd2
    }

    // Put option price calculation
    fn put_price(&self) -> f64 {
        let (d1, d2) = self.calculate_d1_d2();
        let nd1 = Self::normal_cdf(-d1);
        let nd2 = Self::normal_cdf(-d2);

        // Put option price: K * e^(-rT) * N(-d2) - S * N(-d1)
        self.strike * f64::exp(-self.risk_free_rate * self.maturity) * nd2 - self.underlying * nd1
    }
}

// Implement the OptionPricingModel trait for BlackScholesModel
impl OptionPricingModel for BlackScholesModel {
    fn price(&self) -> f64 {
        self.call_price()
    }
}