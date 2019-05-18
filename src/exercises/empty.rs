fn main() {
    (1..100)
        .filter(|i| is_sosu(i))
        .for_each(|i| print!("{} ", i));
}

fn is_sosu(target: &i32) -> bool {
    if target < &2 {
        return false
    }
    !(2..*target)
        .filter(|i| *i > 1)
        .any(|i| target % i == 0)
}