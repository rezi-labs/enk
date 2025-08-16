#[allow(unused)]
mod vars {
    use std::env;

    fn x() {
        env::var("PATH").unwrap_or_else(|_| String::from("No PATH found"));
    }

    fn y() {
        env::var("HOME").unwrap_or_else(|_| String::from("No HOME found"));
    }
}
