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
            n: quantization
        }
    }

    
}

