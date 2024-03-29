use std::fs::{self};
use std::io::Cursor;

use anyhow::Result;
use byteorder::LittleEndian;
use byteorder::ReadBytesExt;
use camino::Utf8Path;
use serde::Deserialize;
use serde::Serialize;
use uom::si::f64::Frequency;
use uom::si::frequency::hertz;

use crate::sim_params::SimParams;

#[derive(Serialize, Deserialize, Debug)]
pub struct Source {
    pub pos: Vec<f64>,
    pub rates: Vec<f64>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SourceFile {
    pub sources: Vec<Source>,
}

impl SourceFile {
    pub fn get_rate(&self, source_file_index: usize, index: usize) -> Frequency {
        Frequency::new::<hertz>(self.sources[source_file_index].rates[index] as f64)
    }

    pub fn read(path: &Utf8Path) -> Result<Self> {
        let bytes_buffer: Vec<u8> = fs::read(path)?;
        let mut reader = Cursor::new(bytes_buffer);
        let n_sigma = reader.read_i32::<LittleEndian>().unwrap();
        let n_energy = reader.read_i32::<LittleEndian>().unwrap();
        let n_sources = reader.read_i32::<LittleEndian>().unwrap();
        let n_rates = reader.read_i32::<LittleEndian>().unwrap();
        let mut next_float = move || reader.read_f64::<LittleEndian>().unwrap();
        let mut sources = vec![];
        for _ in 0..n_sources {
            let pos_x = next_float();
            let pos_y = next_float();
            let pos_z = next_float();
            let _sigmas: Vec<f64> = (0..n_sigma).map(|_| next_float()).collect();
            let _energies: Vec<f64> = (0..n_energy).map(|_| next_float()).collect();
            let rates: Vec<f64> = (0..n_rates).map(|_| next_float()).collect();
            sources.push(Source {
                pos: vec![pos_x, pos_y, pos_z],
                rates,
            })
        }
        Ok(Self { sources })
    }

    pub fn from_params(sim: &SimParams) -> SourceFile {
        Self {
            sources: vec![Source {
                pos: vec![
                    sim.get("TestSourcePosX").unwrap().unwrap_f64(),
                    sim.get("TestSourcePosY").unwrap().unwrap_f64(),
                    sim.get("TestSourcePosZ").unwrap().unwrap_f64(),
                ],
                rates: vec![
                    sim.get("TestSourceRate056").unwrap().unwrap_f64(),
                    sim.get("TestSourceRate112").unwrap().unwrap_f64(),
                    sim.get("TestSourceRate136").unwrap().unwrap_f64(),
                    sim.get("TestSourceRate152").unwrap().unwrap_f64(),
                    sim.get("TestSourceRate246").unwrap().unwrap_f64(),
                ],
            }],
        }
    }
}
