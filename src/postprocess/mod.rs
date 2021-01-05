use anyhow::Result;
use boblib::config;
use plotters_bitmap::bitmap_pixel::RGBPixel;

use crate::args::PostFnName;
use crate::sim_params::SimParams;
use crate::sim_set::SimSet;
use crate::util::get_files;
use snapshot::Snapshot;
use std::{fs, path::PathBuf};

pub mod post_expansion;
pub mod read_hdf5;
pub mod snapshot;

use plotters::chart::ChartBuilder;
use plotters::prelude::*;

pub trait PostFn {
    const NAME: &'static str;
    type Output;
    fn post(&self, sim: &SimParams) -> Result<Self::Output>;
    fn plot(
        &self,
        chartbuilder: &mut ChartBuilder<BitMapBackend<RGBPixel>>,
        result: &Self::Output,
    ) -> Result<()>;

    fn get_name(&self) -> &'static str {
        Self::NAME
    }
}

pub fn postprocess_sim_set(sim_set: &SimSet, function: PostFnName) -> Result<()> {
    let post_fn = match function {
        PostFnName::Expansion(ref l) => l,
    };
    for sim in sim_set.iter() {
        println!("{:?}", &function);
        let res = post_fn.post(sim)?;
        let pic_folder = create_pic_folder_if_nonexistent(sim)?;
        let image_name = &get_image_name(&pic_folder, post_fn);
        let root = BitMapBackend::new(&image_name, (1024, 768)).into_drawing_area();
        root.fill(&WHITE)?;
        let mut chart = ChartBuilder::on(&root);
        post_fn.plot(&mut chart, &res)?;
    }
    Ok(())
}

fn create_pic_folder_if_nonexistent(sim: &SimParams) -> Result<PathBuf> {
    let folder =
        std::path::Path::new("pics").join(sim.folder.file_name().unwrap().to_str().unwrap());
    if !folder.is_dir() {
        fs::create_dir_all(&folder)?;
    }
    Ok(folder.to_owned())
}

fn get_image_name(pic_folder: &std::path::Path, post_fn: impl PostFn) -> PathBuf {
    return pic_folder
        .join(format!(
            "{}.{}",
            post_fn.get_name(),
            config::PIC_FILE_ENDING
        ))
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
