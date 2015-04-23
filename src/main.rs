//use std::io::{stdio};
use std::io;
use std::thread;
use std::rand;

fn start_getting_weather() {
    let mut loop_counter = 0;
	let delay = 3000;
    loop {
        loop_counter += 1;
        println!("counter: {}", loop_counter);
        thread::spawn(move || test_fn());
        //main_weather_getter();
        thread::sleep_ms(delay);
    }

}

fn test_fn() {
    for i in 1..5 {
        thread::spawn(move || {
            let x: u32 = randTime();
            thread::sleep_ms(x);
            println!("{}", i);
        });
    }
}

fn test_print(x: i32) {
    println!("called! {}", x)
}

fn randTime() -> i32 {
    let n: i32 = (rand::random() % 100i64) + 1i64;
    return n
}

fn main() {
	thread::spawn(move || start_getting_weather());
	// Prevent main from exiting early

    let mut stdin = io::stdin();
    let input = &mut String::new();

    // loop {
    //     input.clear();
    //     stdin.read_line(input);
    //     println!("{}", input);
    // }

    // prevent main from exiting
    input.clear();
    stdin.read_line(input);
    println!("{}", input);

}
