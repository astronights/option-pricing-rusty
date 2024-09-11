use options_pricing_rusty::{OptionType, OptionPricingModel, BaseModel, BlackScholesModel, BinomialModel, MonteCarloModel};

fn main() {

    let underlying = 120.0;
    let strike = 100.0;
    let maturity = 1.0;
    let volatility = 0.2; 
    let risk_free_rate = 0.05;   
    let steps = 100;
    let simulations = 10000;

    let option_types = [OptionType::Call, OptionType::Put];

    let base_model = BaseModel {
        underlying,
        strike,
    };

    for option in &option_types {
        let price = base_model.price(*option);
        println!("Model: Base, Option Type: {:?}, Price: {:.4}", option, price);
    }

    let black_scholes_model = BlackScholesModel {
        underlying,
        strike,
        maturity,
        volatility,
        risk_free_rate,
    };
    
    for option in &option_types {
        let price = black_scholes_model.price(*option);
        println!("Model: Black Scholaes, Option Type: {:?}, Price: {:.4}", option, price);
    }


    let binomial_model = BinomialModel {
        underlying,
        strike,
        maturity,
        volatility,
        risk_free_rate,
        steps,
    };

    for option in &option_types {
        let price = binomial_model.price(*option);
        println!("Model: Base, Option Type: {:?}, Price: {:.4}", option, price);
    }


    let monte_carlo_model = MonteCarloModel {
        underlying,
        strike,
        maturity,
        volatility,
        risk_free_rate,
        simulations,
        steps,
    };
    
    for option in &option_types {
        let price = monte_carlo_model.price(*option);
        println!("Model: Base, Option Type: {:?}, Price: {:.4}", option, price);
    }
}
