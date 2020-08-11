#![allow(unused)]
use pyo3::prelude::*;
use std::collections::HashMap;
use pyo3::types::*;
use crate::figax::axes;
use crate::common::Env;


// #[derive( Default)]
pub struct Figure<'p, T> {
    py: Python<'p>,
    plt: &'p PyModule,
    subplots: Subplots<'p, T>,
}

impl<'p, T> Figure<'p, T> {
    pub fn new<'a: 'p>(env: &'a Env) -> Figure<'p, T> { 
        let python = env.gil.python();
        let plot = python.import("matplotlib.pyplot").unwrap();
        Figure {
            py: python,
            plt: plot,
            subplots: Subplots::initialise()
        }
    }

    pub fn add_empty_subplot(&'p mut self) -> &mut axes::Axes<'p, T> {
        // place holder function
        self.subplots.add_empty_subplot()
    }

    pub fn show(self) {
        //place holder function for the chain to pass to python
    }
}

// #[derive(Debug, Default)]
struct Subplots<'p, T> {
    axes: Vec<axes::Axes<'p, T>>,
}

impl<'p, T> Subplots<'p, T> {
    pub fn initialise() -> Subplots<'p, T> {
        Subplots {
            axes: vec![],
        }
    }

    pub fn add_empty_subplot(&'p mut self) -> &'p mut axes::Axes<'p, T> {
        //place holder function
        let mut new_axes = axes::Axes::empty();
        let previous_len = self.num_axes();
        self.axes.push(new_axes);

        &mut self.axes[previous_len]
    }

    fn num_axes(&self) -> usize {
        self.axes.len()
    }
}