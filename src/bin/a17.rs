use proconio::input;

// A15 - Compression
// https://atcoder.jp/contests/tessoku-book/tasks/tessoku_book_o
fn main() {
    input! {
        n: usize,
        a: [u32; n-1],
        b: [u32; n-2]
    }

    let mut cost: Vec<u32> = vec![0; n];
    let mut from: Vec<usize> = vec![0; n];

    cost[1] = a[0];
    from[1] = 1;

    for i in 2..n {
        let route_a = cost[i - 1] + a[i - 1];
        let route_b = cost[i - 2] + b[i - 2];
        if route_a <= route_b {
            cost[i] = route_a;
            from[i] = i;
        } else {
            cost[i] = route_b;
            from[i] = i - 1;
        }
    }

    let route = route(&from, n);
    println!("{}", route.1);
    println!("{}", route.0);
}

fn route(from: &Vec<usize>, idx: usize) -> (String, usize) {
    if idx > 1 {
        let r = route(from, from[idx - 1]);
        (format!("{} {}", r.0, idx), r.1 + 1)
    } else {
        (1.to_string(), 1)
    }
}
