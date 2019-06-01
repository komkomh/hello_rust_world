fn main() {
    let mut vec = vec![1];

    // vecを表示
    vec.iter().for_each(|v| print!("{},", v));
    println!();

    borrow3(&mut vec);

    // vecを表示
    vec.iter().for_each(|v| print!("{},", v));
    println!();

    // vecに追加
    vec.push(5);

    // vecを表示
    vec.iter().for_each(|v| print!("{},", v));
    println!();

    let mut x = 5;

    let y = &mut x;
    *y += 1;
    println!("y = {}", y);

    println!("x = {}", &x);
}

fn borrow1(vec: Vec<i32>) -> Vec<i32> {
    let sum: i32 = vec.iter().sum();
    println!("sum = {}", sum);
    vec
}

fn borrow2(vec: &Vec<i32>) {
    let sum: i32 = vec.iter().sum();
    println!("sum = {}", sum);
}

fn borrow3(vec: &mut Vec<i32>) {
    let sum: i32 = vec.iter().sum();
    println!("sum = {}", sum);
    vec.push(7);
}