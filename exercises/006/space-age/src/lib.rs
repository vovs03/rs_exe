// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

#[derive(Debug)]
pub struct Duration(f64);

impl From<u64> for Duration {
    fn from(s: u64) -> Self {
        // todo!("s, measured in seconds: {s}")
        Duration(s as f64 / 31557600.0)
    }
}

pub trait Planet {    
    fn orbital() -> f64 { 1.0 }
    
    // todo!("convert a duration ({d:?}) to the number of years on this planet for that duration");
    fn years_during(d: &Duration) -> f64 {
    	 d.0 / Self::orbital()
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

impl Planet for Mercury { fn orbital() -> f64 { 0.2408467 }}
impl Planet for Venus { fn orbital() -> f64 { 0.61519726}}
impl Planet for Earth { fn orbital() -> f64 { 1.0 }}
impl Planet for Mars { fn orbital() -> f64 { 1.8808158 }}
impl Planet for Jupiter { fn orbital() -> f64 { 11.862615 }}
impl Planet for Saturn { fn orbital() -> f64 { 29.447498 }}
impl Planet for Uranus { fn orbital() -> f64 { 84.016846 }}
impl Planet for Neptune { fn orbital() -> f64 { 164.79132 }}
