use glfw::{Action, Context, Key};
mod ch_1;
mod ch_2;
mod ch_3;
mod ch_4;
mod section_ii;
mod shader;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    println!("Args: {:?}", args);
    match args[1].as_str() {
        "ch_1" => ch_1::main_hello_triangle(),
        "ch_2" => ch_2::main_hello_color(),
        "ch_2_va" => ch_2::vertex_attributes::main(),
        "ch_3" => ch_3::part1::main(),
        "ch_3_2" => ch_3::part2::main(),
        "ch_3_3" => ch_3::part3::main(),
        "ch_3_4" => ch_3::part4::main(),
        "ch_4_1" => ch_4::part1::main(),
        "ch_4_2" => ch_4::part2::main(),
        "ch_4_3" => ch_4::part3::main(),
        "ch_4_4" => ch_4::part4::main(),
        "ch_5_1" => section_ii::ch_5_1::main(),
        _ => println!("Unimplemented"),
    }
}
