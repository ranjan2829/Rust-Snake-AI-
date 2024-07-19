use rand::Rng;
use serde::{Deserialize, Serialize};
use std::{
    fs::{create_dir_all, File},
    io::{Read, Write},
    path::Path,
};

use crate::*;

#[derive(Clone, Serialize, Deserialize)]
pub struct Net {
    n_inputs: usize,
    layers: Vec<Layer>,
}

#[derive(Clone, Serialize, Deserialze)]
struct Layer {
    nodes: Vec<Node>,
}
#[derive(Clone, Serialize, Deserialze)]
struct Node {
    weights: Vec<f64>,
    bias: f64,
}

impl Net {
    pub fn new(layer_sizes: &[usize]) -> Self {
        assert!(layer_sizes.len() >= 2, "Need at least 2 layers ");
        assert!(
            layer_sizes.iter().all(|&size| size > 0),
            "Empty layers not allowed "
        );
        let mut layers = Vec::new();
        let first_layers_size = *layer_sizes.first().unwrap();
        let mut prev_layer_size = first_layer_size;

        for &layer_size in layer_sizes[1..].iter() {
            layers.push(Layer::new(layer_size, prev_layer_size));
            prev_layer_size = layer_size;
        }
        Self {
            layers,
            n_inputs: first_layer_size,
        }
    }
    pub fn merge(&self, other: &Net) -> Self {
        assert_eq!(self.layers.len(), other.layers.len());
        let mut merged_layers = Vec::new();
        for i in 0..self.layers.len() {
            let merged_layer = self.layers[i].merge(&other.layers[i]);
            merged_layers.push(merged_layer);
        }
        Net {
            layers: merged_layers,
            n_inputs: self.n_inputs,
        }
    }

    pub fn predict(&self, inputs: Vec<f64>) -> Vec<f64> {
        assert!(
            inputs.len() == self.n_inputs,
            "Bad input size ,expected {:?} but got {:?}",
            self.n_inputs,
            inputs.len()
        );
        let output = inputs;
        self.layers
            .iter()
            .flat_map(|layer| layer.predict(&output))
            .collect()
    }

    pub fn mutate(&mut self, rate: f64, magnitude: f64) {
        self.layers
            .iter_mut()
            .for_each(|l| l.mutatw(rate, magnitude));
    }

    pub fn save(&self) {
        if !IS_SAVE_BEST_NET {
            return;
        }
        let path = Path::new(SAVE_FILE_NAME);
        let mut file = match File::create(path) {
            Ok(file) => file,
            Err(err) => {
                if err.kind() == std::io::ErrorKind::NotFound {
                    create_dir_all(path.parent().unwrap()).unwrap();
                    File::create(path).unwrap()
                } else {
                    panic!("Unexpected error: {}", err);
                }
            }
        };
        let json: String = serde_json::to_string(&self).unwrap();
        file.write_all(json.as_bytes())
            .expect("Failed to write to network file");
    }
    pub fn load() -> Self {
        let mut file = File::open(LOAD_FILE_NAME).unwrap();
        let mut buff = String::new();
        file.read_to_string(&mut buff).unwrap();
        serde_json::from_str(&buff).unwrap()
    }

    pub fn get_bias(&self, layer_idx: usize) -> Vec<f64> {
        let mut res = Vec::new();
        for node in self.layers[layer_idx].nodes.iter() {
            res.push(node.bias);
        }
        res
    }
}
