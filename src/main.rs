use owo_colors::{colored::Color, OwoColorize};
use rand::{thread_rng, Rng};

fn main() {
    loop {
        let raw_input = input().to_lowercase();
        magic_output(&raw_input);
    }
}

fn input() -> String {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("err");
    input
}

fn options(x: &[&str], color: Option<Color>) {
    // random
    let mut rng = thread_rng();
    let n = rng.gen_range(0..x.len());
    // Pring a random choine
    let output = match color {
        Some(c) => format!("{}", x[n].color(c).to_string().bold()),
        _ => format!("{}", x[n].color(Color::Default).to_string()),
    };
    println!("{}", output);
}

fn magic_output(raw: &str) {
    // match input to below =
    if raw.contains("rust") {
        options(&["rust1", "rust2", "rust3"], Some(Color::Magenta));
        //
    } else if raw.contains("discord") {
        options(&["discord1", "discord2", "discord3"], Some(Color::Yellow));
        //
    } else if raw.contains("helix") {
        options(&["helix1", "helix2", "helix3", "helix4", "helix5"], None);
        //
    }
}
