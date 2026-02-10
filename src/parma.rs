use pyo3::prelude::*;
use pyo3::exceptions::PySystemError;
use std::ffi::{c_char, c_double, c_int, CString};
use std::path::Path;


pub fn initialise() -> PyResult<()> {
    let filename = match process_path::get_dylib_path() {
        Some(path) => path
                        .to_string_lossy()
                        .to_string(),
        None => return Err(PySystemError::new_err("could not resolve module path")),
    };
    let prefix = Path::new(&filename)
        .parent()
        .unwrap_or_else(|| Path::new("."))
        .join("data");
    let prefix = CString::new(prefix.to_str().unwrap()).unwrap();
    unsafe { set_prefix(prefix.as_ptr()); }

    Ok(())
}

#[link(name = "parma-cpp")]
unsafe extern "C" {
    #[link_name="getdcpp"]
    pub fn getd(alti: c_double, cido: c_double) -> c_double;

    #[link_name="getHPcpp"]
    pub fn getHP(iy0: c_int, im0: c_int, id0: c_int) -> c_double;

    #[link_name="get511fluxCpp"]
    pub fn get511flux(s: c_double, r: c_double, d: c_double) -> c_double;

    #[link_name="getSpecCpp"]
    pub fn getSpec(
        ip: c_int,
        s: c_double,
        r: c_double,
        d: c_double,
        e: c_double,
        g: c_double,
    ) -> c_double;

    #[link_name="getSpecAngFinalCpp"]
    pub fn getSpecAngFinal(
        ia: c_int,
        s: c_double,
        r: c_double,
        d: c_double,
        e: c_double,
        g: c_double,
        ang: c_double,
    ) -> c_double;

    #[link_name="getrcpp"]
    pub fn getr(cido: c_double, ckei: c_double) -> c_double;

    pub fn set_prefix(value: *const c_char);
}
