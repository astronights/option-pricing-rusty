use crate::OptionPricingModel;
use statrs::distribution::{Normal};
use std::f64::consts::E;

pub struct BlackScholesModel {
    pub underlying: f64,
    pub strike: f64,
    pub maturity: f64,
    pub volatility: f64,
    pub risk_free_rate: f64
}

impl BlackScholesModel {

    fn calculate_d1_d2(&self) -> (f64, f64) {
        let d1 = (f64::ln(self.underlying / self.strike)
            + (self.risk_free_rate + 0.5 * self.volatility.powi(2)) * self.maturity)
            / (self.volatility * f64::sqrt(self.maturity));

        let d2 = d1 - self.volatility * f64::sqrt(self.maturity);

        (d1, d2)
    }

    fn normal_cdf(x: f64) -> f64 {
        (1.0 + (x / f64::sqrt(2.0)).erf()) / 2.0
    }

    fn call_price(&self) -> f64 {
        let (d1, d2) = self.calculate_d1_d2();
        let nd1 = Self::normal_cdf(d1);
        let nd2 = Self::normal_cdf(d2);

        self.underlying * nd1 - self.strike * E.powf(-self.risk_free_rate * self.maturity) * nd2
    }

    fn put_price(&self) -> f64 {
        let (d1, d2) = self.calculate_d1_d2();
        let nd1 = Self::normal_cdf(-d1);
        let nd2 = Self::normal_cdf(-d2);

        self.strike * E.powf(-self.risk_free_rate * self.maturity) * nd2 - self.underlying * nd1
    }

}

impl OptionPricingModel for BlackScholesModel {

    fn price(&self) -> f64 {
        self.call_price()
    }
}