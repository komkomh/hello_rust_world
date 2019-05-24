macro_rules! input {
    (source = $s:expr, $($r:tt)*) => {
        let mut iter = $s.split_whitespace();
        let mut next = || { iter.next().unwrap() };
        input_inner!{next, $($r)*}
    };
    ($($r:tt)*) => {
        let stdin = std::io::stdin();
        let mut bytes = std::io::Read::bytes(std::io::BufReader::new(stdin.lock()));
        let mut next = move || -> String{
            bytes
                .by_ref()
                .map(|r|r.unwrap() as char)
                .skip_while(|c|c.is_whitespace())
                .take_while(|c|!c.is_whitespace())
                .collect()
        };
        input_inner!{next, $($r)*}
    };
}

macro_rules! input_inner {
    ($next:expr) => {};
    ($next:expr, ) => {};

    ($next:expr, $var:ident : $t:tt $($r:tt)*) => {
        let $var = read_value!($next, $t);
        input_inner!{$next $($r)*}
    };
}

macro_rules! read_value {
    ($next:expr, ( $($t:tt),* )) => {
        ( $(read_value!($next, $t)),* )
    };

    ($next:expr, [ $t:tt ; $len:expr ]) => {
        (0..$len).map(|_| read_value!($next, $t)).collect::<Vec<_>>()
    };

    ($next:expr, chars) => {
        read_value!($next, String).chars().collect::<Vec<char>>()
    };

    ($next:expr, usize1) => {
        read_value!($next, usize) - 1
    };

    ($next:expr, $t:ty) => {
        $next().parse::<$t>().expect("Parse error")
    };
}

fn main() {
    input! {
        a: [usize; 5]
    };

    // カード数を数える
    let mut card_counts:[usize; 13] = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    for i in 0..a.len() {
        let card_number = a[i];
        let card_count = card_counts[card_number - 1];
        card_counts[card_number - 1] = card_count + 1;
    }

    // 3枚組の数を取得する
    let three = card_counts.iter().filter(|&count| *count == 3).count();
    // 2枚組の数を取得する
    let two = card_counts.iter().filter(|&count| *count == 2).count();

    // 3枚組、2枚組毎に役を表示する
    let message = match (three, two) {
        (1, 1) => "FULL HOUSE",
        (1, 0) => "THREE CARD",
        (0, 2) => "TWO PAIR",
        (0, 1) => "ONE PAIR",
        _ => "NO HAND"
    };
    println!("{}", message);
}

