// Operations based around amicable numbers
// Suports u32 but should be interchangable with other types
// Wikipedia reference: https://en.wikipedia.org/wiki/Amicable_numbers

// Returns vec of amicable pairs below N
// N must be positive

pub fn amicable_nums(n: u32) -> Option<Vec<(u32, u32)>> {
    let mut factor_sum = vec![0; n as usize];

    // Make a list of the sum of the factors of each number below N
    for i in 1..n{
        for j in (i * 2..n).step_by(i as usize) {
            factor_sum[j as usize] += i;
        }
    }

    // Default value of (0, 0) if no pairs are found
    let mut out = vec![(0, 0)];

    // check if the numbers are amicable
    for (i, x) in factor_sum.iter().enumerate() {
        if (*x < n) && (factor_sum[*x as usize] == i as u32) && (*x > i as u32) {
            out.push((i as u32, *x));
        }
    }

    // check if anything was added to the vector, if so remove the (0, 0) from the vec
    if out.len() == 1 {
        None 
    } else {
        out.remove(0);
        Some(out)
    }
}