# Option Pricing with Rust

This repository consists of several options pricing methodologies coded with Rust.

## Pricing Models

The pricing models implemented are as follows:

- Base Model
- Binomial Model
- Black Scholes Model
- Monte Carlo Model

## Option Greeks

The following Option Greeks are implemented:

- Delta $(\delta)$
- Gamma $(\gamma)$
- Theta $(\theta)$
- Vega $(\nu)$
- Rho $(\rho)$

## How to Run

### Main Function

In order to run the code, you would have to compile it with Cargo first.

```bash
cargo build
```

The main can be run as 

```bash
cargo run
```

### Examples

In order to run the examples, you would have to compile it with Cargo first.

```bash
cargo build --examples
```

The pricing examples can be run as

```bash
cargo run --example pricing
```

The greeks examples can be run as

```bash
cargo run --example greeks
```

