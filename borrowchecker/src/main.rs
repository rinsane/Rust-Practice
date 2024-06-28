fn main() {
    println!("Hello, world!");
    let s = String::from("hello world");
    // let s_new = s; // changes ownership from s to s_new!
    let word = first_word(&s);
    // s.clear();

    println!("{}", word)
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, item) in bytes.iter().enumerate() {
        if *item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}
