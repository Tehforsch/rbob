use uom::si::f64::Time;
use uom::si::time::{second, year};

pub fn nice_time(time: Time) -> String {
    let one_year = Time::new::<year>(1.0);
    let kiloyear = Time::new::<year>(1e3);
    let megayear = Time::new::<year>(1e6);
    let units_to_try = [("yr", one_year), ("kyr", kiloyear), ("Myr", megayear)];
    for (name, unit) in units_to_try.iter() {
        let value: f64 = (time / *unit).value;
        if value >= 1.0 && value < 1000.0 {
            return format!("{} {}", value, name);
        }
    }
    let seconds = Time::new::<second>(1.0);
    format!("{} {}", (time / seconds).value, "s")
}
