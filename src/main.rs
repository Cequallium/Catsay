use std::env;
use colored::*;


fn main() {
    // let args: Vec<_> = env::args().collect();
    let args: Vec<_> = env::args().collect();
    if args.len() == 3 {
        catsay(&args[1], &args[2].parse::<i16>().unwrap());
    } else {
        catsay("both <Phrase> & <Color> are required",&1);
    }
}

fn catsay(say: &str,color: &i16) {

    let line1: &str = "   /| ､      ";
    let line2: &str = "  (°､ ｡ 7    ";
    let line3: &str = "   |､  ~ヽ   ";
    let line4: &str = "   じしf_,)〳 ";
    let cat: [&str; 4] = [line1,line2,line3,line4];

    for i in 0..=3{
        if i == 1{
            println!("{} {}",colorize(cat[i],color), colorize(say, color));
        }else{
            println!("{}",colorize(cat[i],color));
        }
    } 

}

fn colorize(i: &str,color: &i16) -> ColoredString{
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
        _ => std::process::exit(1)
    }
}