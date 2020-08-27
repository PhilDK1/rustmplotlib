use crate::addition_objs::colormap::Colormap;
use crate::addition_objs::markerstyle::MarkerStyle;
use crate::figax::plots::*;
use crate::plots::scatter::*;
use pyo3::prelude::*;
use pyo3::types::*;


pub struct Axes3D<'p, T: pyo3::conversion::ToPyObject> {
    plot_data: Option<PlotData>,
}

impl<'p, T: pyo3::conversion::ToPyObject> Axes3D {
    pub fn empty() -> Axes3D<'p, T: pyo3::conversion::ToPyObject> {
        Axes3D {
            plot_data: None
        }
    }

    pub fn get_kwargs(&self, py: Python<'p>) -> &PyDict {
        let new_dict = PyDict::new(py);
        new_dict
    }

}