use owo_colors::{colored::Color, OwoColorize};
use rand::{thread_rng, Rng};
struct EightBall;
impl EightBall {
    fn input() -> String {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).expect("err");
        input
    }

    fn options(x: &[&str], color: Color) {
        // random
        let mut rng = thread_rng();
        let n = rng.gen_range(0..x.len());
        // Pring a zrandom choine
        let output = format!("{}", x[n].color(color).to_string().bold());
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
}
fn main() {
    loop {
        let raw_input = EightBall::input().to_lowercase();
        magic_output(&raw_input);
    }
}

fn magic_output(raw: &str) {
    match raw {
        raw if raw.contains("~") => {
            println!("\x1b[2J");
            //
        }
        raw if raw.contains("rust") => {
            EightBall::options(&["rust1", "rust2", "rust3"], Color::Magenta);
            //
        }
        raw if raw.contains("discord") => {
            EightBall::options(&["discord1", "discord2", "discord3"], Color::Yellow);
            //
        }
        raw if raw.contains("helix") => {
            EightBall::options(
                &["helix1", "helix2", "helix3", "helix4", "helix5"],
                Color::Default,
            );
            //
        }
        _ => {
            EightBall::colored_options(&[
                "Not quite",
                "Yes!",
                "Shoot for the stars",
                "yeh that is not for you",
                "Maybe try again later",
                "Yes! that suits you so well",
                "What do you want me to say?",
                "Funny one...",
                "Of course!",
                "I didn't hear a word you said",
            ]);
        }
    }
}
