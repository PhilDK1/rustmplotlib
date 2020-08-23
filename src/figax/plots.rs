use pyo3::prelude::*;
use pyo3::types::*;
// use crate::addition_objs::colormap::Colormap;
use crate::plots::scatter::Scatter;

pub enum PlotData<'p, T: pyo3::conversion::ToPyObject> {
    // https://matplotlib.org/3.2.2/api/axes_api.html#plotting
    Scatter(Scatter<'p, T>),
    // Plot(Plot),
}

impl<'p, T: pyo3::conversion::ToPyObject> PlotData<'p, T> {
    pub fn identify(&self) -> String {
        // gets name of plotdata method call
        match self {
            PlotData::Scatter(_scatter_plot) => "scatter".to_owned(),
            // PlotData::Plot(plot) => "plot".to_owned(),
        }
    }

    pub fn get_plotdata_pyargs(&self, py: Python<'p>) -> &PyTuple {
        // gets args and returns them as a &PyTuple
        match self {
            PlotData::Scatter(scatter_plot) => scatter_plot.get_plot_pyargs(py),
            // PlotData::Plot(plot) => PyTuple::new(py, vec![].into_iter()),
        }
    }

    pub fn get_plotdata_pykwargs(&self, py: Python<'p>, mpl: &'p PyModule) -> &PyDict {
        match self {
            PlotData::Scatter(scatter_plot) => scatter_plot.get_plot_kwargs(py, mpl), //placeholder
        }
    }
}
