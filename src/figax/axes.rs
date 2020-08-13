#![allow(unused)]
use crate::common::Env;
use pyo3::prelude::*;
use pyo3::types::*;
use std::collections::HashMap;

// lifetimes will probably have to be annotated at a later stage

pub struct Axes<'p, T: pyo3::conversion::ToPyObject> {
    plot_data: Option<PlotData<'p, T>>,
    title: Option<String>, // https://matplotlib.org/3.2.2/api/_as_gen/matplotlib.axes.Axes.set_title.html#matplotlib-axes-axes-set-title
    xlabel: Option<String>, //https://matplotlib.org/3.2.2/api/_as_gen/matplotlib.axes.Axes.set_xlabel.html#matplotlib.axes.Axes.set_xlabel
    ylabel: Option<String>, //https://matplotlib.org/3.2.2/api/_as_gen/matplotlib.axes.Axes.set_ylabel.html#matplotlib.axes.Axes.set_ylabel
    plot_index: Option<usize>,
}

impl<'p, T: pyo3::conversion::ToPyObject> Axes<'p, T> {
    pub fn empty() -> Axes<'p, T> {
        Axes::<T> {
            plot_data: None,
            title: None,
            xlabel: None,
            ylabel: None,
            plot_index: None,
        }
    }

    pub fn set_index(&mut self, index: usize) {
        self.plot_index = Some(index);
    }

    pub fn get_index(&self) -> Result<usize, &'static str> {
        match &self.plot_index {
            Some(ind) => Ok(*ind),
            None => Err("No title set, try: Axis.set_title(title: &str)"),
        }
    }

    pub fn set_title(&mut self, title: &str) {
        // https://matplotlib.org/3.2.2/api/_as_gen/matplotlib.axes.Axes.set_title.html#matplotlib-axes-axes-set-title
        self.title = Some(title.to_owned());
    }

    pub fn get_title(&self) -> Result<String, &'static str> {
        // https://matplotlib.org/3.2.2/api/_as_gen/matplotlib.axes.Axes.get_title.html#matplotlib-axes-axes-get-title
        match &self.title {
            Some(get_title) => Ok(get_title.to_string()),
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
            Some(get_xlabel) => Ok(get_xlabel.to_string()),
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
            Some(get_ylabel) => Ok(get_ylabel.to_string()),
            None => Err("No ylabel set, try: Axis.set_ylabel(ylabel: &str)"),
        }
    }

    pub fn get_plot_data(&self) -> Result<&PlotData<'p, T>, &'static str> {
        match &self.plot_data {
            Some(plot_data) => Ok(plot_data),
            None => Err("No plot data set, try calling desired plot type on Axis object"),
        }
    }

    pub fn scatter(mut self, x: &'p [T], y: &'p [T]) -> Self {
        let scatter_plot: PlotData<'p, T> = PlotData::Scatter(Scatter::new(x, y));
        self.plot_data = Some(scatter_plot);
        self
    }

    pub fn set_xdata(&mut self, x_data: &'p [T]) {
        match &mut self.plot_data {
            Some(PlotData::Scatter(scatter_plot)) => scatter_plot.set_xdata(x_data),
            _ => println!("Not implimented yet."),
        }
    }

    pub fn set_ydata(&mut self, y_data: &'p [T]) {
        match &mut self.plot_data {
            Some(PlotData::Scatter(scatter_plot)) => scatter_plot.set_ydata(y_data),
            _ => println!("Not implimented yet."),
        }
    }

    pub fn identify(&self) -> String {
        match &self.plot_data {
            Some(PlotData::Scatter(scatter_plot)) => "scatter".to_owned(),
            // Some(PlotData::Plot(plot)) => "plot".to_owned(),
            None => "No known plot specified".to_owned(),
        }
    }

}

pub enum PlotData<'p, T: pyo3::conversion::ToPyObject> {
    // https://matplotlib.org/3.2.2/api/axes_api.html#plotting
    Scatter(Scatter<'p, T>),
    // Plot(Plot),
}

impl<'p, T: pyo3::conversion::ToPyObject> PlotData<'p, T> {
    pub fn identify(&self) -> String {
        match self {
            PlotData::Scatter(scatter_plot) => "scatter".to_owned(),
            // PlotData::Plot(plot) => "plot".to_owned(),
        }
    }

    pub fn get_pyargs(&self, py: Python<'p>) -> &PyTuple {
        match self {
            PlotData::Scatter(scatter_plot) => scatter_plot.get_pyargs(py),
            // PlotData::Plot(plot) => PyTuple::new(py, vec![].into_iter()),
        }
    }
}

pub struct Scatter<'p, T: pyo3::conversion::ToPyObject> {
    // https://matplotlib.org/3.2.2/api/_as_gen/matplotlib.axes.Axes.scatter.html#matplotlib.axes.Axes.scatter
    x_data: &'p [T],
    y_data: &'p [T],
}

impl<'p, T: pyo3::conversion::ToPyObject> Scatter<'p, T> {
    pub fn new(x: &'p [T], y: &'p [T]) -> Scatter<'p, T> {
        Scatter {
            x_data: &x,
            y_data: &y,
        }
    }

    fn set_xdata(&mut self, x_data: &'p [T]) {
        self.x_data = x_data;
    }

    fn set_ydata(&mut self, y_data: &'p [T]) {
        self.x_data = y_data;
    }

    fn get_pyargs(&self, py: Python<'p>) -> &PyTuple {
        PyTuple::new(
            py,
            vec![self.x_data.to_owned(), self.y_data.to_owned()].into_iter(),
        )
    }
}

// pub struct Plot {
//     // https://matplotlib.org/3.2.2/api/_as_gen/matplotlib.axes.Axes.plot.html#matplotlib-axes-axes-plot
// }
