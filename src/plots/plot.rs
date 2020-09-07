// use crate::addition_objs::colormap::Colormap;
// use crate::addition_objs::markerstyle::MarkerStyle;
// use crate::addition_objs::normalize::Normalize;
use pyo3::prelude::*;
use pyo3::types::*;
use numpy::{PyArray, Element};
use std::marker::PhantomData;
use ndarray::prelude::*;
use ndarray::Array1;

pub struct Plot<'py, T: pyo3::conversion::ToPyObject+ Element> {
    // https://matplotlib.org/3.2.2/api/_as_gen/matplotlib.axes.Axes.plot.html#matplotlib.axes.Axes.plot
    x: Array1<T>,
    y: Array1<T>,
    phantom: PhantomData<&'py T>,
    fmt: Option<String>,
    scalex: bool,
    scaley: bool,
}

impl<'py, T: pyo3::conversion::ToPyObject + Element> Plot<'py, T> {
    pub fn new(x: &[T], y: &[T]) -> Plot<'py, T> {
        let x_val = Array1::from(x.to_owned().to_vec());
        let y_val = Array1::from(y.to_owned().to_vec());
        Plot {
            x: x_val,
            y: y_val,
            phantom: PhantomData,
            fmt: None,
            scalex: false,
            scaley: false,
        }

    }

    pub fn fmt(&mut self, fmt: String) {
        self.fmt = Some(fmt);
    }

    pub fn get_plot_kwargs(&self, py: Python<'py>, _mpl: &'py PyModule) -> &PyDict {
        PyDict::new(py)
    }

    pub fn set_scalex(mut self, scale: bool) -> Self {
        self.scalex = scale;
        self
    }

    pub fn set_scaley(mut self, scale: bool) -> Self {
        self.scaley = scale;
        self
    }

    fn data_as_np_array(&self, py: Python<'py>) -> (&PyArray<T, Ix1>, &PyArray<T, Ix1>) {
        let x: &PyArray<T, Ix1> = PyArray::from_array(py, &self.x);
        let y: &PyArray<T, Ix1> = PyArray::from_array(py, &self.y);
        (x, y)
    }

    pub fn get_plot_pyargs(&self, py: Python<'py>) -> &PyTuple {
        // makes into &PyTuple to pass up to calling function
        let (x, y) = self.data_as_np_array(py);
        PyTuple::new(
            py,
            vec![x, y].into_iter(), // vec![self.x_data.to_owned(), self.y_data.to_owned()].into_iter()
        )
    }
}

// pub struct Line2D<'py, T: pyo3::conversion::ToPyObject + Element> {
        // https://matplotlib.org/3.2.2/api/_as_gen/matplotlib.lines.Line2D.html#matplotlib.lines.Line2D
        // agg_filter,
        // alpha: Option<f32>,
        // animated: Option<bool>,
        // antialiased: Option<bool>,
        // clip_box: Bbox, // 2x2 matrix of points
        // clip_on: Option<bool>,
        // clip_path: // Patch, Path or transformed path
        // color: color,
        // contains,
        // dash_capstyle: Option<String>,
        // dash_joinstyle: Optio<String>,
        // dashes: (Option<(f32,f32)>, Option<(f32,f32)>)
        // data,
        // drawstyle: String // https://matplotlib.org/3.2.2/api/_as_gen/matplotlib.lines.Line2D.html#matplotlib.lines.Line2D.set_drawstyle
        // figure,
        // fillstyle: String // https://matplotlib.org/3.2.2/api/_as_gen/matplotlib.lines.Line2D.html#matplotlib.lines.Line2D.set_fillstyle
        // gid: String, 
        // in_layout: bool,
        // label: String,
        // linestyle: String,
        // linewidth: String,
        // marker: // Marker,
        // markeredgecolor: color,
        // markeredge_width: f32,
        // markerfacecolor: color,
        // markerfacecoloralt: color,
        // markersize: float,
        // markevery: // https://matplotlib.org/3.2.2/api/_as_gen/matplotlib.lines.Line2D.html#matplotlib.lines.Line2D.set_markevery,
        // path_effects: AbstractPathEffect
        // picker: ,
        // pick_radius: f32, 
        // rasterized: Option<bool>,
        // sketch_params: (Option<f32>,Option<f32>,Option<f32>)
        // snap: Option<bool>,
        // solid_capstyle: String,
        // solid_joinstyle: String,
        // transform:  
        // url: String,
        // visibl: Option<bool>,
        // xdata: ,
        // ydata: ,
        // zorder: f32,
        

// }