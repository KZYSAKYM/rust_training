fn main() {
    println!("Please specify max number");

    let mut _in = String::new();

    std::io::stdin().read_line(&mut _in)
        .expect("Failed to read line");

    println!("{} inputted", _in);

    let max: i32 = _in.trim().parse().unwrap();

    for i in 0..max {
        let mut msg = String::new();
        if i % 3 == 0 {
            msg = msg + "Fizz"
        }
        if i % 5 == 0 {
            msg = msg + "Buzz"
        }
        if msg == "" {
            msg = i.to_string()
        }
        println!("{}", msg)
    }
}
