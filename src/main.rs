mod two_factor_auth;
mod utils;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        eprintln!("USAGE: 2fa your-secret-key");
        std::process::exit(1);
    }
    let args = args.into_iter().skip(1).collect::<String>();
    let secret = args.replace(' ', "");
    let now = utils::get_current_timestamp();
    let auth = two_factor_auth::TwoFactorAuth::new(&secret, now);
    println!("{} ({} second(s) remaining)", auth.calc(), 30 - now % 30);
}
