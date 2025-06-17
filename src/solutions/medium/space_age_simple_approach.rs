// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

const ONE_EARTH_YEAR_IN_SECONDS: u64 = 31_557_600;

#[derive(Debug)]
pub struct Duration {
    seconds: u64,
}

impl From<u64> for Duration {
    fn from(s: u64) -> Self {
        Duration { seconds: s }
    }
}

pub trait Planet {
    fn orbital_period_in_earth_years() -> f64;
    fn years_during(d: &Duration) -> f64 {
        (d.seconds as f64 / ONE_EARTH_YEAR_IN_SECONDS as f64)
            / Self::orbital_period_in_earth_years()
    }
}

pub struct Mercury;
pub struct Venus;
pub struct Earth;
pub struct Mars;
pub struct Jupiter;
pub struct Saturn;
pub struct Uranus;
pub struct Neptune;

impl Planet for Mercury {
    fn orbital_period_in_earth_years() -> f64 {
        0.2408467
    }
}
impl Planet for Venus {
    fn orbital_period_in_earth_years() -> f64 {
        0.61519726
    }
}
impl Planet for Earth {
    fn orbital_period_in_earth_years() -> f64 {
        1.0
    }
}
impl Planet for Mars {
    fn orbital_period_in_earth_years() -> f64 {
        1.8808158
    }
}
impl Planet for Jupiter {
    fn orbital_period_in_earth_years() -> f64 {
        11.862615
    }
}
impl Planet for Saturn {
    fn orbital_period_in_earth_years() -> f64 {
        29.447498
    }
}
impl Planet for Uranus {
    fn orbital_period_in_earth_years() -> f64 {
        84.016846
    }
}
impl Planet for Neptune {
    fn orbital_period_in_earth_years() -> f64 {
        164.79132
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_in_delta(expected: f64, actual: f64) {
        let diff: f64 = (expected - actual).abs();
        let delta: f64 = 0.01;
        if diff > delta {
            panic!(
                "Your result of {actual} should be within {delta} of the expected result {expected}"
            )
        }
    }

    #[test]
    fn age_on_earth() {
        let seconds = 1_000_000_000;
        let duration = Duration::from(seconds);
        let output = Earth::years_during(&duration);
        let expected = 31.69;
        assert_in_delta(expected, output);
    }
    #[test]

    fn age_on_mercury() {
        let seconds = 2_134_835_688;
        let duration = Duration::from(seconds);
        let output = Mercury::years_during(&duration);
        let expected = 280.88;
        assert_in_delta(expected, output);
    }
    #[test]

    fn age_on_venus() {
        let seconds = 189_839_836;
        let duration = Duration::from(seconds);
        let output = Venus::years_during(&duration);
        let expected = 9.78;
        assert_in_delta(expected, output);
    }
    #[test]

    fn age_on_mars() {
        let seconds = 2_129_871_239;
        let duration = Duration::from(seconds);
        let output = Mars::years_during(&duration);
        let expected = 35.88;
        assert_in_delta(expected, output);
    }
    #[test]

    fn age_on_jupiter() {
        let seconds = 901_876_382;
        let duration = Duration::from(seconds);
        let output = Jupiter::years_during(&duration);
        let expected = 2.41;
        assert_in_delta(expected, output);
    }
    #[test]

    fn age_on_saturn() {
        let seconds = 2_000_000_000;
        let duration = Duration::from(seconds);
        let output = Saturn::years_during(&duration);
        let expected = 2.15;
        assert_in_delta(expected, output);
    }
    #[test]

    fn age_on_uranus() {
        let seconds = 1_210_123_456;
        let duration = Duration::from(seconds);
        let output = Uranus::years_during(&duration);
        let expected = 0.46;
        assert_in_delta(expected, output);
    }
    #[test]
    fn age_on_neptune() {
        let seconds = 1_821_023_456;
        let duration = Duration::from(seconds);
        let output = Neptune::years_during(&duration);
        let expected = 0.35;
        assert_in_delta(expected, output);
    }

    #[test]
    fn test_orbital_periods() {
        assert_in_delta(Mercury::orbital_period_in_earth_years(), 0.240846);
        assert_in_delta(Venus::orbital_period_in_earth_years(), 0.615197);
        assert_in_delta(Earth::orbital_period_in_earth_years(), 1.0);
        assert_in_delta(Mars::orbital_period_in_earth_years(), 1.880816);
        assert_in_delta(Jupiter::orbital_period_in_earth_years(), 11.862616);
        assert_in_delta(Saturn::orbital_period_in_earth_years(), 29.447499);
        assert_in_delta(Uranus::orbital_period_in_earth_years(), 84.016);
        assert_in_delta(Neptune::orbital_period_in_earth_years(), 164.79132);
    }
}
