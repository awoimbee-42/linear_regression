use clap::{App, Arg};
use ft_linear_regression::estimator;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let arg_matches = App::new("estimate")
        .version("0.0.1")
        .author("Arthur W. <arthur.woimbee@gmail.com>")
        .about("estimate the price of a car using the prediction algorithm")
        .arg(Arg::with_name("mileage (in km)").required(true))
        .get_matches();

    let mileage = match arg_matches
        .value_of("mileage (in km)")
        .unwrap()
        .parse::<usize>()
    {
        Ok(i) => i as f64,
        _ => return Err("Could not parse the mileage value into a valid integer".into()),
    };

    let estimator = estimator::Estimator::new()?;
    let estimated_price = estimator.estimate(mileage);

    println!(
        "This car has done {}km, its estimated value is {}",
        mileage, estimated_price
    );
    Ok(())
}
