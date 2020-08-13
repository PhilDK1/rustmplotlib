use pyo3::prelude::*;

pub struct Env {
    pub gil: GILGuard,
}

impl Env {
    pub fn new() -> Env {
        Env {
            gil: Python::acquire_gil(),
        }
    }
}

impl Default for Env {
    fn default() -> Self {
        Self::new()
    }
}
