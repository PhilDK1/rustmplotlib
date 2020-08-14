use pyo3::prelude::*;
use pyo3::types::*;

pub enum PlotData<'p, T: pyo3::conversion::ToPyObject> {
    // https://matplotlib.org/3.2.2/api/axes_api.html#plotting
    Scatter(Scatter<'p, T>),
    // Plot(Plot),
}



impl<'p, T: pyo3::conversion::ToPyObject> PlotData<'p, T> {
    pub fn identify(&self) -> String {
        // gets name of plotdata method call
        match self {
            PlotData::Scatter(scatter_plot) => "scatter".to_owned(),
            // PlotData::Plot(plot) => "plot".to_owned(),
        }
    }

    pub fn get_pyargs(&self, py: Python<'p>) -> &PyTuple {
        // gets args and returns them as a &PyTuple
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
        // makes new scatter plot
        Scatter {
            x_data: &x,
            y_data: &y,
        }
    }

    fn set_xdata(&mut self, x_data: &'p [T]) {
        // resets xdata over what was previously there
        self.x_data = x_data;
    }

    fn set_ydata(&mut self, y_data: &'p [T]) {
        // resets ydata over what was previously there
        self.y_data = y_data;
    }

    fn get_pyargs(&self, py: Python<'p>) -> &PyTuple {
        // makes into &PyTuple to pass up to calling function
        PyTuple::new(
            py,
            vec![self.x_data.to_owned(), self.y_data.to_owned()].into_iter(),
        )
    }
}

// pub struct Plot {
//     // https://matplotlib.org/3.2.2/api/_as_gen/matplotlib.axes.Axes.plot.html#matplotlib-axes-axes-plot
// }
