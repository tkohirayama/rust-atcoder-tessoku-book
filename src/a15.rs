use proconio::input;

// A15 - Compression
// https://atcoder.jp/contests/tessoku-book/tasks/tessoku_book_o
fn main() {
    input! {
        n: usize,
        a: [usize; n]
    }

    let mut a_vec = a.clone();
    a_vec.sort();
    a_vec.dedup();

    let mut compress = vec![0; n];
    for i in 0..n {
        let mut l: usize = 0;
        let mut r: usize = a_vec.len();
        while l < r {
            let m = (l + r) / 2;
            if a_vec[m] > a[i]
            {
                r = m;
            }
            else {
                l = m + 1;
            }
        }
        compress[i] = l;
    }

    print!(
        "{}",
        compress.iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join(" ")
    );
}
