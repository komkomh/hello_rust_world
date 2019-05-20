fn main() {
//    for x in 0..10 {
//        println!("Hello, world!{}", x)
//    }
//    (0..5).for_each(|_| println!("Hello, world!"));

    (-1..5).for_each(|x| println!("Hello, world!{}", x));

    println!("--------------");
    (0..10).filter(|x| x % 2 == 0).map(|x| x * 2).for_each(|x| println!("{}", x));
}
