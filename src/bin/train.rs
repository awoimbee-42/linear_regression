use clap::{App, Arg};
use ft_linear_regression::utils;
use plotters::prelude::*;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let arg_matches = App::new("train")
        .version("0.0.1")
        .author("Arthur W. <arthur.woimbee@gmail.com>")
        .about("train the prediction algorithm")
        .arg(
            Arg::with_name("learning_rate")
                .short("r")
                .default_value("0.5"),
        )
        .arg(
            Arg::with_name("iterations")
                .short("i")
                .default_value("5000"),
        )
        .get_matches();

    let learning_rate = match arg_matches
        .value_of("learning_rate")
        .unwrap()
        .parse::<f64>()
    {
        Ok(lr) if lr != std::f64::INFINITY => lr,
        _ => return Err("Could not parse the learning rate into a valid float".into()),
    };
    let iterations = match arg_matches.value_of("iterations").unwrap().parse::<usize>() {
        Ok(i) if i > 0 => i,
        _ => return Err("Could not parse the iteration number into a valid natural number".into()),
    };

    let mut km_prices = match utils::read_data_csv() {
        Ok(d) => d,
        Err(e) => return Err(format!("Could not read data.csv ({})", e).into()),
    };

    utils::normalize_data(&mut km_prices);
    let (t0, t1) = gradient_descent(&km_prices, learning_rate, iterations);
    if let Err(e) = write_theta_csv(t0, t1) {
        return Err(format!("Could not save thetas.csv ({})", e).into());
    };
    if let Err(e) = draw_chart(&km_prices, t0, t1) {
        return Err(format!("Could not draw the results chart ({})", e).into());
    }

    Ok(())
}

fn gradient_descent(
    miles_n_prices: &[(f64, f64)],
    mut learning_rate: f64,
    iterations: usize,
) -> (f64, f64) {
    let mut t0 = 0.;
    let mut t1 = 0.;
    let mut old_loss = std::f64::MAX;
    for i in 0..iterations {
        let (dt0, dt1) = {
            let (mut dt0, mut dt1) = (0., 0.);
            for (m, p) in miles_n_prices.iter() {
                dt0 += (t1 * m + t0) - p;
                dt1 += ((t1 * m + t0) - p) * m;
            }
            (dt0, dt1)
        };
        let new_t0 = t0 - (dt0 / miles_n_prices.len() as f64 * learning_rate);
        let new_t1 = t1 - (dt1 / miles_n_prices.len() as f64 * learning_rate);
        // Bold driver:
        let loss = loss(new_t0, new_t1, miles_n_prices);
        if i % (iterations / 10).max(1) == 0 || i == iterations - 1 {
            println!(
                "epoch {} - loss: {:.8} - learning rate: {}",
                i, loss, learning_rate
            );
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

fn loss(t0: f64, t1: f64, miles_n_prices: &[(f64, f64)]) -> f64 {
    let mut loss = 0.;
    for (m, p) in miles_n_prices.iter() {
        loss += (p - (t0 + m * t1)).powi(2);
    }
    loss / miles_n_prices.len() as f64
}

fn write_theta_csv(t0: f64, t1: f64) -> Result<(), Box<dyn Error>> {
    let mut wtr = csv::Writer::from_path("thetas.csv")?;
    wtr.write_record(&[t0.to_string(), t1.to_string()])?;
    wtr.flush()?;
    Ok(())
}

fn draw_chart(km_prices: &[(f64, f64)], t0: f64, t1: f64) -> Result<(), Box<dyn Error>> {
    let root = SVGBackend::new("training_results.svg", (800, 800)).into_drawing_area();
    root.fill(&WHITE)?;
    let mut chart = ChartBuilder::on(&root)
        .caption("Training results", ("sans-serif", 30).into_font())
        .margin(5)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(-0.1..1.1, -0.1..1.1)?;

    chart
        .configure_mesh()
        .x_desc("mileage")
        .y_desc("price")
        .draw()?;

    chart
        .draw_series(LineSeries::new(
            (0..=1000)
                .map(|x| x as f64 / 1000.)
                .map(|x| (x, t0 + x * t1)),
            &RED,
        ))?
        .label("price prediction")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RED));

    chart
        .draw_series(PointSeries::of_element(
            km_prices,
            5,
            &BLACK,
            &|&c, s, st| {
                let (cx, cy) = c;
                EmptyElement::at(c)
                    + Circle::new((0, 0), s, st.filled())
                    + Text::new(
                        format!("{:.2}, {:.2}", cx, cy),
                        (10, 0),
                        ("sans-serif", 10).into_font(),
                    )
            },
        ))?
        .label("actual prices")
        .legend(|(x, y)| Circle::new((x + 10, y), 4, BLACK.filled()));

    chart
        .configure_series_labels()
        .background_style(&WHITE.mix(0.8))
        .border_style(&BLACK)
        .draw()?;
    println!("Chart generated, check ./training_results.svg !");
    Ok(())
}
