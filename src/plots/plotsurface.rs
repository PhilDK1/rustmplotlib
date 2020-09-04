// use crate::addition_objs::colormap::Colormap;
// use crate::addition_objs::markerstyle::MarkerStyle;
// use crate::addition_objs::normalize::Normalize;
use pyo3::prelude::*;
use pyo3::types::*;

pub struct PlotSurface<'py, T: pyo3::conversion::ToPyObject> {
    x: &'py [T],
    y: &'py [T],
    z: &'py [T],
}

impl<'py, T: pyo3::conversion::ToPyObject> PlotSurface<'py,T>{
    pub fn get_plot_pyargs(&self, py: Python<'py>) -> &PyTuple {
        // makes into &PyTuple to pass up to calling function
        PyTuple::new(
            py,
            vec![self.x.to_owned(), self.y.to_owned(), self.z.to_owned()].into_iter(),
        )
    }

    #[allow(dead_code)]
    fn create_mesh_args(&self) {
        let len_x = self.x.len();
        let len_y = self.y.len();
        let _new_x: Vec<Vec<T>>= Vec::new();
        let _new_y: Vec<Vec<T>>= Vec::new();
        // let s = self.x.to_vec();

        for _i in 0..len_y {
            // new_x.push(self.x.to_vec());
        }
        for _i in 0..len_x {
            // new_y.push(self.y.to_vec());
        }
    }

    pub fn get_plot_kwargs(&self, py: Python<'py>, _mpl: &'py PyModule) -> &PyDict {
        // returns a reference to a pydict, with the various kwargs that are required, sets to None if not specified
        let new_dict = PyDict::new(py);
        new_dict
    }
}