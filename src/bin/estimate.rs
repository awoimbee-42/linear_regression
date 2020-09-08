use clap::App;

fn main() {
    let _arg_matches = App::new("estimate")
        .version("0.0.1")
        .author("Arthur W. <arthur.woimbee@gmail.com>")
        .about("estimate the price of a car using the prediction algorithm")
        .get_matches();

    println!("Hello World");
}
