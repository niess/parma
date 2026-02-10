# The Parma Python package

(**P**HITS-based **A**nalytical **R**adiation **M**odel in the **A**tmosphere)


## Description

Parma is a Python package that wraps the PARMA library, for calculating
atmospheric cosmic-ray spectra (see, for example, [T. Sato, 2016][SAT16]).
This software can calculate terrestrial cosmic ray fluxes of neutrons, protons,
ions with charge up to 28 (Ni), muons, electrons, positrons, and photons, at
nearly any time and in any location in the Earth's atmosphere.


## Installation

Binary distributions of Parma are available as [release][LATEST_RELEASE] assets,
for Linux, OSX and Windows. For instance, on an `x86_64` Linux Parma can be
installed as

```
pip install https://github.com/niess/parma/releases/download/v0.1.1/parma-0.1.1-cp38-abi3-manylinux2014_x86_64.manylinux_2_17_x86_64.whl
```

Alternatively, in order to build Parma from the source, the [Rust
toolchain][RUST_TOOLCHAIN] is required.


## Quick start

```python
>>> import numpy as np
>>> import parma

>>> meter = parma.Fluxmeter(latitude=45, longitude=3, date="2020-12-25")
>>> energy = np.geomspace(1E-06, 1E+03, 91)  # MeV
>>> flux = meter.flux("n", energy)           # 1/(MeV/n)/cm2/s

```


## License

The Parma wrapper (**A**) is under the GNU LGPLv3 license. See the provided
[LICENSE][LICENSE] and [COPYING.LESSER][COPYING] files. The PARMA library
(**B**) is under the [EXPACS/EXPACS-V][EXPACS_LICENSE] license. Using the Parma
Python package thus implies compliance with both **A** and **B**.


[COPYING]: https://github.com/niess/parma/blob/master/COPYING.LESSER
[EXPACS_LICENSE]: https://phits.jaea.go.jp/expacs/
[LICENSE]: https://github.com/niess/parma/blob/master/LICENSE
[LATEST_RELEASE]: https://github.com/niess/parma/releases/tag/v0.1.1
[RUST_TOOLCHAIN]: https://www.rust-lang.org/tools/install
[SAT16]: https://doi.org/10.1371/journal.pone.0160390
