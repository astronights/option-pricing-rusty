pub trait OptionPricingModel {
    fn price(&self) -> f64;
}