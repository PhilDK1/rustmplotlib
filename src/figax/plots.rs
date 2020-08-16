use pyo3::prelude::*;
use pyo3::types::*;
use crate::addition_objs::colormap::Colormap;


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

    pub fn get_plotdata_pykwargs(&self, py: Python<'p>) -> &PyDict {
        match self {
            PlotData::Scatter(scatter_plot) => /*scatter_plot.get_plot_kwargs(py)*/ PyDict::new(py), //placeholder

        }
    }
}



pub struct Scatter<'p, T: pyo3::conversion::ToPyObject> {
    // https://matplotlib.org/3.2.2/api/_as_gen/matplotlib.axes.Axes.scatter.html#matplotlib.axes.Axes.scatter
    x_data: &'p [T],
    y_data: &'p [T],
    // cannot use as they take a obj in python don't know how to convert yet:
    // marker_style: Option<String>,
    // cmap: Option<Colormap>,
}

impl<'p, T: pyo3::conversion::ToPyObject> Scatter<'p, T> {
    pub fn new(x: &'p [T], y: &'p [T]) -> Scatter<'p, T> {
        // makes new scatter plot
        Scatter {
            x_data: &x,
            y_data: &y,
            // marker_style: None,
            // cmap: None,
        }
    }

    pub fn set_xdata(&mut self, x_data: &'p [T]) {
        // resets xdata over what was previously there
        self.x_data = x_data;
    }

    pub fn set_ydata(&mut self, y_data: &'p [T]) {
        // resets ydata over what was previously there
        self.y_data = y_data;
    }

    fn get_plot_pyargs(&self, py: Python<'p>) -> &PyTuple {
        // makes into &PyTuple to pass up to calling function
        PyTuple::new(
            py,
            vec![self.x_data.to_owned(), self.y_data.to_owned()].into_iter(),
        )
    }
/*
    pub fn set_marker(&mut self, markerstyle: String) {
        self.marker_style = Some(markerstyle);
    }

    pub fn set_cmap(&mut self, cmap: Colormap) {
        self.cmap = Some(cmap);
    }

    fn get_plot_kwargs(&self, py: Python<'p>) -> &PyDict {
        let new_dict = PyDict::new(py);
        match &self.cmap {
            Some(colormap) => 
            new_dict.set_item("cmap", ),
            None =>new_dict.set_item("cmap", py.None()),
        };


        match &self.marker_style {
            Some(marker) => new_dict.set_item("marker", marker),
            None => new_dict.set_item("marker", py.None()),
        };
        new_dict
    }*/
}

// pub struct Plot {
//     // https://matplotlib.org/3.2.2/api/_as_gen/matplotlib.axes.Axes.plot.html#matplotlib-axes-axes-plot
// }
