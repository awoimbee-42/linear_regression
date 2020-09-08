use clap::App;
use ft_linear_regression::utils;
use std::error::Error;

fn gradient_descent(mileages: Vec<f64>, prices: Vec<f64>, mut learning_rate: f64, iterations: usize) -> (f64, f64) {
	let mut t0 = 0.;
	let mut t1 = 0.;
	let mut old_loss = std::f64::MAX;
	for i in 0..iterations {
		let (dt0, dt1) = {
			let (mut dt0, mut dt1) = (0., 0.);
			for (&m, &p) in mileages.iter().zip(prices.iter()) {
				dt0 += (t1 * m + t0) - p;
				dt1 += ((t1 * m + t0) - p) * m;
			}
			(dt0, dt1)
		};
		let new_t0 = t0 - (dt0 / mileages.len() as f64 * learning_rate);
		let new_t1 = t1 - (dt1 / mileages.len() as f64 * learning_rate);
		// Bold driver:
		let loss = loss(new_t0, new_t1, &mileages, &prices);
		if i % (iterations / 10) == 0 {
			println!("epoch {} - loss: {:.8} - learning rate: {}", i, loss, learning_rate);
		}
		if loss > old_loss {
			learning_rate *= 0.5;
			continue;
		}
		learning_rate *= 1.05;
		old_loss = loss;
		t0 = new_t0;
		t1 = new_t1;
	}
	(t0, t1)
}

fn loss(t0: f64, t1: f64, mileages: &[f64], prices: &[f64]) -> f64 {
	let mut loss = 0.;
	for (&m, &p) in mileages.iter().zip(prices.iter()) {
		loss += (p - (t0 + m * t1)).powi(2);
	}
	loss / mileages.len() as f64
}

fn write_theta_csv(t0: f64, t1: f64) -> Result<(), Box<dyn Error>> {
	let mut wtr = csv::Writer::from_path("thetas.csv")?;
	wtr.write_record(&[t0.to_string(), t1.to_string()])?;
	wtr.flush()?;
	Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let _arg_matches = App::new("train")
        .version("0.0.1")
        .author("Arthur W. <arthur.woimbee@gmail.com>")
        .about("train the prediction algorithm")
        .get_matches();

	let learning_rate = 0.5;
	let iterations = 5000;

	let (mut mileages, mut prices) = utils::read_data_csv()?;
	utils::normalize_data(&mut mileages);
	utils::normalize_data(&mut prices);
	let (t0, t1) = gradient_descent(mileages, prices, learning_rate, iterations);
	write_theta_csv(t0, t1)?;
	Ok(())
}
