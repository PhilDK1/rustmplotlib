// use pyo3::prelude::*;
// use pyo3::types::PyAny;

pub struct Colormap {
    // https://matplotlib.org/3.2.2/api/_as_gen/matplotlib.colors.Colormap.html#matplotlib.colors.Colormap
    pub name: String,
    pub n: Option<usize>,
}

impl Colormap {
    pub fn new(name: String, quantization: Option<usize>) -> Colormap {
        Colormap {
            name,
            n: quantization,
        }
    }
}


// pub enum Colors {
//     // https://matplotlib.org/3.2.2/tutorials/introductory/customizing.html?highlight=scatter.edgecolors#a-sample-matplotlibrc-file
//     ColorString(String),
//     RGB(f32, f32, f32),
//     Hex(String),
//     GrayScale(f32),
//     LegalHTML(String),
// }