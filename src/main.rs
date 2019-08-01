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
        "thisisfine" | "hot-take" | "fire "=> { tweet = format!("🔥{}🔥", space_regex.replace_all(&tweet, "🔥")); }
        "clap" => { tweet = format!("👏{}👏", space_regex.replace_all(&tweet, "👏")); }   
        "rainbow" =>{ tweet = format!("🌈{}🌈", space_regex.replace_all(&tweet, "🌈")); }
        "sparkles" | "sparkle" =>{ tweet = format!("✨{}✨", space_regex.replace_all(&tweet, "✨")); }
        "hairflip" => { tweet = format!("💁‍{}💁‍", space_regex.replace_all(&tweet, "💁‍")); }
        "facepalm" => { tweet = format!("🤦‍{}🤦‍", space_regex.replace_all(&tweet, "🤦‍")); }
        "poo" | "shit-take" => { tweet = format!("💩{}💩", space_regex.replace_all(&tweet, "💩")); }
        "death" | "jolly-roger" => { tweet = format!("☠︎{}☠︎", space_regex.replace_all(&tweet, "☠︎"));  }
        "tableflip" => { tweet = format!("{} (╯°□°)╯︵ ┻━┻︎", &tweet); }
        "tableback" | "puttableback" => { tweet = format!("{},", "┬─┬ノ( º _ ºノ)") }
        "smallcaps" => {
            let mut newTweet = String::from("");
            for mut chirp in tweet.chars() {
                chirp = toSmallCaps(chirp);
                newTweet.push(chirp);
            }
            println!("{}", newTweet);
            tweet = newTweet;
        }
        &_ => {
            println!("No valid decoration type specified");
        }
    }
    ctx.set_contents(format!("{}", &tweet)).unwrap();

    println!("{} copied to clipboard!", &tweet);
}

fn toSmallCaps (original_letter:char) -> char{
    let mut new_letter= original_letter;
    match original_letter{
        'A' | 'a' => {new_letter= 'ᴀ'}
        'B' | 'b' => {new_letter= 'ʙ'}
        'C' | 'c' => {new_letter= 'ᴄ'}
        'D' | 'd' => {new_letter= 'ᴅ'}
        'E' | 'e' => {new_letter= 'ᴇ'}
        'F' | 'f' => {new_letter= 'ꜰ'}
        'G' | 'g' => {new_letter= 'ɢ'}
        'H' | 'h' => {new_letter= 'ʜ'}
        'I' | 'i' => {new_letter= 'ɪ'}
        'J' | 'j' => {new_letter= 'ᴊ'}
        'K' | 'k' => {new_letter= 'ᴋ'}
        'L' | 'l' => {new_letter= 'ʟ'}
        'M' | 'm' => {new_letter= 'ᴍ'}
        'N' | 'n' => {new_letter= 'ɴ'}
        'O' | 'o' => {new_letter= 'ᴏ'}
        'P' | 'p' => {new_letter= 'ᴘ'}
        'Q' | 'q' => {new_letter= 'ꞯ'}
        'R' | 'r' => {new_letter= 'ʀ'}
        'S' | 's' => {new_letter= 'ꜱ'}
        'T' | 't' => {new_letter= 'ᴛ'}
        'U' | 'u' => {new_letter= 'ᴜ'}
        'V' | 'v' => {new_letter= 'ᴠ'}
        'W' | 'w' => {new_letter= 'ᴡ'}
        'X' | 'x' => {new_letter= '–'}
        'Y' | 'y' => {new_letter= 'ʏ'}
        'Z' | 'z' => {new_letter= 'z'}
        _ => {new_letter= original_letter}
    }
    return new_letter;
}