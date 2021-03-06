mod camera;
mod mesh;
mod model;
mod shader;

mod _1_getting_started;
mod _2_lighting;
mod _3_model_loading;

use _1_getting_started::*;
use _2_lighting::*;
use _3_model_loading::*;

const MAIN_PROGRAM: &str = "3.1";

fn main() {
    let mut main = MAIN_PROGRAM;
    let args: Vec<String> = std::env::args().collect();

    if args.len() > 1 {
        main = args[1].as_str();
    }

    match main {
        // Chapter 1 - Get started
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
        "1.6.1" => main_1_6_1(),
        "1.6.2" => main_1_6_2(),
        "1.6.3" => main_1_6_3(),
        "1.6.4" => main_1_6_4(),
        "1.7.1" => main_1_7_1(),
        "1.7.2" => main_1_7_2(),
        "1.7.3" => main_1_7_3(),
        "1.7.4" => main_1_7_4(),
        "1.7.5" => main_1_7_5(),
        "1.7.6" => main_1_7_6(),

        // Chapter 2 - Lighting
        "2.1" => main_2_1(),
        "2.2.1" => main_2_2_1(),
        "2.2.2" => main_2_2_2(),
        "2.2.3" => main_2_2_3(),
        "2.2.4" => main_2_2_4(),
        "2.2.5" => main_2_2_5(),
        "2.3.1" => main_2_3_1(),
        "2.3.2" => main_2_3_2(),
        "2.4.1" => main_2_4_1(),
        "2.4.2" => main_2_4_2(),
        "2.4.3" => main_2_4_3(),
        "2.4.4" => main_2_4_4(),
        "2.5.1" => main_2_5_1(),
        "2.5.2" => main_2_5_2(),
        "2.5.3" => main_2_5_3(),
        "2.5.4" => main_2_5_4(),
        "2.6" => main_2_6(),

        // Chapter 3 - Model Loading
        "3.1" => main_3_1(),

        _ => {}
    }
}
