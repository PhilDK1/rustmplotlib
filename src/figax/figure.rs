#![allow(unused)]
use pyo3::prelude::*;
use std::collections::HashMap;
use std::unreachable;
use pyo3::types::*;
use crate::figax::axes;
use crate::common::Env;


// #[derive( Default)]
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

    pub fn add_empty_subplot(&'p mut self) -> &mut axes::Axes<'p, T> {
        // place holder function
        // let mut ax = match &mut self.subplots {
        //     None => {
        //         let mut set_of_subplots = Subplots::initialise(&self.py);
        //         let ax = set_of_subplots.add_empty_subplot();
        //         self.subplots = Some(set_of_subplots);
        //         // set_of_subplots.add_empty_subplot()
        //         ax
        //     },
        //     Some(set_of_subplots) => {
        //         set_of_subplots.add_empty_subplot()
        //     }
        // };
        // ax

        // if self.subplots == None {
        //     self.subplots = Some(Subplots::initialise(self.py))
        // } else {
        //     let Some(set_of_subplots) = self.subplots;

        //     // self.subplots.add_empty_subplot()
        // }
        self.subplots.add_empty_subplot()
    }

    pub fn show(&self) {
        //place holder function for the chain to pass to python
        self.plt.call0("figure").map_err(|e| {
            e.print_and_set_sys_last_vars(self.subplots.py);
        }).expect("Python Error");

        // let set_of_subplots = match &self.subplots {
        //     None => {
        //         let set_of_subplots = Subplots::initialise(&self.py);
        //         self.subplots = Some(set_of_subplots);
        //         let set_of_subplots = match &self.subplots {
        //             Some(set_of_subplots) => {
        //                 set_of_subplots
        //             },
        //             None => unreachable!(),
        //         };
        //         set_of_subplots
        //     },
        //     Some(set_of_subplots) => {
        //         set_of_subplots
        //     }
        // };

        for axis in &self.subplots.axes {
            self.plt.call0("axes").map_err(|e| {
                e.print_and_set_sys_last_vars(self.subplots.py);
            }).expect("Python Error");

            let ax_type = &axis.identify();
            // self.plt.call(ax_type.to_str(), args: impl IntoPy<Py<PyTuple>>, kwargs: Option<&PyDict>)
        }
    }
}

// #[derive(Debug, Default)]
struct Subplots<'p, T: pyo3::conversion::ToPyObject> {
    py: Python<'p> ,
    axes: Vec<axes::Axes<'p, T>>,
}

impl<'p, T: pyo3::conversion::ToPyObject> Subplots<'p, T> {
    pub fn initialise<'a: 'p>(env: &'a Env) -> Subplots<'p, T> {
        let python = env.gil.python();
        Subplots {
            py: python,
            axes: vec![],
        }
    }

    pub fn add_empty_subplot(&'p mut self) -> &'p mut axes::Axes<'p, T> {
        //place holder function
        let mut new_axes = axes::Axes::empty(&self.py);
        let previous_len = self.num_axes();
        self.axes.push(new_axes);

        &mut self.axes[previous_len]
    }

    fn num_axes(&self) -> usize {
        self.axes.len()
    }

    pub fn add_subplot(&'p mut self) -> &'p mut axes::Axes<'p, T> {
        
        let mut new_axes = axes::Axes::empty(&self.py);
        let previous_len = self.num_axes();
        self.axes.push(new_axes);

        &mut self.axes[previous_len]
    }
}