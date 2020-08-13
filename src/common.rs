#![allow(unused)]
use pyo3::prelude::*;
use pyo3::types::*;
use std::collections::HashMap;

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
