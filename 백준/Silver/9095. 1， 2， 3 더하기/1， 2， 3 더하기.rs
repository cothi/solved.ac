/* 1, 2, 3 sum type
* 0 - 0
* 1 - 1
* 1 1, 2 - 2
* 1 1, 1 2, 2 1, 3 - 4
* 1 1 1 1, 1 1 2, 1 2 1, 2 1 1, 2 2, 1 3, 3 1 - 7
* 1 1 1 1 1
* 1 1 1 2, 1 1 2 1, 1 1 2 1, 1 2 1 1, 2 1 1 1;
* 1 2 2, 2 1 2, 2 2 1
* 1 1 3, 1 3 1, 3 1 1
* 2 3, 3 2
* dp - 뒤에 것을 더해야하는데 이것은 불가능하다.
*/
use ::std::io;

fn down_top(sz: usize) -> u32 {
    let mut v: Vec<u32> = vec![0; sz + 4];

    v[0] = 0;
    v[1] = 1;
    v[2] = 2;
    v[3] = 4;

    if sz <= 3 {
        return v[sz];
    };

    for i in 4..sz + 1 {
        v[i] = v[i - 1] + v[i - 2] + v[i - 3];
    }
    v[sz]
}

fn main() {
    let mut n = String::new();
    io::stdin().read_line(&mut n).unwrap();
    let sz: usize = n.trim().parse().unwrap();

    for _i in 0..sz {
        let mut tmp = String::new();
        io::stdin().read_line(&mut tmp).unwrap();
        let check_n: usize = tmp.trim().parse().unwrap();
        let res = down_top(check_n);
        println!("{}", res);
    }
}
