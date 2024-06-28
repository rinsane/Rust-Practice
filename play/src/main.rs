#![allow(dead_code)]
#![allow(unused_variables)]

#[derive(Debug)]
struct User {
    name: String,
    age: u32,
    gender: char,
}

#[derive(Debug)]
struct TupUser(String, u32, char);

impl User {
    fn printer(&self) {
        println!("{:?}", self);
    }
}

#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn printer(&self) {
        match self {
            Message::Quit => println!("{:?}ting...", Message::Quit),
            Message::Move { x, y } => println!("x: {x}, y: {y}"),
            Message::Write(string) => println!("Writing: {}", string),
            Message::ChangeColor(r, g, b) => println!("{}-{}-{}", r, g, b),
        }
    }

    fn quit(&mut self) {
        *self = Message::Quit;
    }
}
fn main() {
    println!("Hello, world!");
    {
        let u1 = User {
            name: String::from("Rishabh Raj"),
            age: 21 * 2,
            gender: 'M',
        };
        let u2 = TupUser(String::from("Satvic"), 5, 'M');

        dbg!(&u2);
        u1.printer();
        println!("{:#?}", u2);
        println!("{}", u1.name);
    }

    {
        let mut myenum = Message::Move { x: 10, y: 100 };
        myenum.printer();
        myenum = Message::ChangeColor(44, 55, 66);
        myenum.printer();
        myenum.quit();
        myenum.printer();
        // println!("{}", myenum as u32) // panics! because this can only be done with type unit-only
    }

    {
        let somenumber: Option<i32> = Some(69);
        let somenumber2: Option<i32> = Some(42);
        println!("{:?}", somenumber);
        match somenumber {
            Some(num) => println!("we have a number {num}"),
            None => println!("Nothing bro"),
        }
        // println!("{}", somenumber.unwrap()); // panic if somenumber == None!
        if let Some(n1) = somenumber {
            if let Some(n2) = somenumber2 {
                println!("{}", n1 + n2);
            }
        }
        println!("{:?}", somenumber);
    }

    {
        let apple = println!("this is an assigned print statement!");
        println!("{apple}");
    }
    let s = String::from("hello world");
}
