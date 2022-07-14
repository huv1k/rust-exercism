// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

#[derive(Debug)]
pub struct Duration {
    pub value: f64,
}

impl From<u64> for Duration {
    fn from(s: u64) -> Self {
        Self { value: s as f64 }
    }
}

pub trait Planet {
    const EARTH_YEAR_IN_SECONDS: f64 = 31_557_600.0;
    const EARTH_YEAR_RATIO: f64;

    fn years_during(d: &Duration) -> f64 {
        d.value / Self::EARTH_YEAR_IN_SECONDS / Self::EARTH_YEAR_RATIO
    }
}

macro_rules! planets {
    ($($name: ident => $value: expr),+) => {
        $(
            pub struct $name;
            impl Planet for $name {
                const EARTH_YEAR_RATIO: f64 = $value;
            }
        )*
    };
}

planets!(
    Mercury => 0.2408467,
    Venus => 0.61519726,
    Earth => 1.0,
    Mars => 1.8808158,
    Jupiter => 11.862615,
    Saturn => 29.447498,
    Uranus => 84.016846,
    Neptune => 164.79132
);
