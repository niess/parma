from datetime import date
from numpy.testing import assert_allclose
import parma


def test_fluxmeter():
    """Test the Fluxmeter interface."""

    meter = parma.Fluxmeter()
    assert meter.date == date(2000, 1, 1)
    assert meter.latitude == 0.0
    assert meter.longitude == 0.0
    assert meter.altitude == 0.0
    assert meter.geometry == 0.0

    # Test the date getter & setter.
    meter.date = "1978-08-16"
    assert meter.date == date(1978, 8, 16)

    # Test the geometry getter & setter.
    meter.geometry = 0.2
    assert meter.geometry == 0.2
    meter.geometry = "cabin"
    assert meter.geometry == "cabin"

    # Test the atmospheric depth.
    meter.altitude = 0.0 # cm
    assert meter.atmospheric_depth == 1033.227453 # g/cm2
    meter.altitude = 1E+06 # cm
    assert meter.atmospheric_depth == 270.2145993 # g/cm2

    # Test the cutoff rigidity.
    meter.latitude = 0.0 # deg
    assert_allclose(meter.cutoff_rigidity, 13600.0) # MV
    meter.latitude = 45.0 # deg
    assert_allclose(meter.cutoff_rigidity, 4940.0) # MV

    # Test the particle parser.
    assert meter.flux("n", 1.0) == meter.flux(2112, 1.0)
    assert meter.flux("p", 1.0) == meter.flux(2212, 1.0)
    assert meter.flux("mu+", 1.0) == meter.flux(-13, 1.0)
    assert meter.flux("mu-", 1.0) == meter.flux(13, 1.0)
    assert meter.flux("e+", 1.0) == meter.flux(-11, 1.0)
    assert meter.flux("e-", 1.0) == meter.flux(11, 1.0)
    assert meter.flux("gamma", 1.0) == meter.flux(22, 1.0)

    # Test the grid option.
    flux = meter.flux("n", (1.0, 1.0), (0.0, 45.0), grid=False)
    assert flux.shape == (2,)
    flux = meter.flux("n", (1.0, 1.0), (0.0, 45.0, 90.0), grid=True)
    assert flux.shape == (2, 3)
    assert_allclose(flux[0,:], flux[1,:])
    flux = meter.flux("n", (1.0, 10.0), (45.0, 45.0, 45.0), grid=True)
    assert flux.shape == (2, 3)
    assert_allclose(flux[:,0], flux[:,1])
    assert_allclose(flux[:,0], flux[:,2])

    # Test the 511 kev emission line.
    meter = parma.Fluxmeter()
    assert_allclose(meter.flux(22, "me"), 0.00550347, atol=1E-06)
    assert_allclose(meter.flux(22, "me", theta=0), 0.00110993, atol=1E-06)
