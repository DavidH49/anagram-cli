use clap::Parser;

fn main() {
    let args = Args::parse();

    let anagram = is_anagram(&args.a_string, &args.b_string);
    
    if anagram {
        println!("'{}' ist ein Anagramm von '{}'.", args.a_string, args.b_string)
    } else {
        println!("'{}' ist kein Anagramm von '{}'.", args.a_string, args.b_string)
    }
}


#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short)]
    a_string: String,

    #[arg(short)]
    b_string: String,
}


fn is_anagram(s1: &String, s2: &String) -> bool {
    if s1.len() != s2.len() {
        return false;
    }

    let mut s1 = Vec::from(s1.trim().to_lowercase());
    let mut s2 = Vec::from(s2.trim().to_lowercase());

    s1.sort();
    s2.sort();
    
    s1 == s2
}