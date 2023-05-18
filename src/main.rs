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
    // Pring a zrandom choine
    let output = match color {
        Some(c) => format!("{}", x[n].color(c).to_string().bold()),
        _ => format!("{}", x[n].color(Color::Default).to_string()),
    };
    println!("{}", output);
}
fn colored_options(x: &[&str]) {
    let mut rng = thread_rng();
    let n = rng.gen_range(1..10);

    match n {
        1 => println!("{}", x[n].red()),
        2 => println!("{}", x[n].bright_red().bold()),
        3 => println!("{}", x[n].yellow()),
        4 => println!("{}", x[n].bright_yellow().bold()),
        5 => println!("{}", x[n].green()),
        6 => println!("{}", x[n].bright_green().bold()),
        7 => println!("{}", x[n].blue()),
        8 => println!("{}", x[n].bright_blue().bold()),
        9 => println!("{}", x[n].purple()),
        10 => println!("{}", x[n].bright_purple().bold()),

        _ => eprintln!("You are not meant to be here!"),
    }
    return;
}

fn magic_output(raw: &str) {
    // match input to below =
    if raw.contains("~") {
        println!("\x1b[2J");
        //
    } else if raw.contains("rust") {
        options(&["rust1", "rust2", "rust3"], Some(Color::Magenta));
        //
    } else if raw.contains("discord") {
        options(&["discord1", "discord2", "discord3"], Some(Color::Yellow));
        //
    } else if raw.contains("helix") {
        options(&["helix1", "helix2", "helix3", "helix4", "helix5"], None);
        //
    } else {
        colored_options(&[
            "Not quite",
            "Yes!",
            "Shoot for the starst",
            "yeh that is not for you",
            "Maybe try again later",
            "Yes! that suites you so well",
            "What do you want me to say?",
            "Funny one...",
            "Of cource!",
            "I didnt hear a word you said",
        ]);
    }
}
