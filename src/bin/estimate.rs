use clap::{App, Arg};
use std::error::Error;
use ft_linear_regression::utils;

fn read_thetas() -> (f64, f64) {
    fn really_read_thetas() -> Result<(f64, f64), Box<dyn Error>> {
        let mut rdr = csv::Reader::from_path("thetas.csv")?;
        let (t0, t1) = rdr.headers()?.deserialize(None)?;
        Ok((t0, t1))
    }
    match really_read_thetas() {
        Ok(t) => t,
        Err(e) => {
            println!("Could not read thetas.csv, using default values ({})", e);
            (0., 0.)
        }
    }
}

fn estimate(t0: f64, t1: f64, mileage: f64) -> f64 {
    t0 + mileage * t1
}

fn main() -> Result<(), Box<dyn Error>> {
    let arg_matches = App::new("estimate")
        .version("0.0.1")
        .author("Arthur W. <arthur.woimbee@gmail.com>")
        .about("estimate the price of a car using the prediction algorithm")
        .arg(Arg::with_name("mileage (in km)").required(true))
        .get_matches();

    let mileage = match arg_matches.value_of("mileage (in km)").unwrap().parse::<usize>() {
        Ok(i) => i as f64,
        _ => return Err("Could not parse the mileage value into a valid integer".into()),
    };

    let (t0, t1) = read_thetas();

    let km_prices = match utils::read_data_csv() {
        Ok(d) => d,
        Err(e) => return Err(format!("Could not read data.csv ({})", e).into()),
    };
    let (km_range, prices_range) = utils::min_max(&km_prices);

    let norm_mileage = utils::normalize_elem(mileage, km_range.0, km_range.1);
    let norm_estimation = estimate(t0, t1, norm_mileage);
    let estimation = utils::denormalize_elem(norm_estimation, prices_range.0, prices_range.1);
    println!("This car has done {}km, its estimated value is {}", mileage, estimation);
    Ok(())
}
