fn main() {
    for i in 1..100 {
        if is_sosu(i) {
            print!("{} ", i)
        }
    }
}

fn is_sosu(target: i32) -> bool {
    if target < 2 {
        return false;
    }

    for i in 2..target {
        if target % i == 0 {
            return false;
        }
    }
    return true;
}
