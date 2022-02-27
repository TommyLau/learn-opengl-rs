mod shader;
mod _1_getting_started;

use _1_getting_started::*;

const MAIN_PROGRAM: &str = "1.5.3";

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
        "1.3.4" => main_1_3_4(),
        "1.3.5" => main_1_3_5(),
        "1.3.6" => main_1_3_6(),
        "1.4.1" => main_1_4_1(),
        "1.4.2" => main_1_4_2(),
        "1.4.3" => main_1_4_3(),
        "1.4.4" => main_1_4_4(),
        "1.4.5" => main_1_4_5(),
        "1.4.6" => main_1_4_6(),
        "1.5.1" => main_1_5_1(),
        "1.5.2" => main_1_5_2(),
        "1.5.3" => main_1_5_3(),
        _ => {}
    }
}
