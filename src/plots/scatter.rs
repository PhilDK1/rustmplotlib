#![allow(unused_imports)]
use crate::addition_objs::colormap::Colormap;
use crate::addition_objs::markerstyle::MarkerStyle;
use crate::addition_objs::normalize::Normalize;
use ndarray::prelude::*;
use ndarray::{Array, Array1};
use pyo3::prelude::*;
use pyo3::types::*;
// use ndarray::IntoDimension;
use numpy::{Element, PyArray};
use std::marker::PhantomData;
pub struct Scatter<'py, T: pyo3::conversion::ToPyObject + Element> {
    // https://matplotlib.org/3.2.2/api/_as_gen/matplotlib.axes.Axes.scatter.html#matplotlib.axes.Axes.scatter
    // x_data: &'py [T],
    x_data: Array1<T>,
    y_data: Array1<T>,
    phantom: PhantomData<&'py T>,
    marker_style: Option<MarkerStyle>,
    cmap: Option<Colormap>,
    norm: Option<Normalize>,
    alpha: Option<f32>,
    // linewidths,
    // edge colors,
    plotnonfinite: bool,
}

impl<'py, T: pyo3::conversion::ToPyObject + Element> Scatter<'py, T> {
    pub fn new(x: &'py [T], y: &'py [T]) -> Scatter<'py, T> {
        // makes new scatter plot
        let x_val = Array1::from(x.to_owned().to_vec());
        let y_val = Array1::from(y.to_owned().to_vec());
        Scatter {
            x_data: x_val,
            y_data: y_val,
            phantom: PhantomData,
            marker_style: None,
            cmap: None,
            norm: None,
            alpha: None,
            plotnonfinite: false,
        }
    }

    pub fn set_xdata(&mut self, x_data: &'py [T]) {
        // resets xdata over what was previously there
        self.x_data = Array1::from(x_data.to_owned().to_vec());
    }

    pub fn set_ydata(&mut self, y_data: &'py [T]) {
        // resets ydata over what was previously there
        self.y_data = Array1::from(y_data.to_owned().to_vec());
    }

    pub fn get_plot_pyargs(&self, py: Python<'py>) -> &PyTuple {
        // makes into &PyTuple to pass up to calling function
        let (x, y) = self.data_as_np_array(py);
        PyTuple::new(
            py,
            vec![x, y].into_iter(), // vec![self.x_data.to_owned(), self.y_data.to_owned()].into_iter()
        )
    }

    fn data_as_np_array(&self, py: Python<'py>) -> (&PyArray<T, Ix1>, &PyArray<T, Ix1>) {
        let x: &PyArray<T, Ix1> = PyArray::from_array(py, &self.x_data);
        let y: &PyArray<T, Ix1> = PyArray::from_array(py, &self.y_data);
        (x, y)
    }

    pub fn set_marker(&mut self, markerstyle: MarkerStyle) {
        // set a markerstyle over writes if already there
        self.marker_style = Some(markerstyle);
    }

    pub fn set_cmap(&mut self, cmap: Colormap) {
        // sets a colormap, over writes if already there
        self.cmap = Some(cmap);
    }

    pub fn set_norm(&mut self, norm: Normalize) {
        self.norm = Some(norm);
    }

    pub fn set_alpha(&mut self, alpha: f32) {
        self.alpha = Some(alpha);
    }

    pub fn plotnonfinite(&mut self) {
        self.plotnonfinite = true;
    }

    pub fn get_plot_kwargs(&self, py: Python<'py>, mpl: &'py PyModule) -> &PyDict {
        // returns a reference to a pydict, with the various kwargs that are required, sets to None if not specified
        let new_dict = PyDict::new(py);

        // makes an instance of colormap and adds ref to pydict
        match &self.cmap {
            // if colormap has been specified then unpack data and add to pydict
            Some(colormap) => {
                // options are kwargs of the matplotlib.colors.Colormap object
                let options = PyDict::new(py);

                // matches n if specified if not default of 256 is applied
                match colormap.n {
                    Some(num) => options.set_item("N", num),
                    None => options.set_item("N", 256),
                }.expect("Err of some kind in default vals of src/figax/plots.rs:: Scatter.get_plot_kwargs()");
                // colormap name must be specified
                let cmap = mpl.call_method("colors.Colormap", (colormap.name.to_string(),), Some(options)).unwrap();
                // adds colormap to kwargs of scatter call if colormap specified
                new_dict.set_item("cmap", cmap)

            },
            // if colormap not specified then defaults to None
            None =>new_dict.set_item("cmap", py.None()),

        }.expect("Err of some kind in colormap vals of rc/plots/scatter.rs Scatter.get_plot_kwargs()");

        // same as above, sets default to None for unspecified arguement, and adds markerstyle to scatter call kwargs
        match &self.marker_style {

            Some(markerstyle) => {
                let options = PyDict::new(py);

                match &markerstyle.marker {
                    Some(str_marker) => options.set_item("marker", str_marker.to_string()),
                    None => options.set_item("marker", py.None()),
                }
                .expect("Err of some kind in markerstyle vals of src/plots/scatter.rs Scatter.get_plot_kwargs()");

                match &markerstyle.fillstyle {
                    Some(fillstyle_str) => options.set_item("fillstyle", fillstyle_str.to_string()),
                    None => options.set_item("fillstyle", py.None()),
                }
                .expect("Err of some kind in markerstyle vals of src/plots/scatter.rs Scatter.get_plot_kwargs()");

                let markerstyling = mpl
                    .call_method("markers.MarkerStyle", (), Some(options))
                    .unwrap();

                new_dict.set_item("marker", markerstyling)

            },

            None => new_dict.set_item("marker", py.None()),

        }.expect("Err of some kind in markerstyle vals of src/plots/scatter.rs Scatter.get_plot_kwargs()");

        // same as above, sets default to None for unspecified arguement, and adds norm to scatter call kwargs
        match &self.norm {
            Some(norm) => {
                let options = PyDict::new(py);
                match &norm.vmin {
                    Some(vmin) => options.set_item("vmin", vmin),
                    None => options.set_item("vmin", py.None()),
                }
                .expect("error for setting vmin in scatter.rs");

                match &norm.vmax {
                    Some(vmax) => options.set_item("vmax", vmax),
                    None => options.set_item("vmax", py.None()),
                }
                .expect("error for setting vmax in scatter.rs");

                match &norm.clip {
                    true => options.set_item("clip", true),
                    _flase => options.set_item("clip", false),
                }
                .expect("error when setting clip of normalise in scatter.rs");

                let normal = mpl
                    .call_method("colors.Normalize", (), Some(options))
                    .unwrap();
                new_dict.set_item("norm", normal)
            }
            None => new_dict.set_item("norm", py.None()),
        }
        .expect("error when making norm");

        match &self.alpha {
            Some(alp) => new_dict.set_item("alpha", *alp),
            None => new_dict.set_item("alpha", py.None()),
        }
        .expect("error when setting alpha");

        match &self.plotnonfinite {
            true => new_dict.set_item("plotnonfinite", true),
            false => new_dict.set_item("plotnonfinite", false),
        }
        .expect("error when setting plotnonfinite kwarg");

        new_dict
    }
}
