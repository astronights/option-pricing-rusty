use options_pricing_rusty::{OptionType, OptionPricingModel, BaseModel, BlackScholesModel, BinomialModel, MonteCarloModel};

fn main() {

    let underlying = 120.0;
    let strike = 100.0;
    let maturity = 1.0;
    let volatility = 0.2; 
    let risk_free_rate = 0.05;   
    let steps = 252;
    let simulations = 50000;

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
        print!("Option: {:?}", option);

        let delta = base_model.delta(*option);
        let gamma = base_model.gamma(*option);
        let theta = base_model.theta(*option);
        let vega = base_model.vega(*option);
        let rho = base_model.rho(*option);

        println!("\n  Delta: {:.4}\n  Gamma: {:.4}\n  Theta: {:.4}\n  Vega: {:.4}\n  Rho: {:.4}", 
                delta, gamma, theta, vega, rho);
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
        print!("Option: {:?}", option);

        let delta = black_scholes_model.delta(*option);
        let gamma = black_scholes_model.gamma(*option);
        let theta = black_scholes_model.theta(*option);
        let vega = black_scholes_model.vega(*option);
        let rho = black_scholes_model.rho(*option);

        println!("\n  Delta: {:.4}\n  Gamma: {:.4}\n  Theta: {:.4}\n  Vega: {:.4}\n  Rho: {:.4}", 
                delta, gamma, theta, vega, rho);
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
        print!("Option: {:?}", option);

        let delta = binomial_model.delta(*option);
        let gamma = binomial_model.gamma(*option);
        let theta = binomial_model.theta(*option);
        let vega = binomial_model.vega(*option);
        let rho = binomial_model.rho(*option);

        println!("\n  Delta: {:.4}\n  Gamma: {:.4}\n  Theta: {:.4}\n  Vega: {:.4}\n  Rho: {:.4}", 
                delta, gamma, theta, vega, rho);
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
        print!("Option: {:?}", option);

        let delta = monte_carlo_model.delta(*option);
        let gamma = monte_carlo_model.gamma(*option);
        let theta = monte_carlo_model.theta(*option);
        let vega = monte_carlo_model.vega(*option);
        let rho = monte_carlo_model.rho(*option);

        println!("\n  Delta: {:.4}\n  Gamma: {:.4}\n  Theta: {:.4}\n  Vega: {:.4}\n  Rho: {:.4}", 
                delta, gamma, theta, vega, rho);
    }
}
