#![allow(unused)]
use pyo3::prelude::*;
use std::collections::HashMap;
use pyo3::types::*;
use crate::figax::axes;
use crate::common::Env;

// lifetimes will probably have to be annotated at a later stage


#[derive(Debug, Default)]
pub struct Figure {
    subplots: Subplots,
}

impl Figure {
    pub fn new() -> Figure { 
        Figure{
            subplots: Subplots::initialise()
        }
    }

    pub fn add_empty_subplot(&mut self) -> &mut axes::Axes{
        // place holder function
        self.subplots.add_empty_subplot()
    }

    pub fn show(self) {
        //place holder function for the chain to pass to python
    }
}

#[derive(Debug, Default)]
struct Subplots {
    axes: Vec<axes::Axes>,
}

impl Subplots{
    pub fn initialise() -> Subplots {
        Subplots {
            axes: vec![],
        }
    }

    pub fn add_empty_subplot(&mut self) -> &mut axes::Axes{
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