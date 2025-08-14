pub fn factors(n: u64) -> Vec<u64> {
    let mut factors: Vec<u64> = Vec::new();
    let mut _n = n;

    while _n & 1 == 0 {
        factors.push(2);
        _n = _n / 2;
    }
    for i in (3..=_n.isqrt()).step_by(2) {
        while _n % i == 0 {
            factors.push(i);
            _n = _n / i;
        }
    }
    if _n > 2 {
        factors.push(_n);
    }
    factors
}
