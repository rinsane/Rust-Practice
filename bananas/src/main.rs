
fn main() {
    let banana = 'a';
    let x = "abc";
    let y = x;
    let arr = [1,2,3];
    let t = x.chars().nth(1).unwrap_or_default();
    println!("{}", t);

}