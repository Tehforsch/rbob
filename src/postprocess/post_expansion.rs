use super::get_snapshots;
// use super::snapshot::Snapshot;
use super::PostFn;
use crate::sim_params::SimParams;

use crate::unit_array::UArray1;
use anyhow::{Context, Result};
use clap::Clap;
use plotters::chart::ChartBuilder;
use plotters::prelude::*;
use plotters_bitmap::bitmap_pixel::RGBPixel;
use uom::si::{
    f64::{Ratio, Time},
    ratio::ratio,
};

#[derive(Clap, Debug)]
pub struct ExpansionFn {}

pub struct ExpansionResult {
    times: UArray1<Time>,
    mean_abundance: UArray1<Ratio>,
}

impl PostFn for &ExpansionFn {
    const NAME: &'static str = "expansion";
    type Output = ExpansionResult;
    fn post(&self, sim: &SimParams) -> Result<Self::Output> {
        let mut times: Vec<f64> = vec![];
        let mut mean_abundance: Vec<f64> = vec![];
        for mb_snap in get_snapshots(sim)? {
            dbg!(&mb_snap);
            let snap = mb_snap?;
            let coords = snap.coordinates()?;
            let dens = snap.density()?;
            let h_plus_abundance = snap.h_plus_abundance()?;
            times.push((snap.time / sim.units.time).value);
            mean_abundance.push(h_plus_abundance.mean().unwrap().value);
        }
        Ok(ExpansionResult {
            times: UArray1::from_vec(times, sim.units.time),
            mean_abundance: UArray1::from_vec(mean_abundance, Ratio::new::<ratio>(1.0)),
        })
    }

    fn plot(
        &self,
        chartbuilder: &mut ChartBuilder<BitMapBackend<RGBPixel>>,
        result: &Self::Output,
    ) -> Result<()> {
        let mut chart = chartbuilder
            // .top_x_label_area_size(40)
            // .y_label_area_size(40)
            .build_cartesian_2d(0i32..15i32, 15i32..0i32)?;
        println!("{}", &result.times);
        println!("{}", &result.mean_abundance);

        chart
            .configure_mesh()
            .x_labels(15)
            .y_labels(15)
            .x_label_offset(35)
            .y_label_offset(25)
            .disable_x_mesh()
            .disable_y_mesh()
            .label_style(("sans-serif", 20))
            .draw()?;

        let mut matrix = [[0; 15]; 15];

        for i in 0..15 {
            matrix[i][i] = i + 4;
        }

        chart.draw_series(
            matrix
                .iter()
                .zip(0..)
                .map(|(l, y)| l.iter().zip(0..).map(move |(v, x)| (x as i32, y as i32, v)))
                .flatten()
                .map(|(x, y, v)| {
                    Rectangle::new(
                        [(x, y), (x + 1, y + 1)],
                        HSLColor(
                            240.0 / 360.0 - 240.0 / 360.0 * (*v as f64 / 20.0),
                            0.7,
                            0.1 + 0.4 * *v as f64 / 20.0,
                        )
                        .filled(),
                    )
                }),
        )?;

        Ok(())
    }
}
