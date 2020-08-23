use pyo3::prelude::*;
use pyo3::types::*;
use crate::addition_objs::colormap::Colormap;
use crate::addition_objs::markerstyle::MarkerStyle;
pub struct Scatter<'p, T: pyo3::conversion::ToPyObject> {
    // https://matplotlib.org/3.2.2/api/_as_gen/matplotlib.axes.Axes.scatter.html#matplotlib.axes.Axes.scatter
    x_data: &'p [T],
    y_data: &'p [T],
    // cannot use as they take a obj in python don't know how to convert yet:
    marker_style: Option<MarkerStyle>,
    cmap: Option<Colormap>,
}

impl<'p, T: pyo3::conversion::ToPyObject> Scatter<'p, T> {
    pub fn new(x: &'p [T], y: &'p [T]) -> Scatter<'p, T> {
        // makes new scatter plot
        Scatter {
            x_data: &x,
            y_data: &y,
            marker_style: None,
            cmap: None,
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

    pub fn get_plot_pyargs(&self, py: Python<'p>) -> &PyTuple {
        // makes into &PyTuple to pass up to calling function
        PyTuple::new(
            py,
            vec![self.x_data.to_owned(), self.y_data.to_owned()].into_iter(),
        )
    }

    pub fn set_marker(&mut self, markerstyle: MarkerStyle) {
        self.marker_style = Some(markerstyle);
    }

    pub fn set_cmap(&mut self, cmap: Colormap) {
        self.cmap = Some(cmap);
    }

    pub fn get_plot_kwargs(&self, py: Python<'p>, mpl: &'p PyModule) -> &PyDict {
        let new_dict = PyDict::new(py);

        match &self.cmap {
            Some(colormap) => {
                let options = PyDict::new(py);
                match colormap.n {
                    Some(num) => options.set_item("N", num),
                    None => options.set_item("N", 256),
                }.expect("Err of some kind in default vals of src/figax/plots.rs:: Scatter.get_plot_kwargs()");
                
                
                let cmap = mpl.call_method("colors.Colormap", (colormap.name.to_string(),), Some(options)).unwrap();
                new_dict.set_item("cmap", cmap)
            },
            None =>new_dict.set_item("cmap", py.None()),
        }.expect("Err of some kind in colormap vals of rc/plots/scatter.rs Scatter.get_plot_kwargs()");

        match &self.marker_style {
            Some(markerstyle) => {
                let options = PyDict::new(py);
                
                match &markerstyle.marker {
                    Some(str_marker) => options.set_item("marker", str_marker.to_string()),
                    None => options.set_item("marker", py.None()),
                }.expect("Err of some kind in markerstyle vals of src/plots/scatter.rs Scatter.get_plot_kwargs()");

                match &markerstyle.fillstyle {
                    Some(fillstyle_str) => options.set_item("fillstyle", fillstyle_str.to_string()),
                    None => options.set_item("fillstyle", py.None()),
                }.expect("Err of some kind in markerstyle vals of src/plots/scatter.rs Scatter.get_plot_kwargs()");
                let markerstyling = mpl.call_method("markers.MarkerStyle", (), Some(options)).unwrap();
                new_dict.set_item("marker", markerstyling)
            },
            None => new_dict.set_item("marker", py.None()),
        }.expect("Err of some kind in markerstyle vals of src/plots/scatter.rs Scatter.get_plot_kwargs()");

        new_dict
    }
}

// pub struct Plot {
//     // https://matplotlib.org/3.2.2/api/_as_gen/matplotlib.axes.Axes.plot.html#matplotlib-axes-axes-plot
// }