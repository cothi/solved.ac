// X - 1
// 1 2, 2 1 - 2
//
//

fn down_top(n: usize) -> u128 {
    let mut v: Vec<u128> = vec![0; n + 4];
    v[1] = 1;
    v[2] = 2;
    for i in 3..n + 1 {
        v[i] = v[i - 1] + v[i - 2];
        v[i] = v[i] % 10007
        //println!("{}", v[i]);
    }
    v[n]
}

fn main() {
    let mut n = String::new();
    std::io::stdin().read_line(&mut n).unwrap();
    let size: usize = n.trim().parse().unwrap();
    let res = down_top(size);
    println!("{}", res % 10007);
}
