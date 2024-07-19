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
}
