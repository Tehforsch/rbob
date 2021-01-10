use anyhow::Result;
use plotters_bitmap::bitmap_pixel::RGBPixel;

use self::post_expansion::ExpansionFn;
use self::post_slice::SliceFn;
use crate::config;
use crate::sim_params::SimParams;
use crate::sim_set::SimSet;
use crate::util::get_files;
use snapshot::Snapshot;
use std::{fs, path::PathBuf};

pub mod axis;
pub mod post_expansion;
pub mod post_slice;
pub mod read_hdf5;
pub mod snapshot;

use clap::Clap;
use plotters::prelude::*;
use plotters::{chart::ChartBuilder, coord::Shift};

#[derive(Clap, Debug)]
pub enum PostFnName {
    Expansion(ExpansionFn),
    Slice(SliceFn),
    // Shadowing(ShadowingType),
}

impl std::fmt::Display for PostFnName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = match self {
            Self::Expansion(_) => "expansion",
            Self::Slice(_) => "slice",
        };
        write!(f, "{}", name)
    }
}

pub trait SnapPostFn {
    type Output;
    fn post(&self, sim: &SimParams, snap: &Snapshot) -> Result<Self::Output>;
    fn plot(
        &self,
        chartbuilder: &mut ChartBuilder<BitMapBackend<RGBPixel>>,
        result: &Self::Output,
    ) -> Result<()>;

    fn run_on_sim_snap(
        &self,
        sim: &SimParams,
        snap: &Snapshot,
        mut chartbuilder: &mut ChartBuilder<BitMapBackend<RGBPixel>>,
    ) -> Result<()> {
        let res = self.post(sim, snap)?;
        self.plot(&mut chartbuilder, &res)
    }
}

pub trait SimPostFn {
    type Output;
    fn post(&self, sim: &SimParams) -> Result<Self::Output>;
    fn plot(
        &self,
        chartbuilder: &mut ChartBuilder<BitMapBackend<RGBPixel>>,
        result: &Self::Output,
    ) -> Result<()>;

    fn run_on_sim(
        &self,
        sim: &SimParams,
        mut chartbuilder: &mut ChartBuilder<BitMapBackend<RGBPixel>>,
    ) -> Result<()> {
        let res = self.post(sim)?;
        self.plot(&mut chartbuilder, &res)
    }
}

pub fn postprocess_sim_set(sim_set: &SimSet, function: PostFnName) -> Result<()> {
    for sim in sim_set.iter() {
        let pic_folder = create_pic_folder_if_nonexistent(sim)?;
        let image_name = get_image_name(&pic_folder, &function.to_string());
        let root = get_drawing_area(&image_name)?;
        match function {
            PostFnName::Expansion(ref l) => l.run_on_sim(sim, &mut ChartBuilder::on(&root))?,
            _ => {}
        };
    }
    for sim in sim_set.iter() {
        let pic_folder = create_pic_folder_if_nonexistent(sim)?;
        for mb_snap in get_snapshots(sim)? {
            let snap = mb_snap?;
            let image_name = get_image_name(
                &pic_folder,
                &format!("{}_{}", snap.to_string(), function.to_string()),
            );
            let root = get_drawing_area(&image_name)?;
            match function {
                PostFnName::Slice(ref l) => {
                    l.run_on_sim_snap(sim, &snap, &mut ChartBuilder::on(&root))?
                }
                _ => {}
            }
        }
    }
    Ok(())
}

fn get_drawing_area(
    image_name: &std::path::Path,
) -> Result<DrawingArea<BitMapBackend<RGBPixel>, Shift>> {
    let root = BitMapBackend::new(image_name, (1024, 768)).into_drawing_area();
    root.fill(&WHITE)?;
    Ok(root)
}

fn create_pic_folder_if_nonexistent(sim: &SimParams) -> Result<PathBuf> {
    let folder = sim.get_pic_folder();
    if !folder.is_dir() {
        fs::create_dir_all(&folder)?;
    }
    Ok(folder.to_owned())
}

fn get_image_name(pic_folder: &std::path::Path, post_fn_name: &str) -> PathBuf {
    return pic_folder
        .join(format!("{}.{}", post_fn_name, config::PIC_FILE_ENDING))
        .to_owned();
}

pub fn get_snapshots<'a>(
    sim: &'a SimParams,
) -> Result<Box<dyn Iterator<Item = Result<Snapshot<'a>>> + 'a>> {
    Ok(Box::new(get_snapshot_files(sim)?.map(move |snap_file| {
        Snapshot::from_file(sim, &snap_file)
    })))
}

pub fn get_snapshot_files(sim: &SimParams) -> Result<Box<dyn Iterator<Item = PathBuf>>> {
    Ok(Box::new(
        get_files(&sim.output_folder())?.into_iter().filter(|f| {
            f.extension()
                .map(|ext| ext.to_str().unwrap() == "hdf5")
                .unwrap_or(false)
        }),
    ))
}
