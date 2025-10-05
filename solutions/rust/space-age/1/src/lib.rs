// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

const SECONDS_IN_EARTH_YEAR: f64 = 365.25 * 24.0 * 3600.0;

const MERCURY: f64 = 0.2408467;
const VENUS: f64 = 0.61519726;
const MARS: f64 = 1.8808158;
const JUPITER: f64 = 11.862615;
const SATURN: f64 = 29.447498;
const URANUS: f64 = 84.016846;
const NEPTUNE: f64 = 164.79132;

#[derive(Debug)]
pub struct Duration {
    seconds: f64,
}

impl From<u64> for Duration {
    fn from(s: u64) -> Self {
        Duration { seconds: s as f64 }
    }
}

pub trait Planet {
    fn years_during(d: &Duration) -> f64 {
        todo!("convert a duration ({d:?}) to the number of years on this planet for that duration");
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
    fn years_during(d: &Duration) -> f64 {
        d.seconds / SECONDS_IN_EARTH_YEAR / MERCURY
    }
}

impl Planet for Venus {
    fn years_during(d: &Duration) -> f64 {
        d.seconds / SECONDS_IN_EARTH_YEAR / VENUS
    }
}

impl Planet for Earth {
    fn years_during(d: &Duration) -> f64 {
        d.seconds / SECONDS_IN_EARTH_YEAR
    }
}

impl Planet for Mars {
    fn years_during(d: &Duration) -> f64 {
        d.seconds / SECONDS_IN_EARTH_YEAR / MARS
    }
}

impl Planet for Jupiter {
    fn years_during(d: &Duration) -> f64 {
        d.seconds / SECONDS_IN_EARTH_YEAR / JUPITER
    }
}

impl Planet for Saturn {
    fn years_during(d: &Duration) -> f64 {
        d.seconds / SECONDS_IN_EARTH_YEAR / SATURN
    }
}

impl Planet for Uranus {
    fn years_during(d: &Duration) -> f64 {
        d.seconds / SECONDS_IN_EARTH_YEAR / URANUS
    }
}

impl Planet for Neptune {
    fn years_during(d: &Duration) -> f64 {
        d.seconds / SECONDS_IN_EARTH_YEAR / NEPTUNE
    }
}
