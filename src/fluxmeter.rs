use chrono::{Datelike, NaiveDate};
use crate::geometry::Geometry;
use crate::numpy::{AnyArray, ArrayMethods, NewArray};
use crate::parma;
use crate::particles::{Particle, ParticleArg};
use pyo3::exceptions::{PyTypeError, PyValueError};
use pyo3::prelude::*;
use std::ffi::{c_double, c_int};


/// An atmospheric cosmic-ray's fluxmeter.
#[pyclass]
pub struct Fluxmeter {
    /// The observation date.
    #[pyo3(get)]
    date: NaiveDate,

    /// The observer latitude, in deg.
    #[pyo3(get, set)]
    latitude: c_double,

    /// The observer longitude, in deg.
    #[pyo3(get, set)]
    longitude: c_double,

    /// The observer altitude, in cm.
    #[pyo3(get, set)]
    altitude: c_double,

    /// The local geometry (neutrons, only).
    #[pyo3(get, set)]
    geometry: Geometry,
}

#[derive(FromPyObject)]
enum DateArg {
    String(String),
    Date(NaiveDate),
}

#[pymethods]
impl Fluxmeter {
    #[pyo3(
        signature=(/, *, date=None, latitude=None, longitude=None, altitude=None, geometry=None)
    )]
    #[new]
    fn new(
        date: Option<DateArg>,
        latitude: Option<c_double>,
        longitude: Option<c_double>,
        altitude: Option<c_double>,
        geometry: Option<Geometry>,
    ) -> PyResult<Self> {
        let date = match date {
            Some(date) => date.try_into()?,
            None => NaiveDate::from_ymd_opt(2000, 1, 1).unwrap(),
        };
        let latitude = latitude.unwrap_or(0.0);
        let longitude = longitude.unwrap_or(0.0);
        let altitude = altitude.unwrap_or(0.0);
        let geometry = geometry.unwrap_or_else(|| Geometry::default());
        let location = Self { date, latitude, longitude, altitude, geometry };
        Ok(location)
    }

    /// The atmospheric depth, in g/cm2.
    #[getter]
    fn get_atmospheric_depth(&self) -> c_double {
        unsafe { parma::getd(self.altitude * Self::CM_TO_KM, self.latitude) }
    }

    /// The vertical cutoff rigidity, in MV.
    #[getter]
    fn get_cutoff_rigidity(&self) -> c_double {
        unsafe { parma::getr(self.latitude, self.longitude) * Self::GV_TO_MV }
    }

    /// The solar activity (W-index).
    #[getter]
    fn get_solar_activity(&self) -> c_double {
        unsafe {
            parma::getHP(
                self.date.year() as c_int,
                self.date.month() as c_int,
                self.date.day() as c_int,
            )
        }
    }

    #[setter]
    fn set_date(&mut self, date: DateArg) -> PyResult<()> {
        self.date = date.try_into()?;
        Ok(())
    }

    /// Computes the local flux.
    #[pyo3(signature=(particle, energy, theta=None, *, atmospheric_depth=None, cutoff_rigidity=None, grid=false, solar_activity=None))]
    fn flux<'py>(
        &self,
        particle: ParticleArg,
        energy: AnyArray<'py, c_double>,
        theta: Option<AnyArray<'py, c_double>>,
        atmospheric_depth: Option<c_double>,
        cutoff_rigidity: Option<c_double>,
        grid: Option<bool>,
        solar_activity: Option<c_double>,
    ) -> PyResult<NewArray<'py, c_double>> {
        let py = energy.py();
        let grid = grid.unwrap_or(false);
        let particle: Particle = particle.try_into()?;
        let geometry: c_double = self.geometry.into();
        let (m, iang, shape) = match &theta {
            Some(theta) => {
                let iang = particle.angular_index();
                if grid {
                    let mut shape = energy.shape();
                    shape.append(&mut theta.shape());
                    (theta.size(), iang, shape)
                } else if theta.shape().is_empty() || (theta.size() == energy.size()) {
                    (0, iang, energy.shape())
                } else {
                    let msg = format!(
                        "bad theta (expected a size {} array, found size {})",
                        energy.size(),
                        theta.size(),
                    );
                    return Err(PyTypeError::new_err(msg))
                }
            },
            None => (0, 0, energy.shape()),
        };
        let particle = particle as c_int;
        let r = cutoff_rigidity
            .unwrap_or_else(|| unsafe {
                parma::getr(self.latitude, self.longitude)
            });
        let d = atmospheric_depth
            .unwrap_or_else(|| unsafe {
                parma::getd(self.altitude * Self::CM_TO_KM, self.latitude)
            });
        let s = solar_activity
            .unwrap_or_else(|| unsafe {
                parma::getHP(
                    self.date.year() as c_int,
                    self.date.month() as c_int,
                    self.date.day() as c_int,
                )
            });
        const DEG: c_double = (std::f64::consts::PI / 180.0) as c_double;
        let mut array = NewArray::empty(py, shape)?;
        let flux = array.as_slice_mut();
        for i in 0..energy.size() {
            let ei = energy.get_item(i)?;
            let fi = unsafe { parma::getSpec(particle, s, r, d, ei, geometry) };
            match &theta {
                Some(theta) => if grid {
                    for j in 0..m {
                        let cj = (theta.get_item(j)? * DEG).cos();
                        let aij = unsafe {
                            parma::getSpecAngFinal(iang, s, r, d, ei, geometry, cj)
                        };
                        flux[i * m + j] = fi * aij;
                    }
                } else {
                    let ci = (theta.get_item(i)? * DEG).cos();
                    let ai = unsafe {
                        parma::getSpecAngFinal(iang, s, r, d, ei, geometry, ci)
                    };
                    flux[i] = fi * ai;
                },
                None => flux[i] = fi,
            }
        }
        Ok(array)
    }
}

impl Fluxmeter {
    const CM_TO_KM: c_double = 1E-05;
    const GV_TO_MV: c_double = 1E+03;
}

impl TryFrom<DateArg> for NaiveDate {
    type Error = PyErr;

    fn try_from(date: DateArg) -> PyResult<Self> {
        let date = match date {
            DateArg::String(date) => NaiveDate::parse_from_str(date.as_str(), "%Y-%m-%d")
                .map_err(|msg| {
                    let msg = format!("bad date: {}", msg);
                    PyValueError::new_err(msg)
                })?,
            DateArg::Date(date) => date,
        };
        Ok(date)
    }
}
