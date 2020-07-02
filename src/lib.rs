use pyo3::prelude::*;


pub fn builtins_sum(vec:Vec<i32>) -> PyResult<i32> {
    let gil = Python::acquire_gil();
    let py = gil.python();
    let builtins = PyModule::import(py, "builtins")?;
    let total: i32 = builtins.call1("sum", (vec,))?.extract()?;
    Ok(total)
}


pub fn numpy_sum(vec: Vec<i32>) -> PyResult<i32> {
    let gil = Python::acquire_gil();
    let py = gil.python();
    let np = PyModule::import(py, "numpy")?;
    let total: i32 = np.call1("sum", (vec,))?.extract()?;
    Ok(total)
}