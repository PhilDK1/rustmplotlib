// use crate::addition_objs::colormap::Colormap;
// use crate::addition_objs::markerstyle::MarkerStyle;
// use crate::addition_objs::normalize::Normalize;
use pyo3::prelude::*;
use pyo3::types::*;

pub struct PlotSurface<'p, T: pyo3::conversion::ToPyObject> {
    x: &'p [T],
    y: &'p [T],
    z: &'p [T],
}

impl<'p, T: pyo3::conversion::ToPyObject> PlotSurface<'p,T>{
    pub fn get_plot_pyargs(&self, py: Python<'p>) -> &PyTuple {
        // makes into &PyTuple to pass up to calling function
        PyTuple::new(
            py,
            vec![self.x.to_owned(), self.y.to_owned(), self.z.to_owned()].into_iter(),
        )
    }

    pub fn get_plot_kwargs(&self, py: Python<'p>, _mpl: &'p PyModule) -> &PyDict {
        // returns a reference to a pydict, with the various kwargs that are required, sets to None if not specified
        let new_dict = PyDict::new(py);
        new_dict
    }
}