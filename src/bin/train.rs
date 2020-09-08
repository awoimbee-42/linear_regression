use clap::App;
use ft_linear_regression::utils;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let _arg_matches = App::new("train")
        .version("0.0.1")
        .author("Arthur W. <arthur.woimbee@gmail.com>")
        .about("train the prediction algorithm")
        .get_matches();

	let (kms, prices) = utils::read_data()?;

	println!("Hello World");
	Ok(())
}
