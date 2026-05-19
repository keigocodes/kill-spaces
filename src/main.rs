mod helpers;

use std::env;

// Local directory added by default
const DEFAULT_DIRS: &[&str] = &["."];

fn main() {
    let mut dirs: Vec<String> = DEFAULT_DIRS.iter().map(|s| s.to_string()).collect();
    dirs.extend(env::args().skip(1));

    helpers::run_kill_spaces(&dirs);
}
