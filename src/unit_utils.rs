use uom::si::f64::Time;
use uom::si::time::second;
use uom::si::time::year;

pub fn nice_time(time: Time) -> String {
    let seconds = Time::new::<second>(1.0);
    let one_year = Time::new::<year>(1.0);
    let kiloyear = Time::new::<year>(1e3);
    let megayear = Time::new::<year>(1e6);
    let units_to_try = [
        ("s", seconds),
        ("yr", one_year),
        ("kyr", kiloyear),
        ("Myr", megayear),
    ];
    let (_, (name, value)) = units_to_try
        .iter()
        .map(|(name, unit)| (name, (time / *unit).value))
        .enumerate()
        .filter(|(i, (_, value))| (1.0..1000.0).contains(value) || *i == units_to_try.len() - 1)
        .next()
        .unwrap();
    format!("{} {}", value, name)
}
