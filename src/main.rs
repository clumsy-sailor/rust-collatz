mod collatz;

use std::time::Instant;
fn main() {
    let n_begin = 1_000_000_000u64;
    let n_end  = 10_000_000_000u64;

    let t_start = Instant::now();

    let (n_min, longest_subseq_length) =
    //     collatz::collatz_longest_seq_identical_heights(n_begin, n_end);
        collatz::runner_par(n_begin, n_end);

    let et = t_start.elapsed().as_secs_f32();
    
    println!("\nJob ended.");
    println!("{} {}", n_min, longest_subseq_length);
    println!("{}", et);
}
