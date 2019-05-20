const BAN_SIZE: usize = 4;

fn main() {
    let ban = create_ban();
    show_ban(ban);

    let count = get_reverse_count(ban, -1, (0, 1));
    println!("{}", count);
}

fn create_ban() -> [[i8; 4]; 4] {
    [
        [0, 0, 0, 0],
        [0, 1, -1, 0],
        [0, -1, 1, 0],
        [0, 0, 0, 0],
    ]
}

fn show_ban(ban: [[i8; BAN_SIZE]; BAN_SIZE]) {
    ban.iter().for_each(|array| {
        array.iter().for_each(|stone| {
            let stone_display = match stone {
                -1 => "●",
                1 => "○",
                _ => " "
            };
            print!("|{}", stone_display);
        });
        println!("|")
    });
}

fn get_reverse_count(ban: [[i8; BAN_SIZE]; BAN_SIZE], stone: i8, point: (i8, i8)) -> i8 {
    let point_stone = ban[point.0 as usize][point.1 as usize];
    if point_stone != 0 {
        return 0;
    }
    let reverse_count = (-1..2)
        .map(|y| {
            let line_reverse_count: i8 = (-1..2)
                .map(|x| get_reverse_count_line(ban, stone, point, (y, x), 0))
                .sum();
            return line_reverse_count;
        })
        .sum();
    return reverse_count;
}

fn get_reverse_count_line(ban: [[i8; BAN_SIZE]; BAN_SIZE],
                          stone: i8,
                          point: (i8, i8),
                          direction: (i8, i8),
                          reverse_count: i8) -> i8 {
    // 調査点を生成する
    let y = point.0 + direction.0;
    let x = point.1 + direction.1;

    // 盤の端まで来ていたら
    if y < 0 || y >= BAN_SIZE as i8 || x < 0 || x >= BAN_SIZE as i8 {
        // 反転数0を返却する
        return 0;
    }

    // この場所の石が
    return match ban[y as usize][x as usize] {
        // 無ければ反転数0を返す
        point_stone if (point_stone == 0) => 0,
        // 自石なら反転数を返す
        point_stone if (point_stone == stone) => reverse_count as i8,
        // 他石なら次の場所を読む
        _ => get_reverse_count_line(
            ban, stone, (y, x), direction, reverse_count + 1)
    };
}

// 場所毎の点数を取得する
// -> 置けるか判定する -> ダメならEmpty
// -> x階層潜る
// -> 場所の最大点数を取得する
// -> x階層帰る
// 最大点数を返す