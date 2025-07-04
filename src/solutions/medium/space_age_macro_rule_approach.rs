#[derive(Debug)]
pub struct Duration {
    seconds: u64,
}

impl From<u64> for Duration {
    fn from(value: u64) -> Self {
        Duration { seconds: value }
    }
}

pub trait Planet {
    const EARTH_YEAR_IN_SECONDS: f64 = 31_557_600.0;
    const EARTH_YEAR_RATIO: f64;
    fn years_during(d: &Duration) -> f64 {
        d.seconds as f64 / Self::EARTH_YEAR_IN_SECONDS / Self::EARTH_YEAR_RATIO
    }
}

macro_rules! init_planet {
    ($($t:ident => $e:expr), +) => {
        $(
            pub struct $t {}
            impl Planet for $t {
                const EARTH_YEAR_RATIO: f64 = $e;
                }
        )*
    };
}

init_planet!(
    Mercury => 0.2408467,
    Venus => 0.61519726,
    Earth => 1.0,
    Mars => 1.8808158,
    Jupiter => 11.862615,
    Saturn => 29.447498,
    Uranus => 84.016846,
    Neptune => 164.7913
);

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
}
