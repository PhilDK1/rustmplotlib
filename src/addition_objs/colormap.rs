pub struct Colormap {
    // https://matplotlib.org/3.2.2/api/_as_gen/matplotlib.colors.Colormap.html#matplotlib.colors.Colormap
    name: String,
    N: usize,
}

impl Colormap {
    pub fn new(name: String, quantization: usize) -> Colormap {
        Colormap {
            name,
            N: quantization
        }
    }
}

impl pyo3::conversion::ToPyObject for Colormap {
    fn to_object(&self, py: Python) -> PyObject {
        
    }
}