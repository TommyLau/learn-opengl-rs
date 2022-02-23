mod _1_getting_started;

use _1_getting_started::*;

const MAIN_PROGRAM: &str = "1.2.1";

fn main() {
    let mut main = MAIN_PROGRAM;
    let args: Vec<String> = std::env::args().collect();

    if args.len() > 1 {
        main = args[1].as_str();
    }

    match main {
        "1.1.1" => main_1_1_1(),
        "1.1.2" => main_1_1_2(),
        "1.2.1" => main_1_2_1(),
        _ => {}
    }
}
