extern crate clap;
use clap::{Arg, App};
extern crate clipboard;
use clipboard::{ClipboardContext, ClipboardProvider};
extern crate regex;
use regex::Regex;

fn main() {
    let matches = App::new("Tweet Formatter")
        .version("0.1")
        .author("Kas Perch <the@nodebotani.st>")
        .about("Takes you tweet text, decorates it, and copies the decorated tweet to your clipboard")
        .arg(Arg::with_name("tweet_text")
            .help("The text you would like to decorate")
            .short("t")
            .long("tweet")
            .required(true)
            .index(2))
        .arg(Arg::with_name("decoration_type")
            .help("tells the cli what decoration you want on your tweet")
            .short("d")
            .long("decoration")
            .required(true)
            .index(1))
        .get_matches();

    let space_regex = Regex::new(r"\s").unwrap();

    let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
    let mut tweet = String::from(matches.value_of("tweet_text").unwrap());
    let decoration = matches.value_of("decoration_type").unwrap();

    println!("Original Tweet: {}", tweet);

    match decoration{
        "thisisfine" => { tweet = format!("🔥{}🔥", space_regex.replace_all(&tweet, "🔥")); }
        "clap" => { tweet = format!("👏{}👏", space_regex.replace_all(&tweet, "👏")); }   
        "rainbow" =>{ tweet = format!("🌈{}🌈", space_regex.replace_all(&tweet, "🌈")); }
        "sparkles" =>{ tweet = format!("✨{}✨", space_regex.replace_all(&tweet, "✨")); }
        &_ => {
            println!("No valid decoration type specified");
        }
    }
    ctx.set_contents(format!("{}", &tweet)).unwrap();

    println!("{} copied to clipboard!", &tweet);

}