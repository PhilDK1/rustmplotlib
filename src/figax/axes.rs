#![allow(unused)]
use pyo3::prelude::*;
use std::collections::HashMap;
use pyo3::types::*;
use crate::common::Env;

// lifetimes will probably have to be annotated at a later stage

pub struct Axes<'a, T: pyo3::conversion::ToPyObject> {
    py: &'a Python<'a>,
    plot_data: Option<PlotData<'a, T>>,
    title: Option<String>, // https://matplotlib.org/3.2.2/api/_as_gen/matplotlib.axes.Axes.set_title.html#matplotlib-axes-axes-set-title
    xlabel: Option<String>, //https://matplotlib.org/3.2.2/api/_as_gen/matplotlib.axes.Axes.set_xlabel.html#matplotlib.axes.Axes.set_xlabel
    ylabel: Option<String>, //https://matplotlib.org/3.2.2/api/_as_gen/matplotlib.axes.Axes.set_ylabel.html#matplotlib.axes.Axes.set_ylabel

}

impl<'a, T: pyo3::conversion::ToPyObject> Axes<'a, T> {
    pub fn empty(py: &'a Python) -> Axes<'a, T> {
        Axes::<T> {
            py: py,
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

    pub fn scatter(&mut self, x: &'a [T], y: &'a [T]) {
        let scatter_plot: PlotData<'_, T> = PlotData::Scatter(Scatter::new(self.py, x, y));
        self.plot_data = Some(scatter_plot);
    }

    pub fn set_xdata(&mut self, x_data: &'a [T]) {
        match &mut self.plot_data {
            Some(PlotData::Scatter(scatter_plot)) => scatter_plot.set_xdata(x_data),
            _ => println!("Not implimented yet."),
        }
    }

    pub fn set_ydata(&mut self, y_data: &'a [T]) {
        match &mut self.plot_data {
            Some(PlotData::Scatter(scatter_plot)) => scatter_plot.set_ydata(y_data),
            _ => println!("Not implimented yet."),
        }
    }

    pub fn identify(&self) -> String {
        let ans = match &self.plot_data {
            Some(PlotData::Scatter(scatter_plot)) => "scatter".to_owned(),
            Some(PlotData::Plot(plot)) => "plot".to_owned(),
            None => "No known plot specified".to_owned(),
        };
        ans
    }

    // pub fn get_pyargs(&self) -> PyTuple {

    // }
}

// #[derive(Debug)]
enum PlotData<'a, T: pyo3::conversion::ToPyObject> {
    // https://matplotlib.org/3.2.2/api/axes_api.html#plotting
    Scatter(Scatter<'a, T>),
    Plot(Plot),
}

impl<'a, T: pyo3::conversion::ToPyObject> PlotData<'a, T> {
    pub fn identify(&self) -> String {
        match self {
            PlotData::Scatter(scatter_plot) => "scatter".to_owned(),
            PlotData::Plot(plot) => "plot".to_owned(),
        }
    }
}

// #[derive(Debug, Default)]
pub struct Scatter<'a, T: pyo3::conversion::ToPyObject> {
    // https://matplotlib.org/3.2.2/api/_as_gen/matplotlib.axes.Axes.scatter.html#matplotlib.axes.Axes.scatter
    py: &'a Python<'a>, 
    x_data: &'a [T],
    y_data: &'a [T],

}

impl<'a, T: pyo3::conversion::ToPyObject> Scatter<'a, T> {
    pub fn new(py: &'a Python, x: &'a [T], y: &'a [T]) -> Scatter<'a, T> {
        Scatter {
            py: py,
            x_data: &x,
            y_data: &y,
        }
    }

    fn set_xdata(&mut self, x_data: &'a [T]) {
        self.x_data = x_data;
    }

    fn set_ydata(&mut self, y_data: &'a [T]) {
        self.x_data = y_data;
    }

    fn get_pyargs(&self) -> &PyTuple {
        PyTuple::new(*self.py, vec![self.x_data.to_owned(), self.y_data.to_owned()].into_iter())
    }
}


#[derive(Debug, Default)]
pub struct Plot {
    // https://matplotlib.org/3.2.2/api/_as_gen/matplotlib.axes.Axes.plot.html#matplotlib-axes-axes-plot
}
