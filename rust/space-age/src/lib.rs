// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

#[derive(Debug)]
pub struct Duration(f64);
const SECS_IN_EARTH_YEARS: f64 = 31_557_600.0;

impl From<u64> for Duration {
    fn from(s: u64) -> Self {
        Self(s as f64)
    }
}

pub trait Planet {
    const RELATIVE_EARTH_YEARS: f64;

    fn years_during(d: &Duration) -> f64 {
        d.0 / Self::RELATIVE_EARTH_YEARS
    }
}

macro_rules! solar_system {
    ($($planet:ident === $period:literal),+) => {
        $(
            pub struct $planet;

            impl Planet for $planet {
                const RELATIVE_EARTH_YEARS : f64 = $period * SECS_IN_EARTH_YEARS;
            }
        )+
    };
}

solar_system! {
    Mercury === 0.2408467,
    Venus === 0.61519726,
    Earth === 1.0,
    Mars === 1.8808158,
    Jupiter === 11.862615,
    Saturn === 29.447498,
    Uranus === 84.016846,
    Neptune === 164.79132
}
