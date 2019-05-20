fn main() {
    for x in 1..40 {
        match x {
            x if (x % 15 == 0) => print!("fizzbuzz "),
            x if (x % 5 == 0) => print!("buzz "),
            x if (x % 3 == 0) => print!("fizz "),
            _ => print!("{} ", x),
        }
    }
}
