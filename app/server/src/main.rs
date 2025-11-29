use crate::env::ENV;

pub mod env;

fn main() {
    let env = ENV.mafiascum_username.clone();
    println!("{}", env);
}
