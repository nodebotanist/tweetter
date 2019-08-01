extern crate clap;
use clap::{Arg, App};

fn main() {
    let matches = App::new("Tweet Formatter")
        .version("0.1")
        .author("Kas Perch <the@nodebotani.st>")
        .about("Takes you tweet text, decorates it, and copies the decorated tweet to your clipboard")
        .arg(Arg::with_name("TWEET_TEXT")
            .help("The text you would like to decorate")
            .short("t")
            .long("tweet")
            .required(true)
            .index(1))
        .arg(Arg::with_name("decoration_type")
            .help("tells the cli what decoration you want on your tweet")
            .short("d")
            .long("decoration")
            .required(true)
            .index(2))
        .get_matches();

    if matches.value_of("decoration_type").unwrap() == "thisisfine" {
        println!("🔥")
    } else if matches.value_of("decoration_type").unwrap() == "clap" {
        println!("👏")
    } else {
        println!("No subcommand entered!")
    }
}
