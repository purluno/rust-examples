use readfile;

pub fn run() {
    let arr = readfile::i64_vector("binsearch.txt");
    let arr = arr.as_slice();
    println!("array = {:?}", arr);
    if let (Some(a), Some(b)) = (arr.iter().min(), arr.iter().max()) {
        for i in a-1..b+2 {
            println!("binsearch(&arr, {}) = {:?}", i, binsearch(&arr, i));
        }
    }
}

fn binsearch(arr: &[i64], v: i64) -> Option<i64> {
    let mut a: i64 = 0;
    let mut b: i64 = arr.len() as i64 - 1;
    while a <= b {
        let m = (a + b) / 2;
        let mv = arr[m as usize];
        if mv == v {
            return Some(m);
        } else if mv > v {
            b = m - 1;
        } else {
            a = m + 1;
        }
    }
    None
}
