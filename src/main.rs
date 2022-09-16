use std::env::args;
use colored::Colorize;
use colored::ColoredString;

fn main() {

    let args: Vec<String> = args().collect();
    
    // spaghetti error handling / routing
    if args[1] == "--help" {

        // spacing
        println!("");

        println!("  {}","Usage: catsay <Phrase> <Color>".white());
        print!("  {}","Colors:".black());
        println!("  {}{}{}{}{}{}{}{}",
                " 1 ".on_black(),
                " 2 ".on_red(),
                " 3 ".on_green(),
                " 4 ".on_yellow(),
                " 5 ".on_blue(),
                " 6 ".on_magenta(),
                " 7 ".on_cyan(),
                " 8 ".on_white());
    }
    else if args.len() == 3 {

        let arg2: &i8 = &args[2].parse::<i8>().unwrap();

        if arg2 <= &8{
            catsay(&args[1], arg2);
        }
        else {
            catsay("<Color> has to be =<8", &2)
        }
    }
    else {
        catsay("both <Phrase> & <Color> are required",&1);
    }
}

fn catsay(say: &str,color: &i8) {

    let line1: &str = "   /| ､      ";
    let line2: &str = "  (°､ ｡ 7    ";
    let line3: &str = "   |､  ~ヽ   ";
    let line4: &str = "   じしf_,)〳 ";
    let cat: [&str; 4] = [line1,line2,line3,line4];

    println!(""); // blank space
    for i in 0..=3{
        if i == 1{
            println!("{} {}",colorize(cat[i],color), colorize(say, color));
        }else{
            println!("{}",colorize(cat[i],color));
        }
    } 

}

fn colorize(i: &str,color: &i8) -> ColoredString{
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
        _ => return "invalid color specified".red()
    }
}