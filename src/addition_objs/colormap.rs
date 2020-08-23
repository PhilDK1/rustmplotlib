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

// impl<'p> Colormap {
//     fn to_pyand(&self, mpl: &'p PyModule, py: Python<'p>) -> &PyAny {
//         mpl.call_method("colormap", (self.name, self.N), None).map_err(|e| {
//             // reads pythons returned errors and prints them
//             e.print_and_set_sys_last_vars(py);
//         }).unwrap()
//     }
// }