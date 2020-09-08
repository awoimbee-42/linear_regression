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

pub fn min_max<T>(data: &[(T, T)]) -> ((T, T), (T, T))
where
    T: float::Float,
{
    let mut min0 = T::max_value();
    let mut min1 = T::max_value();
    let mut max0 = T::min_value();
    let mut max1 = T::min_value();

    for (d0, d1) in data.iter() {
        if *d0 < min0 {
            min0 = *d0;
        }
        if *d0 > max0 {
            max0 = *d0;
        }
        if *d1 < min1 {
            min1 = *d1;
        }
        if *d1 > max1 {
            max1 = *d1;
        }
    }
    ((min0, max0), (min1, max1))
}

pub fn normalize_data<T>(data: &mut Vec<(T, T)>)
where
    T: float::Float,
{
    let ((min0, max0), (min1, max1)) = min_max(&data);
    for (d0, d1) in data.iter_mut() {
        *d0 = normalize_elem(*d0, min0, max0);
        *d1 = normalize_elem(*d1, min1, max1);
    }
}

/// Reads `data.csv` and returns (kilometers, prices)
pub fn read_data_csv() -> Result<Vec<(f64, f64)>, Box<dyn Error>> {
    let mut km_prices = Vec::new();

    let mut rdr = csv::Reader::from_path("data.csv")?;
    for record in rdr.records() {
        let record = record?;
        let km_price: (f64, f64) = record.deserialize(None)?;
        km_prices.push(km_price);
    }
    Ok(km_prices)
}
