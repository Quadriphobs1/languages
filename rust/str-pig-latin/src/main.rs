// Convert strings to pig latin. The first consonant of each word is moved to the
// end of the word and “ay” is added, so “first” becomes “irst-fay.” Words that
// start with a vowel have “hay” added to the end instead
// (“apple” becomes “apple-hay”)

fn pig_latin(s: String) -> String {
    let mut result = String::new();
    let c = s.chars().next().unwrap();
    let mut suffix = String::new();
    match c {
        'a' | 'e' | 'i' | 'o' | 'u' => {
            result.push(c);
            suffix.push_str("-hay");
        }
        _ => {
            suffix.push_str(&format!("-{}ay", c));
        }
    };

    let str = format!("{}{}", &s[1..], suffix);
    result.push_str(&str);
    return result;
}

fn main() {
    let a = pig_latin(String::from("first"));
    let b = pig_latin(String::from("apple"));
    let c = pig_latin(String::from("bingo"));
    let d = pig_latin(String::from("universe"));

    println!("{}", a);
    println!("{}", b);
    println!("{}", c);
    println!("{}", d);
}
