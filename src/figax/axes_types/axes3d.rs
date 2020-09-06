// use crate::addition_objs::colormap::Colormap;
// use crate::addition_objs::markerstyle::MarkerStyle;
use crate::figax::plots::*;
// use crate::plots::scatter::*;
use numpy::Element;
use pyo3::prelude::*;
use pyo3::types::*;

pub struct Axes3D<'py, T: pyo3::conversion::ToPyObject + Element> {
    plot_data: Option<PlotData<'py, T>>,
    title: Option<String>, // https://matplotlib.org/3.2.2/api/_as_gen/matplotlib.axes.Axes.set_title.html#matplotlib-axes-axes-set-title
    xlabel: Option<String>, //https://matplotlib.org/3.2.2/api/_as_gen/matplotlib.axes.Axes.set_xlabel.html#matplotlib.axes.Axes.set_xlabel
    ylabel: Option<String>, //https://matplotlib.org/3.2.2/api/_as_gen/matplotlib.axes.Axes.set_ylabel.html#matplotlib.axes.Axes.set_ylabel
    plot_index: Option<usize>,
}

impl<'py, T: pyo3::conversion::ToPyObject + Element> Axes3D<'py, T> {
    pub fn empty() -> Axes3D<'py, T> {
        Axes3D {
            plot_data: None,
            title: None,
            xlabel: None,
            ylabel: None,
            plot_index: None,
        }
    }

    pub fn get_kwargs(&self, py: Python<'py>) -> &PyDict {
        let new_dict = PyDict::new(py);
        new_dict
    }

    pub fn set_index(&mut self, index: usize) {
        // specifies the index of this instance of the axis
        self.plot_index = Some(index);
    }

    pub fn get_index(&self) -> Result<usize, &'static str> {
        // gets the index previously specified or errors
        match &self.plot_index {
            Some(ind) => Ok(*ind),
            None => Err("No index set, try: Axis.set_index(index: usize)"),
        }
    }
    // --- below directly copied from axes2d.rs
    pub fn set_title(&mut self, title: &str) {
        // https://matplotlib.org/3.2.2/api/_as_gen/matplotlib.axes.Axes.set_title.html#matplotlib-axes-axes-set-title
        // sets title
        self.title = Some(title.to_owned());
    }

    pub fn get_title(&self) -> Result<String, &'static str> {
        // https://matplotlib.org/3.2.2/api/_as_gen/matplotlib.axes.Axes.get_title.html#matplotlib-axes-axes-get-title
        // gets title or errors
        match &self.title {
            Some(get_title) => Ok(get_title.to_string()),
            None => Err("No title set, try: Axis.set_title(title: &str)"),
        }
    }

    pub fn set_xlabel(&mut self, xlabel: &str) {
        // https://matplotlib.org/3.2.2/api/_as_gen/matplotlib.axes.Axes.set_xlabel.html#matplotlib.axes.Axes.set_xlabel
        // sets xlabel of the data
        self.xlabel = Some(xlabel.to_owned());
    }

    pub fn get_xlabel(&self) -> Result<String, &'static str> {
        // https://matplotlib.org/3.2.2/api/_as_gen/matplotlib.axes.Axes.get_xlabel.html#matplotlib.axes.Axes.get_xlabel
        // gets the xlabel if specified, errors if not
        match &self.xlabel {
            Some(get_xlabel) => Ok(get_xlabel.to_string()),
            None => Err("No xlabel set, try: Axis.set_xlabel(xlabel: &str)"),
        }
    }

    pub fn set_ylabel(&mut self, ylabel: &str) {
        // https://matplotlib.org/3.2.2/api/_as_gen/matplotlib.axes.Axes.set_ylabel.html#matplotlib.axes.Axes.set_ylabel
        // sets the ylabel of the data
        self.ylabel = Some(ylabel.to_owned());
    }

    pub fn get_ylabel(&self) -> Result<String, &'static str> {
        // https://matplotlib.org/3.2.2/api/_as_gen/matplotlib.axes.Axes.get_xlabel.html#matplotlib.axes.Axes.get_xlabel
        // gets the ylabel if specified, errors if not
        match &self.ylabel {
            Some(get_ylabel) => Ok(get_ylabel.to_string()),
            None => Err("No ylabel set, try: Axis.set_ylabel(ylabel: &str)"),
        }
    }

    pub fn get_plot_data(&self) -> Result<&PlotData<'py, T>, &'static str> {
        // gets the plotdata if specified else errors
        match &self.plot_data {
            Some(plot_data) => Ok(plot_data),
            None => Err("No plot data set, try calling desired plot type on Axis object"),
        }
    }

    pub fn identify(&self) -> String {
        // getst the type of plot and returns name of method call
        match &self.plot_data {
            Some(PlotData::Scatter(_scatter_plot)) => "scatter".to_owned(),
            // Some(PlotData::Plot(_plot)) => "plot".to_owned(),
            Some(PlotData::PlotSurface(_surface_plot)) => "plot_surface".to_owned(),
            None => "No known plot specified".to_owned(), // this will completely mess up the call
        }
    }
    // --- end of direct copying
}
