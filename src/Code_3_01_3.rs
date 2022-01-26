//! [p73] 約数をすべて出力するプログラム

fn main() {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();
    let mut ws = s.split_whitespace();

    let N: i64 = ws.next().unwrap().parse().unwrap();

    let mut answer: Vec<i64> = Vec::new();
    let mut i = 1;
    while i * i <= N {
        if N % i != 0 {
            i += 1;
            continue;
        }
        answer.push(i);
        if i != N / i {
            answer.push(N / i);
        }
        i += 1;
    }
    answer.sort();
    println!("{:?}", answer);
}
