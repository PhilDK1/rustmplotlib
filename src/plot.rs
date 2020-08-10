#![allow(unused)]
use pyo3::prelude::*;
use std::collections::HashMap;
use pyo3::types::*;

pub struct PyPlot<'p> {
    // struct based on python3 matplotlib pyploy.plot command 
    // using https://matplotlib.org/3.2.2/api/_as_gen/matplotlib.pyplot.plot.html
    // as api reference
    // y: HashMap<String,Vec<T>>,
    // x: Option<Vec<T>>,
    // format: LineFMT,
    // kwargs: HashMap<String, Option<T>>
    py: Python<'p>,
    plt: &'p PyModule,
}

pub struct Env {
    gil: GILGuard
}

impl Env {
    pub fn new() -> Env {
        Env {
            gil: Python::acquire_gil(),
        }
    }
}


impl<'p> PyPlot<'p> {
    pub fn new<'a: 'p>(env: &'a Env) -> PyPlot<'p> {
        let python = env.gil.python();
        let pltd = PyModule::import(python, "matplotlib.pyplot").unwrap();
        PyPlot {
            py: python,
            plt: pltd,
        }
    }

    pub fn plot(&self, x: &[f64], y: &[f64]) {
        self.plt.call("plot", (x.to_owned(), y.to_owned()), /*sd.as_ref()*/ None).map_err(|e| {
            e.print_and_set_sys_last_vars(self.py);
        }).expect("Python Error");
    }

    pub fn scatter(&self, x: &[f64], y: &[f64]) {
        self.plt.call("scatter", (x.to_owned(), y.to_owned()), None).map_err(|e| {
            e.print_and_set_sys_last_vars(self.py);
        }).expect("Python Error");
    }

    pub fn show(self) -> PyResult<()>{
        self.plt.call0("show").map_err(|e| {
            e.print_and_set_sys_last_vars(self.py);
        }).expect("Python Error");

        Ok(())
    }
}