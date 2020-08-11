#![allow(unused)]
use pyo3::prelude::*;
use std::collections::HashMap;
use pyo3::types::*;
use crate::common::Env;

// lifetimes will probably have to be annotated at a later stage

#[derive(Debug, Default)]
pub struct Axes {
    plot_data: Option<PlotData>,
}

impl Axes {
    pub fn empty() -> Axes {
        Axes {
            plot_data: None,
        }
    }
}

#[derive(Debug)]
enum PlotData {
    // https://matplotlib.org/3.2.2/api/axes_api.html#plotting
    Scatter(Scatter),
    Plot(Plot),
}

#[derive(Debug, Default)]
pub struct Scatter {
    // https://matplotlib.org/3.2.2/api/_as_gen/matplotlib.axes.Axes.scatter.html#matplotlib.axes.Axes.scatter

}

#[derive(Debug, Default)]
pub struct Plot {
    // https://matplotlib.org/3.2.2/api/_as_gen/matplotlib.axes.Axes.plot.html#matplotlib-axes-axes-plot
}
