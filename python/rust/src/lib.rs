pub use _kolo::*;

#[path = ""]
mod _kolo {
    use pyo3::ffi;
    use pyo3::prelude::*;
    use std::os::raw::c_int;
    use std::ptr;

    // Safety:
    //
    // We match the type signature of `Py_tracefunc`.
    //
    // https://docs.rs/pyo3-ffi/latest/pyo3_ffi/type.Py_tracefunc.html
    extern "C" fn noop_profile(
        _obj: *mut ffi::PyObject,
        _frame: *mut ffi::PyFrameObject,
        _what: c_int,
        _arg: *mut ffi::PyObject,
    ) -> c_int {
        0
    }

    #[pyfunction]
    fn register_noop_profiler() {
        // Safety:
        //
        // PyEval_SetProfile takes two arguments:
        //  * trace_func: Option<Py_tracefunc>
        //  * arg1:       *mut PyObject
        //
        // `noop_profile` matches the signature of a `Py_tracefunc`, so
        // we only need to wrap it in `Some`.
        // `arg1` can accept a NULL pointer, so that's what we pass.
        //
        // PyEval_SetProfile also requires we hold the GIL, so we wrap the
        // `unsafe` block in `Python::with_gil`.
        //
        // https://docs.rs/pyo3-ffi/latest/pyo3_ffi/fn.PyEval_SetProfile.html
        // https://docs.python.org/3/c-api/init.html#c.PyEval_SetProfile
        Python::with_gil(|_py| unsafe {
            ffi::PyEval_SetProfile(Some(noop_profile), ptr::null_mut());
        })
    }

    #[pymodule]
    fn _kolo(_py: Python, m: &PyModule) -> PyResult<()> {
        m.add_function(wrap_pyfunction!(register_noop_profiler, m)?)?;
        Ok(())
    }
}
