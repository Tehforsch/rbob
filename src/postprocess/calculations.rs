use std::f64::consts::PI;

use uom::{
    si::{
        f64::{Frequency, Length, Mass, Time},
        length::centimeter,
        mass::gram,
        time::second,
    },
    typenum::{P2, P3},
};

use anyhow::Result;

use super::snapshot::Snapshot;

pub fn get_recombination_time(snap: &Snapshot) -> Result<Time> {
    let h = snap.get_header_attribute("HubbleParam", 1.0).unwrap();
    let redshift = snap.get_header_attribute("Redshift", 1.0).unwrap();
    let density_previous = (snap.sim.units.mass / snap.sim.units.length.powi(P3::new()))
        * ((redshift + 1.0).powi(3) * h.powi(2));
    let proton_mass = Mass::new::<gram>(1.672623e-24);
    let density_to_number_density = 1.0 / proton_mass;
    let mean_density = snap.density()?.mean().unwrap();
    dbg!(mean_density);
    let number_density_hydrogen = mean_density * density_to_number_density * density_previous;
    let alpha_b =
        2.59e-13 * Length::new::<centimeter>(1.0).powi(P3::new()) / Time::new::<second>(1.0);
    let recombination_time = 1.0 / (alpha_b * number_density_hydrogen);
    Ok(recombination_time)
}

pub fn get_stroemgren_radius(snap: &Snapshot, photon_rate: Frequency) -> Result<Length> {
    let h = get_hubble_param(snap);
    let redshift = get_redshift(snap);
    let density_previous = (snap.sim.units.mass / snap.sim.units.length.powi(P3::new()))
        * ((redshift + 1.0).powi(3) * h.powi(2));
    let proton_mass = Mass::new::<gram>(1.672623e-24);
    let density_to_number_density = 1.0 / proton_mass;
    let mean_density = snap.density()?.mean().unwrap();
    let number_density_hydrogen = mean_density * density_to_number_density * density_previous;
    dbg!(density_to_number_density, mean_density,number_density_hydrogen);
    let alpha_b =
        2.59e-13 * Length::new::<centimeter>(1.0).powi(P3::new()) / Time::new::<second>(1.0);
    let number_density_electron = number_density_hydrogen;
    let stroemgren_radius =
        (3.0 * photon_rate / (4.0 * PI * alpha_b * number_density_electron.powi(P2::new()))).cbrt();
    Ok(stroemgren_radius)
}

pub fn get_hubble_param(snap: &Snapshot) -> f64 {
    snap.get_header_attribute("HubbleParam", 1.0).unwrap()
}


pub fn get_redshift(snap: &Snapshot) -> f64 {
    snap.get_header_attribute("Redshift", 1.0).unwrap()
}
