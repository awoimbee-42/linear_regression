use clap::App;
use ft_linear_regression::estimator;
use ft_linear_regression::utils;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let _arg_matches = App::new("estimate")
        .version("0.0.1")
        .author("Arthur W. <arthur.woimbee@gmail.com>")
        .about("Calculate the accuracy of the estimator")
        .get_matches();

    let known_data = utils::read_data_csv()?;
    let estimator = estimator::Estimator::new()?;

    let mut prices = Vec::new();
    let mut average_price = 0.;
    let mut estimated_prices = Vec::new();

    for (mileage, price) in known_data.iter() {
        let estimated_price = estimator.estimate(*mileage);
        prices.push(price);
        estimated_prices.push(estimated_price);
        average_price += price;
    }
    average_price /= known_data.len() as f64;

    let mean_absolute_err: f64 = prices
        .iter()
        .zip(estimated_prices.iter())
        .map(|(p, e)| (e - **p).abs())
        .sum::<f64>()
        / prices.len() as f64;
    let root_mean_sqr_err: f64 = (prices
        .iter()
        .zip(estimated_prices.iter())
        .map(|(p, e)| (e - **p).powi(2))
        .sum::<f64>()
        / prices.len() as f64)
        .sqrt();
    let relative_abs_err: f64 = prices
        .iter()
        .zip(estimated_prices.iter())
        .map(|(p, e)| (e - **p).abs())
        .sum::<f64>()
        / prices
            .iter()
            .map(|&&p| (average_price - p).abs())
            .sum::<f64>();
    let root_rel_sqr_err: f64 = (prices
        .iter()
        .zip(estimated_prices.iter())
        .map(|(p, e)| (e - **p).powi(2))
        .sum::<f64>()
        / prices
            .iter()
            .map(|&&p| (average_price - p).powi(2))
            .sum::<f64>())
    .sqrt();

    println!(
        "mean absolute error (MAE):                  {:.4}",
        mean_absolute_err
    );
    println!(
        "normalized mean absolute error (nMAE):      {:.4}",
        mean_absolute_err / (estimator.max_price() - estimator.min_price())
    );
    println!(
        "root mean squared error (RMSE):             {:.4}",
        root_mean_sqr_err
    );
    println!(
        "normalized root mean squared error (nRMSE): {:.4}",
        root_mean_sqr_err / (estimator.max_price() - estimator.min_price())
    );
    println!(
        "relative absolute error (RAE):              {:.4}",
        relative_abs_err
    );
    println!(
        "root relative squared error (RRSE):         {:.4}",
        root_rel_sqr_err
    );

    Ok(())
}
