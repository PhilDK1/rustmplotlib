use pyo3::prelude::*;
use pyo3::types::*;
// use crate::addition_objs::colormap::Colormap;
use crate::plots::*;
use numpy::Element;

pub enum PlotData<'py, T: pyo3::conversion::ToPyObject + Element> {
    // https://matplotlib.org/3.2.2/api/axes_api.html#plotting
    Scatter(Scatter<'py, T>),
    Plot(Plot<'py, T>),
    PlotSurface(PlotSurface<'py, T>),
}

// possibly make a second type of PlotData corresponding to the different axis types
impl<'py, T: pyo3::conversion::ToPyObject + Element> PlotData<'py, T> {
    pub fn identify(&self) -> String {
        // gets name of plotdata method call
        match self {
            PlotData::Scatter(_scatter_plot) => "scatter".to_owned(),
            PlotData::PlotSurface(_surface_plot) => "plot_surface".to_owned(),
            PlotData::Plot(_plot) => "plot".to_owned(),
        }
    }

    pub fn get_plotdata_pyargs(&self, py: Python<'py>) -> &PyTuple {
        // gets args and returns them as a &PyTuple
        match self {
            PlotData::Scatter(scatter_plot) => scatter_plot.get_plot_pyargs(py),
            PlotData::PlotSurface(surface_plot) => surface_plot.get_plot_pyargs(py),
            PlotData::Plot(plot) => plot.get_plot_pyargs(py),
        }
    }

    pub fn get_plotdata_pykwargs(&self, py: Python<'py>, mpl: &'py PyModule) -> &PyDict {
        match self {
            PlotData::Scatter(scatter_plot) => scatter_plot.get_plot_kwargs(py, mpl), //placeholder
            PlotData::PlotSurface(surface_plot) => surface_plot.get_plot_kwargs(py, mpl),
            PlotData::Plot(plot) => plot.get_plot_kwargs(py, mpl),
        }
    }
}
