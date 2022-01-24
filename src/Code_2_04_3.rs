fn main() {
    let mut answer: i64 = 0;
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();
    let mut ws = s.split_whitespace();

    let N: i32 = ws.next().unwrap().parse().unwrap();
    let S: i32 = ws.next().unwrap().parse().unwrap();

    for i in 1..=N {
        for j in 1..=N {
            if i + j <= S {
                answer += 1;
            };
        }
    }
    println!("{}", answer);
}
