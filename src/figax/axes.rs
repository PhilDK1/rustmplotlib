use crate::figax::plots::*;
// use crate::plots::scatter::*;
// use crate::addition_objs::colormap::Colormap;
// use crate::addition_objs::markerstyle::MarkerStyle;
use crate::figax::axes_types::*;
use pyo3::prelude::*;
use pyo3::types::*;

pub enum Axes<'p, T: pyo3::conversion::ToPyObject> {
    Axes2d(Axes2D<'p, T>),
}

impl<'p, T: pyo3::conversion::ToPyObject> Axes<'p, T> {
    pub fn axes2d() -> Axes<'p, T> {
        Axes::Axes2d(Axes2D::empty())
    }

    pub fn get_kwargs(&self, py: Python<'p>) -> &PyDict {
        match &self {
            Axes::Axes2d(ax2d) => ax2d.get_kwargs(py),
        }
    }

    pub fn set_index(mut self, index: usize) -> Self {
        match &mut self {
            Axes::Axes2d(ax2d) => {
                ax2d.set_index(index);
                self
            }
        }
    }

    pub fn get_index(&self) -> Result<usize, &'static str> {
        match &self {
            Axes::Axes2d(ax2d) => ax2d.get_index(),
        }
    }

    pub fn set_title(mut self, title: &str) {
        match &mut self {
            Axes::Axes2d(ax2d) => ax2d.set_title(title),
        };
    }

    pub fn get_title(&self) -> Result<String, &'static str> {
        match &self {
            Axes::Axes2d(ax2d) => ax2d.get_title(),
        }
    }

    pub fn set_xlabel(mut self, xlabel: &str) {
        match &mut self {
            Axes::Axes2d(ax2d) => ax2d.set_xlabel(xlabel),
        }
    }

    pub fn get_xlabel(&self) -> Result<String, &'static str> {
        match &self {
            Axes::Axes2d(ax2d) => ax2d.get_xlabel(),
        }
    }

    pub fn set_ylabel(mut self, ylabel: &str) {
        match &mut self {
            Axes::Axes2d(ax2d) => ax2d.set_ylabel(ylabel),
        }
    }

    pub fn get_ylabel(&self) -> Result<String, &'static str> {
        match &self {
            Axes::Axes2d(ax2d) => ax2d.get_ylabel(),
        }
    }

    pub fn get_plot_data(&self) -> Result<&PlotData<'p, T>, &'static str> {
        match &self {
            Axes::Axes2d(ax2d) => ax2d.get_plot_data(),
        }
    }

    pub fn scatter(self, x: &'p [T], y: &'p [T]) -> Self {
        match self {
            Axes::Axes2d(ax2d) => Axes::Axes2d(ax2d.scatter(x, y)),
        }
    }
    /*
        pub fn set_xdata(mut self, x_data: &'p [T]) {
            match &mut self {
                Axes::Axes2d(ax2d) => ax2d.set_xdata(x_data),
            }
        }

        pub fn set_ydata(mut self, y_data: &'p [T]) {
            match &mut self {
                Axes::Axes2d(ax2d) => ax2d.set_ydata(y_data),
            }
        }
    */
    pub fn identify(&self) -> String {
        match &self {
            Axes::Axes2d(ax2d) => ax2d.identify(),
        }
    }
}
