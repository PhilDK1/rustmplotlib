#![allow(unused)]
use pyo3::prelude::*;
use std::collections::HashMap;
use pyo3::types::*;
use crate::common::Env;

// lifetimes will probably have to be annotated at a later stage

// #[derive(Debug, Default)]
pub struct Axes<'a, T> {
    plot_data: Option<PlotData<'a, T>>,
    title: Option<String>, // https://matplotlib.org/3.2.2/api/_as_gen/matplotlib.axes.Axes.set_title.html#matplotlib-axes-axes-set-title
    xlabel: Option<String>,
    ylabel: Option<String>,

}

impl<'a, T> Axes<'a, T> {
    pub fn empty() -> Axes<'a, T> {
        Axes::<T> {
            plot_data: None,
            title: None,
            xlabel: None,
            ylabel: None,
        }
    }

    pub fn set_title(&mut self, title: &str) {
        // https://matplotlib.org/3.2.2/api/_as_gen/matplotlib.axes.Axes.set_title.html#matplotlib-axes-axes-set-title
        self.title = Some(title.to_owned());
    }

    pub fn get_title(&self) -> Result<String, &'static str> {
        // https://matplotlib.org/3.2.2/api/_as_gen/matplotlib.axes.Axes.get_title.html#matplotlib-axes-axes-get-title
        match &self.title {
            Some(set_title) => Ok(set_title.to_string()),
            None => Err("No title set, try: Axis.set_title(title: &str)"),
        } 
    }

    pub fn set_xlabel(&mut self, xlabel: &str) {
        // https://matplotlib.org/3.2.2/api/_as_gen/matplotlib.axes.Axes.set_xlabel.html#matplotlib.axes.Axes.set_xlabel
        self.xlabel = Some(xlabel.to_owned());
    }

    pub fn get_xlabel(&self) -> Result<String, &'static str> {
        // https://matplotlib.org/3.2.2/api/_as_gen/matplotlib.axes.Axes.get_xlabel.html#matplotlib.axes.Axes.get_xlabel
        match &self.xlabel {
            Some(set_xlabel) => Ok(set_xlabel.to_string()),
            None => Err("No xlabel set, try: Axis.set_xlabel(xlabel: &str)"),
        } 
    }


    pub fn set_ylabel(&mut self, ylabel: &str) {
        // https://matplotlib.org/3.2.2/api/_as_gen/matplotlib.axes.Axes.set_ylabel.html#matplotlib.axes.Axes.set_ylabel
        self.ylabel = Some(ylabel.to_owned());
    }

    pub fn get_ylabel(&self) -> Result<String, &'static str> {
        // https://matplotlib.org/3.2.2/api/_as_gen/matplotlib.axes.Axes.get_xlabel.html#matplotlib.axes.Axes.get_xlabel
        match &self.ylabel {
            Some(set_ylabel) => Ok(set_ylabel.to_string()),
            None => Err("No ylabel set, try: Axis.set_ylabel(ylabel: &str)"),
        } 
    }
}

// #[derive(Debug)]
enum PlotData<'a, T> {
    // https://matplotlib.org/3.2.2/api/axes_api.html#plotting
    Scatter(Scatter<'a, T>),
    Plot(Plot),
}

// #[derive(Debug, Default)]
pub struct Scatter<'a, T> {
    // https://matplotlib.org/3.2.2/api/_as_gen/matplotlib.axes.Axes.scatter.html#matplotlib.axes.Axes.scatter
    x_data: &'a [T],
    y_data: &'a [T],

}

#[derive(Debug, Default)]
pub struct Plot {
    // https://matplotlib.org/3.2.2/api/_as_gen/matplotlib.axes.Axes.plot.html#matplotlib-axes-axes-plot
}
