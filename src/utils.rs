use num_traits::float;
use std::error::Error;

pub fn normalize_elem<T>(elem: T, min: T, max: T) -> T
where
    T: float::Float,
{
    (elem - min) / (max - min)
}

pub fn denormalize_elem<T>(elem: T, min: T, max: T) -> T
where
    T: float::Float,
{
    (elem * (max - min)) + min
}

pub fn min_max<T>(data: &[T]) -> (T, T)
where
    T: float::Float,
{
    let mut min = T::max_value();
    let mut max = T::min_value();
    for &d in data {
        if d < min {
            min = d;
        } else if d > max {
            max = d;
        }
    }
    (min, max)
}

pub fn normalize_data<T>(data: &mut Vec<T>)
where
    T: float::Float,
{
    let (min, max) = min_max(data);
    for d in data.iter_mut() {
        *d = normalize_elem(*d, min, max);
    }
}

/// Reads `data.csv` and returns (kilometers, prices)
pub fn read_data_csv() -> Result<(Vec<f64>, Vec<f64>), Box<dyn Error>> {
    let mut kilometers = Vec::new();
    let mut prices = Vec::new();

    let mut rdr = csv::Reader::from_path("data.csv")?;
    for record in rdr.records() {
        let record = record?;
        let (km, price): (f64, f64) = record.deserialize(None)?;
        kilometers.push(km);
        prices.push(price);
    }
    Ok((kilometers, prices))
}
