use super::utils;
use std::error::Error;

pub struct Estimator {
    t0: f64,
    t1: f64,
    km_range: (f64, f64),
    prices_range: (f64, f64),
}

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

impl Estimator {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        let (t0, t1) = read_thetas();

        let km_prices = match utils::read_data_csv() {
            Ok(d) => d,
            Err(e) => return Err(format!("Could not read data.csv ({})", e).into()),
        };
        let (km_range, prices_range) = utils::min_max(&km_prices);
        Ok(Self {
            t0,
            t1,
            km_range,
            prices_range,
        })
    }

    pub fn estimate(&self, mileage: f64) -> f64 {
        let norm_mileage = utils::normalize_elem(mileage, self.km_range.0, self.km_range.1);
        let norm_estimation = estimate(self.t0, self.t1, norm_mileage);
        utils::denormalize_elem(norm_estimation, self.prices_range.0, self.prices_range.1)
    }

    pub fn min_price(&self) -> f64 {
        self.prices_range.0
    }

    pub fn max_price(&self) -> f64 {
        self.prices_range.1
    }
}
