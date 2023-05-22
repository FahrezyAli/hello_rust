use ferris_says::say; // from the previous step
use std::io::{stdout, BufWriter};

fn main() {
    let stdout = stdout();
    let message = String::from("Hello fellow Rustaceans!");
    let width = message.chars().count();

    let mut writer = BufWriter::new(stdout.lock());
    say(message.as_bytes(), width, &mut writer).unwrap();

    let number: Vec<i32> = (0..10).collect();

    fn do_stuff(val: &Vec<i32>) {
        println!("{}", val.len());
    }

    do_stuff(&number);
}
