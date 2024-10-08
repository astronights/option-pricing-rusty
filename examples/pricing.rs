use options_pricing_rusty::{OptionType, OptionPricingModel, BaseModel, BlackScholesModel, BinomialModel, MonteCarloModel};

fn main() {

    let underlying = 120.0;
    let strike = 100.0;
    let maturity = 1.0;
    let volatility = 0.2; 
    let risk_free_rate = 0.05;   
    let steps = 252;
    let simulations = 10000;

    let option_types = [OptionType::Call, OptionType::Put];

    let header = "=".repeat(50);
    let mid = "-".repeat(50);

    print!("\n{}\nEnvironment\n{}\n", header, mid);
    println!("Underlying Price: {:.4}\nStrike Price: {:.4}\nTime to Maturity: {:.4}\nVolatility: {:.4}\nRisk Free Rate: {:.4}",
           underlying, strike, maturity, volatility, risk_free_rate);

    let base_model = BaseModel {
        underlying,
        strike,
    };

    print!("\n{}\nModel: Base\n{}\n", header, mid);

    for option in &option_types {
        let price = base_model.price(*option);
        println!("Option: {:?}, Price: {:.4}", option, price);
    }

    let black_scholes_model = BlackScholesModel {
        underlying,
        strike,
        maturity,
        volatility,
        risk_free_rate,
    };

    print!("\n{}\nModel: Black Scholes\n{}\n", header, mid);
    
    for option in &option_types {
        let price = black_scholes_model.price(*option);
        println!("Option: {:?}, Price: {:.4}", option, price);
    }


    let binomial_model = BinomialModel {
        underlying,
        strike,
        maturity,
        volatility,
        risk_free_rate,
        steps,
    };

    print!("\n{}\nModel: Binomial\n{}\n", header, mid);

    for option in &option_types {
        let price = binomial_model.price(*option);
        println!("Option: {:?}, Price: {:.4}", option, price);
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

    print!("\n{}\nModel: Monte Carlo\nSteps: {}\nSimlulations: {}\n{}\n", header, steps, simulations, mid);
    
    for option in &option_types {
        let price = monte_carlo_model.price(*option);
        println!("Option: {:?}, Price: {:.4}", option, price);
    }
}
