fn main() {
    let ban = create_ban();
    show_ban(ban);
}

fn create_ban() -> [[i8; 4]; 4] {
    [
        [0, 0, 0, 0],
        [0, -1, 1, 0],
        [0, 1, -1, 0],
        [0, 0, 0, 0],
    ]
}

fn show_ban(ban: [[i8; 4]; 4]) {
    ban.iter().for_each(|array| {
        array.iter().for_each(|stone| {
            let stone_display = match stone {
                1 => "●",
                -1 => "○",
                _ => " "
            };
            print!("|{}", stone_display);
        });
        println!("|");
    });
}
