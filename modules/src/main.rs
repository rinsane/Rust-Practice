use outer2::inner2;
use outer3::inner3::inner3_function;

mod outer;
mod outer2;
mod outer3 {
    pub mod inner3 {
        pub fn inner3_function() {
            println!("This is inside the outer3 => inner3 => inner3_function()");
        }
    }
}

fn main() {
    println!("Hello, world!");
    outer::inner::inner_function();
    inner2::inner2_function();
    inner3_function();
}
