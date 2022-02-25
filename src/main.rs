mod shader;
mod _1_getting_started;

use _1_getting_started::*;

const MAIN_PROGRAM: &str = "1.3.3";

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
        "1.2.2" => main_1_2_2(),
        "1.2.3" => main_1_2_3(),
        "1.2.4" => main_1_2_4(),
        "1.2.5" => main_1_2_5(),
        "1.3.1" => main_1_3_1(),
        "1.3.2" => main_1_3_2(),
        "1.3.3" => main_1_3_3(),
        _ => {}
    }
}
