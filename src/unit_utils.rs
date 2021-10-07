use uom::si::f64::Time;
use uom::si::time::second;
use uom::si::time::year;

pub fn nice_time(time: Time) -> String {
    let one_year = Time::new::<year>(1.0);
    let kiloyear = Time::new::<year>(1e3);
    let megayear = Time::new::<year>(1e6);
    let units_to_try = [("yr", one_year), ("kyr", kiloyear), ("Myr", megayear)];
    for (name, unit) in units_to_try.iter() {
        let value: f64 = (time / *unit).value;
        if (1.0..1000.0).contains(&value) {
            return format!("{} {}", value, name);
        }
    }
    let seconds = Time::new::<second>(1.0);
    format!("{} {}", (time / seconds).value, "s")
}
