use colored::{ColoredString, Colorize};
use std::{env, process::exit};

fn main() {
    // get all args
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        show_help();
    }
    else if args[1] == "help" {
        show_help();
    }

    let color: Result<i32, _> = args[2].parse::<i32>();
    let unwrapped_color: i32 = if color.is_err() { 
        show_errorhelp("Invalid color");
        exit(0)
    }
    else {
        color.unwrap()
    };

    if &unwrapped_color > &8 {
        show_errorhelp("<Color> cannot be above 8");
    }
    else {
        catsay(&args[1], unwrapped_color);
    }
}

fn show_errorhelp(error: &str) {
    show_error(error);
    show_help();
}

fn show_error(error: &str) {
    print!("\n");
    println!(" {}", error.red());
}

fn show_help() {
    // spacing
    print!("\n");

    println!("  {}", "Usage: catsay <Phrase> <Color>");
    print!("  {}", "Colors:".black());
    println!(
        "  {}{}{}{}{}{}{}{}",
        " 1 ".white().on_black(),
        " 2 ".black().on_red(),
        " 3 ".black().on_green(),
        " 4 ".black().on_yellow(),
        " 5 ".black().on_blue(),
        " 6 ".black().on_magenta(),
        " 7 ".black().on_cyan(),
        " 8 ".black().on_white()
    );
    exit(0);
}

fn catsay(say: &str, color: i32) {
    let line1 = "   /| ､      ";
    let line2 = format!("  (°､ ｡ 7    {say}");
    let line3 = "   |､  ~ヽ   ";
    let line4 = "   じしf_,)〳 ";
    let cat: [&str; 4] = [line1, &line2, line3, line4];
    
    print!("\n");
    for i in cat {
        println!("{}", colorize(i, &color));
    }
}

fn colorize(i: &str, color: &i32) -> ColoredString {
    match color {
        1 => return i.black(),
        2 => return i.red(),
        3 => return i.green(),
        4 => return i.yellow(),
        5 => return i.blue(),
        6 => return i.magenta(),
        7 => return i.cyan(),
        8 => return i.white(),
        // catch all
        _ => return "Invalid color specified".red(),
    }
}
