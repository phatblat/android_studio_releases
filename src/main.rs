use crate::cli::cli;

mod cli;

fn main() {
    let args = cli().get_matches();
    println!("Hello, world!");
}
