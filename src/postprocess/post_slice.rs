use super::SnapPostFn;
use crate::sim_params::SimParams;

use super::axis::Axis;
use super::snapshot::Snapshot;
use crate::array_utils::meshgrid2;
use anyhow::Result;
use clap::Clap;
use ndarray::{array, s};
use plotters::chart::ChartBuilder;
use plotters::prelude::*;
use plotters_bitmap::bitmap_pixel::RGBPixel;

pub struct SliceResult {}

#[derive(Clap, Debug)]
pub struct SliceFn {
    axis: Axis,
}

impl SnapPostFn for &SliceFn {
    type Output = SliceResult;

    fn post(&self, sim: &SimParams, snap: &Snapshot) -> Result<Self::Output> {
        let coords = snap.coordinates()?;
        let dens = snap.density()?;
        let h_plus_abundance = snap.h_plus_abundance()?;
        let n0 = 8;
        let n1 = 6;
        let grid = meshgrid2(snap.min_extent(), snap.max_extent(), n0, n1);
        let axis = self.axis.get_axis_vector();
        let (orth1, orth2) = self.axis.get_orthogonal_vectors();
        let center = snap.center();
        for i0 in 0..n0 {
            for i1 in 0..n1 {
                let pos2d = grid.slice(s![i0, i1, ..]);
            }
        }
        todo!()
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
