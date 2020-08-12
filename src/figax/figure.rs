#![allow(unused)]
use pyo3::prelude::*;
use std::collections::HashMap;
use std::unreachable;
use pyo3::types::*;
use crate::figax::axes;
use crate::common::Env;


pub struct Figure<'p, T: pyo3::conversion::ToPyObject> {
    py: Python<'p>,
    plt: &'p PyModule,
    subplots: Subplots<'p, T>,
}

impl<'p, T: pyo3::conversion::ToPyObject> Figure<'p, T> {
    pub fn new<'a: 'p>(env: &'a Env) -> Figure<'p, T> { 
        let python = env.gil.python();
        let plot = python.import("matplotlib.pyplot").unwrap();
        let set_of_subplots = Subplots::initialise(env);
        Figure {
            py: python,
            plt: plot,
            subplots: set_of_subplots,
        }
    }

    // pub fn add_empty_subplot(&'p mut self) -> &mut axes::Axes<'p, T> {
        
    //     self.subplots.add_empty_subplot()
    // }

    // pub fn show(&self) {
    //     //place holder function for the chain to pass to python
    //     self.plt.call0("figure").map_err(|e| {
    //         e.print_and_set_sys_last_vars(self.subplots.py);
    //     }).expect("Python Error");

    //     for axis in &self.subplots.axes {
    //         self.plt.call0("axes").map_err(|e| {
    //             e.print_and_set_sys_last_vars(self.subplots.py);
    //         }).expect("Python Error");

    //         let ax_type = &axis.identify();
    //         // self.plt.call(ax_type.to_str(), args: impl IntoPy<Py<PyTuple>>, kwargs: Option<&PyDict>)
    //     }
    // }

    pub fn show(self) {
        self.plt.call0("figure").map_err(|e| {
            e.print_and_set_sys_last_vars(self.py);
        }).expect("Python Error");

        for axis in self.subplots.axes {
            let name = axis.identify();
            let plotdata = axis.get_plot_data().unwrap();
            let args = plotdata.get_pyargs(self.py);
            self.plt.call(name.as_str(), args, None);
        }

        self.plt.call0("show");

    }
    


    pub fn add_subplot(mut self, new_axes: axes::Axes<'p, T>) -> Self{
        self.subplots = self.subplots.add_subplot(new_axes);
        self
    }
}

struct Subplots<'p, T: pyo3::conversion::ToPyObject> {
    // py: Python<'p> ,
    axes: Vec<axes::Axes<'p, T>>,
}

impl<'p, T: pyo3::conversion::ToPyObject> Subplots<'p, T> {
    pub fn initialise<'a: 'p>(env: &'a Env) -> Subplots<'p, T> {
        // let python = env.gil.python();
        Subplots {
            // py: python,
            axes: vec![],
        }
    }

    // pub fn add_empty_subplot(&'p mut self) -> &'p mut axes::Axes<'p, T> {
    //     //place holder function
    //     let mut new_axes = axes::Axes::empty(&self.py);
    //     let previous_len = self.num_axes();
    //     self.axes.push(new_axes);

    //     &mut self.axes[previous_len]
    // }

    fn num_axes(&self) -> usize {
        self.axes.len()
    }

    pub fn add_subplot(mut self, new_axes: axes::Axes<'p, T>)-> Self {
        
        // let mut new_axes = axes::Axes::empty(&self.);
        let previous_len = self.num_axes();
        self.axes.push(new_axes);
        self
        // &mut self.axes[previous_len]
    }
}