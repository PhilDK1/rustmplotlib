use pyo3::prelude::*;

/// prehaps have a way to specify a particular environment and check if mpl
/// is installed as well as the particular version. Need to check the pyo3
/// docs to determine if that is possible

// represents Python environment
pub struct Env {
    pub gil: GILGuard,
}

impl Env {
    pub fn new() -> Env {
        // gets the environement
        Env {
            gil: Python::acquire_gil(),
        }
    }
}

impl Default for Env {
    // clippy told me to
    fn default() -> Self {
        Self::new()
    }
}
