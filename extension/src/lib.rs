use pyo3::prelude::*;

/// This is a :py:class:`MyClass` class
///
/// Arguments
/// ---------
/// value : int
///   The value of the class
///
/// Returns
/// -------
/// MyClass
///   The class
///
#[pyclass]
struct MyClass {
    value: i32,
}

#[pymethods]
impl MyClass {
    /// docstring for get_value
    ///
    #[getter]
    fn get_value(&self) -> PyResult<i32> {
        Ok(self.value)
    }

    /// docstring for set_value **avoid it**!!!
    ///
    #[setter]
    fn set_value(&mut self, value: i32) -> PyResult<()> {
        self.value = value;
        Ok(())
    }

    #[new]
    fn new(value: i32) -> Self {
        Self { value }
    }

    /// This is a constructor
    ///
    /// Arguments
    /// ---------
    /// value : int
    ///   The value of the class
    ///
    /// Returns
    /// -------
    /// MyClass
    ///   The class
    ///
    #[staticmethod]
    fn constructor(value: i32) -> Self {
        Self::new(value)
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

/// Add two integers
///
/// Arguments
/// ---------
/// a : int
///   The first integer
/// b : int
///   The second integer
///
/// Returns
/// -------
/// int
///   The sum of the two integers
///
#[pyfunction]
fn add(a: i32, b: i32) -> i32 {
    a + b
}

/// Return the version of the Rust crate
///
/// This function is a part of Python module `extension`.
///
/// Returns
/// -------
/// str
///    The version of the Rust crate
///
/// Raises
/// ------
/// None
///   This function does not raise any exceptions
///
/// Example
/// -------
///
/// .. code-block:: python
///
///   from savant_rs.draw_spec import PaddingDraw
///   padding = PaddingDraw(1, 2, 3, 4)
///   padding_copy = padding.copy()
///
#[pyfunction]
fn version() -> String {
    format!("{}", env!("CARGO_PKG_VERSION"))
}

/// This module is a part of Python module `extension`.
///
/// It contains the class :py:class:`MyClass` and the
/// functions :py:func:`add`, :py:func:`version`.
///
/// You can use the regular Sphinx syntax to build relationships between objects.
///
#[pymodule]
fn extension(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<MyClass>()?;
    m.add_function(wrap_pyfunction!(add, m)?)?;
    m.add_function(wrap_pyfunction!(version, m)?)?;
    Ok(())
}
