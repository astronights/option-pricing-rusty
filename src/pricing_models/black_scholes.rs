use crate::{OptionPricingModel,OptionType};
use distrs::Normal;
use core::f64::consts::E;

#[derive(Clone)]
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
        Normal::cdf(x, 0.0, 1.0)
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
    fn price(&self, option_type: OptionType) -> f64 {
        if option_type == OptionType::Call {
            self.call_price()
        } else {
            self.put_price()
        }
    }

    fn delta(&self, option_type: OptionType) -> f64 {
        let (d1, _) = self.calculate_d1_d2();
        match option_type {
            OptionType::Call => Self::normal_cdf(d1),
            OptionType::Put => Self::normal_cdf(d1) - 1.0,
        }
    }

    fn gamma(&self, _option_type: OptionType) -> f64 {
        let (d1, _) = self.calculate_d1_d2();
        let pdf_d1 = (1.0 / f64::sqrt(2.0 * std::f64::consts::PI)) * f64::exp(-0.5 * d1.powi(2));
        pdf_d1 / (self.underlying * self.volatility * f64::sqrt(self.maturity))
    }

    fn theta(&self, option_type: OptionType) -> f64 {
        let (d1, d2) = self.calculate_d1_d2();
        let pdf_d1 = (1.0 / f64::sqrt(2.0 * std::f64::consts::PI)) * f64::exp(-0.5 * d1.powi(2));
        match option_type {
            OptionType::Call => -((self.underlying * pdf_d1 * self.volatility) / (2.0 * f64::sqrt(self.maturity)))
                - self.risk_free_rate * self.strike * E.powf(-self.risk_free_rate * self.maturity) * Self::normal_cdf(d2),
            OptionType::Put => -((self.underlying * pdf_d1 * self.volatility) / (2.0 * f64::sqrt(self.maturity)))
                + self.risk_free_rate * self.strike * E.powf(-self.risk_free_rate * self.maturity) * Self::normal_cdf(-d2),
        }
    }

    fn vega(&self, _option_type: OptionType) -> f64 {
        let (d1, _) = self.calculate_d1_d2();
        let pdf_d1 = (1.0 / f64::sqrt(2.0 * std::f64::consts::PI)) * f64::exp(-0.5 * d1.powi(2));
        self.underlying * pdf_d1 * f64::sqrt(self.maturity)
    }

    fn rho(&self, option_type: OptionType) -> f64 {
        let epsilon = 0.01; // Small change in the risk-free rate
        let mut model_up = self.clone();
        model_up.risk_free_rate += epsilon; // Increment the risk-free rate
        let price_up = model_up.price(option_type.clone());
        let price = self.price(option_type);

        (price_up - price) / epsilon
    }
}