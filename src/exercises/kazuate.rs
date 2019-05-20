use std::io;

fn main() {
    println!("数字を入力してください");

    let mut guess = String::new();

    io::stdin().read_line(&mut guess)
        .expect("Failed to read line");

    println!("入力された数字は: {}", guess);
}