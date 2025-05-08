mod two_factor_auth;
mod utils;

use std::{
    env,
    io::{self, Write},
    thread::sleep,
    time::Duration,
};

use utils::clear_current_line;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        eprintln!("USAGE: 2fa your-secret-key [--no-loop]");
        std::process::exit(1);
    }
    let mut no_loop = false;
    if args.iter().skip_while(|x| *x != "--no-loop").count() > 0 {
        no_loop = true;
    }
    let secret_key = args.iter().skip(1).take(1).last().unwrap();
    loop {
        let now = utils::get_current_timestamp();
        let auth = two_factor_auth::TwoFactorAuth::new(secret_key, now);
        let auth_next = two_factor_auth::TwoFactorAuth::new(secret_key, now + 30 - now % 30);
        clear_current_line();
        print!(
            "\x1b[1;46m {:06} \x1b[0m ({:02} second(s) remaining) ==> \x1b[1;41m {:06} \x1b[0m",
            auth.calc(),
            30 - now % 30,
            auth_next.calc(),
        );
        io::stdout().flush().unwrap();
        if no_loop {
            break;
        } else {
            sleep(Duration::from_secs(1));
        }
    }
}
