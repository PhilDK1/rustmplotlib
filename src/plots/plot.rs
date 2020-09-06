// use crate::addition_objs::colormap::Colormap;
// use crate::addition_objs::markerstyle::MarkerStyle;
// use crate::addition_objs::normalize::Normalize;
use pyo3::prelude::*;
use pyo3::types::*;
use numpy::{PyArray, Element};

pub struct Plot<'py, T: pyo3::conversion::ToPyObject+ Element> {
    // https://matplotlib.org/3.2.2/api/_as_gen/matplotlib.axes.Axes.plot.html#matplotlib.axes.Axes.plot
    x: &'py [T],
    y: &'py [T],
    fmt: Option<String>,
    scalex: bool,
    scaley: bool,
}

impl<'py, T: pyo3::conversion::ToPyObject + Element> Plot<'py, T: pyo3::conversion::ToPyObject>{
    pub fn new(x: &[T], y: &[T]) -> Plot<'py, T> {
        Plot {
            x,
            y,
            fmt: None,
            scalex: false,
            scaley: false,
        }

    }

    pub fn fmt(&mut self, fmt: String) {
        self.fmt = Some(fmt);
    }
}

pub struct Line2D<'py, T: pyo3::conversion::ToPyObject + Element> {
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
        

}