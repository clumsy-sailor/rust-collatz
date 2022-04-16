use std::thread;

fn collatz_next(n: u64) -> u64 {
    if n % 2 == 0
        { n / 2}
    else { 3 * n + 1}
}

pub fn collatz_height(mut n: u64) -> u64 {
    let mut h = 0u64;
    while n != 1 {
        n = collatz_next(n);
        h += 1;
    }
    h
}

pub fn collatz_longest_seq_identical_heights(n_begin: u64, n_end: u64) -> (u64, u64) {
    // Output: (n_min, longest_subseq_length).
    let mut n_min = 0u64;
    let mut longest_subseq_length = 0u64; 
    let mut subseq_length = 1u64;
    let mut inside_subseq = false;
    let mut len1 = 1u64;
    let mut len2: u64;
    for n in n_begin..n_end {
        len2 = collatz_height(n);
        if len2 == len1 { // Start of contiguous subsequence identified.
            inside_subseq = true;
            subseq_length += 1;
            //len1 = len2;
            //continue;
        }
        else {
            if inside_subseq { // End of contiguous subsequence identified.
                inside_subseq = false;
                if subseq_length > longest_subseq_length {
                    longest_subseq_length = subseq_length;
                    n_min = n - subseq_length;
                }
                subseq_length = 1u64;
            }
        }
        len1 = len2;
    }
    (n_min, longest_subseq_length)
}

pub fn runner_par(n_begin: u64, n_end: u64) -> (u64, u64) {
    const N_THREADS: u64 = 8;
    let k: u64 = (n_end - n_begin) / N_THREADS;
    let mut th_handles = Vec::new();
    let mut result = (0u64, 0u64);
    for i in 0..N_THREADS {
        let n1: u64 = n_begin + i * k;
        let n2: u64 = n_begin + (i + 1) * k;
        th_handles.push(
            thread::spawn(move|| collatz_longest_seq_identical_heights(n1, n2))
        );
    }
    for th_handle in th_handles {
        let part_res = th_handle.join().unwrap();
        result = my_max(result, part_res);
    }

    result
}

fn my_max(r1: (u64, u64), r2: (u64, u64)) -> (u64, u64) {
    if r1.1 > r2.1 { r1 } else { r2 }
}
