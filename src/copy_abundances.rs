use anyhow::Result;
use camino::Utf8Path;
use kdtree::distance::squared_euclidean;
use kdtree::KdTree;
use ndarray::Array;
use ndarray::ArrayBase;
use ndarray::OwnedRepr;

use crate::array_utils::FArray1;
use crate::array_utils::FArray2;
use crate::config::SWEEP_NFREQ;
use crate::postprocess::get_snapshots;
use crate::postprocess::snapshot::Snapshot;
use crate::sim_params::SimParams;
use crate::sim_params::SimParamsKind;
use crate::util::copy_file;

pub fn copy_abundances(
    abundances_sim: &Utf8Path,
    coordinates_sim_path: &Utf8Path,
    snap_output: &Utf8Path,
) -> Result<()> {
    let abundances_sim = SimParams::from_folder(abundances_sim, SimParamsKind::Output)?;
    let coordinates_sim = SimParams::from_folder(coordinates_sim_path, SimParamsKind::Output)?;
    let coordinates_snap_path = coordinates_sim_path.join(&coordinates_sim.get_ics_filename());
    let coordinates_snap = Snapshot::from_file(&coordinates_sim, &coordinates_snap_path)?;
    let abundances_snap = last(get_snapshots(&abundances_sim)?).unwrap()?;
    let (result_abundances, result_energies) =
        get_remapped_abundances_and_energies(abundances_snap, coordinates_snap)?;
    copy_file(coordinates_snap_path, snap_output)?;
    let h5file = hdf5::File::open_rw(snap_output)?;
    write_dataset(&h5file, "PartType0/ChemicalAbundances", result_abundances)?;
    write_dataset(&h5file, "PartType0/InternalEnergy", result_energies)?;
    Ok(())
}

fn write_dataset<D: ndarray::Dimension>(
    h5file: &hdf5::File,
    name: &str,
    data: ArrayBase<OwnedRepr<f64>, D>,
) -> Result<()> {
    let dataset = h5file.dataset(&name);
    match dataset {
        Ok(dataset) => dataset.write(&data)?,
        Err(_) => {
            h5file
                .new_dataset_builder()
                .with_data(data.view())
                .create(name)?;
        }
    }
    Ok(())
}

fn get_remapped_abundances_and_energies<'a>(
    abundances_snap: Snapshot<'a>,
    coordinates_snap: Snapshot<'a>,
) -> Result<(FArray2, FArray1)> {
    let reference_coords = abundances_snap.coordinates()?;
    let mut tree = KdTree::new(3);
    let reference_coords_iter = reference_coords.outer_iter().map(|x| [x[0], x[1], x[2]]);
    for (i, coord) in reference_coords_iter.enumerate() {
        tree.add(coord, i)?;
    }
    let coords = coordinates_snap.coordinates()?;
    let shape = coords.shape();
    let coords_iter = coords.outer_iter().map(|x| [x[0], x[1], x[2]]);
    let reference_abundances = abundances_snap.chemical_abundances()?;
    let reference_energies = abundances_snap.energies()?;
    let mut result_energies = coordinates_snap.energies()?;
    let mut result_abundances = Array::zeros((shape[0], SWEEP_NFREQ));
    for (i, pos) in coords_iter.enumerate() {
        let (_, index) = tree.nearest(&pos, 1, &squared_euclidean).unwrap()[0];
        for j in 0..SWEEP_NFREQ {
            result_abundances[[i, j]] = reference_abundances[[*index, j]];
        }
        if reference_energies[[*index]] > result_energies[[i]] {
            result_energies[[i]] = reference_energies[[*index]];
        }
    }
    Ok((result_abundances, result_energies))
}

fn last<T>(iter: impl Iterator<Item = T>) -> Option<T> {
    let mut result = None;
    for item in iter {
        result = Some(item)
    }
    return result;
}
