use pyo3::prelude::*;
use pyo3::exceptions::PyValueError;
use std::ffi::c_int;


#[derive(Clone, Copy, Debug)]
#[repr(u8)]
pub enum Particle {
    Neutron = 0,
    Proton = 1,
    He = 2,
    Li = 3,
    Be = 4,
    B = 5,
    C = 6,
    N = 7,
    O = 8,
    F = 9,
    Ne = 10,
    Na = 11,
    Mg = 12,
    Al = 13,
    Si = 14,
    P = 15,
    S = 16,
    Cl = 17,
    Ar = 18,
    K = 19,
    Ca = 20,
    Sc = 21,
    Ti = 22,
    V = 23,
    Cr = 24,
    Mn = 25,
    Fe = 26,
    Co = 27,
    Ni = 28,
    MuonPlus = 29,
    MuonMinus = 30,
    Electron = 31,
    Positron = 32,
    Photon = 33,
}

#[derive(FromPyObject)]
pub enum ParticleArg {
    Pid(i32),
    Symbol(String),
}

impl Particle {
    pub fn angular_index(&self) -> c_int {
        match self {
            Self::Neutron => 1,
            Self::Proton => 2,
            Self::He => 3,
            Self::MuonPlus | Self::MuonMinus => 4,
            Self::Electron | Self::Positron => 5,
            Self::Photon => 6,
            _ => 0,
        }
    }
}

impl TryFrom<&str> for Particle {
    type Error = PyErr;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let particle = match value {
            "n" => Particle::Neutron,
            "p" => Particle::Proton,
            "He" => Particle::He,
            "Li" => Particle::Li,
            "Be" => Particle::Be,
            "B" => Particle::B,
            "C" => Particle::C,
            "N" => Particle::N,
            "O" => Particle::O,
            "F" => Particle::F,
            "Ne" => Particle::Ne,
            "Na" => Particle::Na,
            "Mg" => Particle::Mg,
            "Al" => Particle::Al,
            "Si" => Particle::Si,
            "P" => Particle::P,
            "S" => Particle::S,
            "Cl" => Particle::Cl,
            "Ar" => Particle::Ar,
            "K" => Particle::K,
            "Ca" => Particle::Ca,
            "Sc" => Particle::Sc,
            "Ti" => Particle::Ti,
            "V" => Particle::V,
            "Cr" => Particle::Cr,
            "Mn" => Particle::Mn,
            "Fe" => Particle::Fe,
            "Co" => Particle::Co,
            "Ni" => Particle::Ni,
            "mu+" => Particle::MuonPlus,
            "mu-" => Particle::MuonMinus,
            "e-" => Particle::Electron,
            "e+" => Particle::Positron,
            "gamma" => Particle::Photon,
            _ => {
                let msg = format!("bad particle ('{}')", value);
                return Err(PyValueError::new_err(msg))
            },
        };
        Ok(particle)
    }
}

impl TryFrom<i32> for Particle {
    type Error = PyErr;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        let particle = match value {
            -13 => Particle::MuonPlus,
            -11 => Particle::Positron,
            11 => Particle::Electron,
            13 => Particle::MuonMinus,
            22 => Particle::Photon,
            2112 => Particle::Neutron,
            2212 => Particle::Proton,
            _ => {
                let msg = format!("bad particle id ({})", value);
                return Err(PyValueError::new_err(msg))
            },
        };
        Ok(particle)
    }
}

impl TryFrom<ParticleArg> for Particle {
    type Error = PyErr;

    fn try_from(value: ParticleArg) -> Result<Self, Self::Error> {
        let particle = match value {
            ParticleArg::Pid(value) => value.try_into()?,
            ParticleArg::Symbol(value) => value.as_str().try_into()?,
        };
        Ok(particle)
    }
}
