fn main() {
    let s = get_line();
    let a: Vec<_> = s.trim().split(' ').collect();
    let x: i32 = a[0].parse().unwrap();
    let y: i32 = a[1].parse().unwrap();
    print!("{} ", x + y);
    println!("{}", getline());
}

fn get_line() -> String {
    let mut __ret = String::new();
    std::io::stdin().read_line(&mut __ret).ok();
    return __ret;
}
