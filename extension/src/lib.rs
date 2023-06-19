use pyo3::prelude::*;

#[pyclass]
struct MyClass {
    #[pyo3(get)]
    value: i32,
}

#[pymethods]
impl MyClass {
    #[new]
    fn new(value: i32) -> Self {
        Self { value }
    }

    fn add(&self, other: &Self) -> Self {
        Self {
            value: self.value + other.value,
        }
    }

    #[staticmethod]
    fn static_method() -> MyClass {
        MyClass { value: 0 }
    }
}

#[pyfunction]
fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[pyfunction]
fn version() -> String {
    format!("{}", env!("CARGO_PKG_VERSION"))
}

#[pymodule]
fn extension(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<MyClass>()?;
    m.add_function(wrap_pyfunction!(add, m)?)?;
    m.add_function(wrap_pyfunction!(version, m)?)?;
    Ok(())
}
