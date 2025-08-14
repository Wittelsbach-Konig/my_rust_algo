pub fn nth(n: u32) -> u32 {
    let mut count = 0;
    let mut num = 2;
    while count < n {
        num +=1;
        if is_prime(num) {
            count += 1
        }
    }
    num
}

fn is_prime(n: u32) -> bool {
    match n {
        2 => true,
        n if n < 2 || n & 1 == 0 => false,
        _ => {
            let n_sqrt = n.isqrt();
            (3..=n_sqrt).step_by(2).all(|i| n % i != 0)
        }
    }
}
               