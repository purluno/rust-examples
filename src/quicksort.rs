use readfile;

pub fn run() {
    let mut a = readfile::i64_vector("quicksort.txt");
    println!("before sort: {:?}", a);
    quicksort(&mut a);
    println!("after sort: {:?}", a);
}

fn quicksort(arr: &mut [i64]) {
    let len = arr.len();
    if len > 2 {
        let p = partition(arr);
        quicksort(&mut arr[0..p+1]);
        quicksort(&mut arr[p+1..len]);
    }
}

fn partition(arr: &mut [i64]) -> usize {
    let pivot = arr[arr.len() / 2];
    let mut a = 0;
    let mut b = arr.len() - 1;
    loop {
        while arr[a] < pivot { a += 1 }
        while arr[b] > pivot { b -= 1 }
        if a >= b {
            return b;
        }
        arr.swap(a, b);
        a += 1; b -= 1;
    }
}
