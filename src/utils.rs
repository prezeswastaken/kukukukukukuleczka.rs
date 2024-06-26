use anyhow::anyhow;

pub fn clean_storage(storage_path: String) -> anyhow::Result<()> {
    let paths = std::fs::read_dir(&storage_path)?;
    for path in paths {
        let path = path?.path();
        if path.is_file() {
            let file_name = path
                .file_name()
                .ok_or_else(|| anyhow!("Couldn't get file name"))?
                .to_str()
                .ok_or_else(|| anyhow!("Couldn't convert file name to string"))?;
            if !file_name.ends_with(".pdf") {
                std::fs::remove_file(&path)?;
            }
        }
    }

    Ok(())
}

extern crate ndarray;
extern crate ndarray_rand;

use ndarray::prelude::*;
use ndarray_rand::rand_distr::Uniform;
use ndarray_rand::RandomExt;
use std::f64;

fn sigmoid(x: f64) -> f64 {
    1.0 / (1.0 + f64::exp(-x))
}

pub fn propagation(inputs: Array1<f64>) -> Array1<f64> {
    let weights = array![[23.71784994], [27.84819623], [0.94665866], [11.42126197]];
    let weights = Array2::from(weights);
    let dot_product = inputs.dot(&weights);
    dot_product.mapv(|x| sigmoid(x))
}
