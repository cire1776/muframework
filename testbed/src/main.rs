use chrono::Duration;

#[derive(Debug, Clone)]
enum Command {
    First,
    Second,
    Third,
    Fourth,
    Fifth,
    Sixth,
}

fn main() {
    let (tx, rx) = std::sync::mpsc::channel();

    let timer = MessageTimer::new(tx.clone());

    let guard1 = timer.schedule_repeating(Duration::seconds(1), Command::First);

    let guard2 = timer.schedule_repeating(Duration::seconds(3), Command::Second);

    let guard3 = timer.schedule_repeating(Duration::seconds(5), Command::Third);

    let guard4 = timer.schedule_repeating(Duration::seconds(6), Command::Fourth);

    let guard5 = timer.schedule_repeating(Duration::seconds(7), Command::Fifth);

    let guard6 = timer.schedule_repeating(Duration::seconds(8), Command::Sixth);
    loop {
        let command = rx.recv();

        match command {
            Ok(Command::First) => println!("first"),
            Ok(Command::Second) => println!("second"),

            Ok(Command::Third) => println!("third"),
            Ok(Command::Fourth) => println!("fourth"),
            Ok(Command::Fifth) => println!("fifth"),
            Ok(Command::Sixth) => println!("sixth"),
            Err(error) => println!("{:?}", error),
        }
    }
}
