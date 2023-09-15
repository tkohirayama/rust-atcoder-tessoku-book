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

    cost[1] = a[0];
    for i in 2..n {
        let route_a = cost[i - 1] + a[i - 1];
        let route_b = cost[i - 2] + b[i - 2];
        cost[i] = if route_a <= route_b { route_a } else { route_b }
    }

    println!("{}", cost[n-1]);
}

// fn dynamic_programming(
//     a: &Vec<u32>,
//     b: &Vec<u32>,
//     cache: &mut Vec<u32>,
//     from: usize,
//     to: usize,
//     goal: usize,
// ) -> u32 {
//     if cache[from] > 0 {
//         cache[from];
//     }

//     let distance = goal - to;
//     let cost = if distance == 0 {
//         0
//     } else if distance == 1 {
//         dynamic_programming(a, b, cache, to, to + 1, goal)
//     } else {
//         let route_a_cost = dynamic_programming(a, b, cache, to, to + 1, goal);
//         let route_b_cost = dynamic_programming(a, b, cache, to, to + 2, goal);
//         if route_a_cost > route_b_cost {
//             route_b_cost
//         } else {
//             route_a_cost
//         }
//     };

//     let diff = to - from;
//     if diff == 1 {
//         cache[from] = cost + a[from];
//     } else {
//         cache[from] = cost + b[from];
//     }
//     cache[from]
// }
