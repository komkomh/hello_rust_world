use std::collections::HashSet;

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
        n: usize,
        answers: [[String; 5]; n],
    }

    // 全カード
    let mut cards: HashSet<usize> = vec!(0, 1, 2, 3, 4, 5, 6, 7, 8, 9).into_iter().collect();

    // YESカードを処理する
    answers
        .iter()
        .filter(|anss| anss[4] == "YES")
        .map(|anss| to_num_slice(anss.to_vec()))
        .for_each(|anss| {
            (0 as usize..10 as usize) // borrow checker...
                .filter(|&card| !anss.contains(&card))
                .for_each(|no_card| {
                    cards.remove(&no_card);
                });
        });

    // NOカードを処理する
    answers
        .iter()
        .filter(|anss| anss[4] == "NO")
        .flat_map(|anss| to_num_slice(anss.to_vec()).to_vec())
        .for_each(|no_card| {
            cards.remove(&no_card);
        });

    // 答え
    cards.iter().for_each(|card| println!("{}", card));
}

fn to_num_slice(answers: Vec<String>) -> [usize; 4] {
    [
        answers[0].parse().unwrap(),
        answers[1].parse().unwrap(),
        answers[2].parse().unwrap(),
        answers[3].parse().unwrap()
    ]
}
