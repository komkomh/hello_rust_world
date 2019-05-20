fn main() {
    let new_array = create_array();
    show_array(new_array);

    println!("[0][2] {}", new_array[0][2]);
    println!("[1][2] {}", new_array[1][2]);
    println!("[2][0] {}", new_array[2][0]);
}

fn create_array() -> [[i8; 3]; 3] {
    [[0, 1, 2], [3, 4, 5], [6, 7, 8]]
}

fn show_array(array2: [[i8; 3]; 3]) {
    array2.iter().for_each(|array| {
        array.iter().for_each(|x| print!("{}", x));
        println!();
    });
}