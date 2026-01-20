use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use std::ffi::c_double;


#[derive(Clone, Copy)]
pub enum Geometry {
    Blackhole,
    Cabin,
    Humidity(c_double),
    NoGround,
    Pilot,
}

impl Default for Geometry {
    fn default() -> Self {
        Self::Humidity(0.0)
    }
}

impl<'py> FromPyObject<'py> for Geometry {
    fn extract_bound(ob: &Bound<'py, PyAny>) -> PyResult<Self> {
        #[derive(FromPyObject)]
        enum Arg {
            Float(c_double),
            String(String),
        }
        let geometry = match ob.extract::<Arg>()? {
            Arg::Float(humidity) => Self::Humidity(humidity),
            Arg::String(arg) => match arg.as_str() {
                "blackhole" => Self::Blackhole,
                "cabin" => Self::Cabin,
                "no_ground" => Self::NoGround,
                "pilot" => Self::Pilot,
                _ => {
                    let msg = format!("bad geometry ('{}')", arg);
                    return Err(PyValueError::new_err(msg))
                },
            },
        };
        Ok(geometry)
    }
}

impl<'py> IntoPyObject<'py> for Geometry {
    type Target = PyAny;
    type Output = Bound<'py, PyAny>;
    type Error = PyErr;

    fn into_pyobject(self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        let geometry = match self {
            Self::Blackhole => "blackhole".into_pyobject(py)?.into_any(),
            Self::Cabin => "cabin".into_pyobject(py)?.into_any(),
            Self::Humidity(humidity) => humidity.into_pyobject(py)?.into_any(),
            Self::NoGround => "no_ground".into_pyobject(py)?.into_any(),
            Self::Pilot => "pilot".into_pyobject(py)?.into_any(),
        };
        Ok(geometry)
    }
}

impl From<Geometry> for c_double {
    fn from(value: Geometry) -> Self {
        match value {
            Geometry::Blackhole => 100.0,
            Geometry::Cabin => -11.0,
            Geometry::Humidity(h) => h,
            Geometry::NoGround => 10.0,
            Geometry::Pilot => -1.0,
        }
    }
}
