use pyo3::ObjectProtocol;
use pyo3::PyDict;
use pyo3::PyList;
use pyo3::PyObject;
use pyo3::Python;
use pyo3::FromPyObject;

macro_rules! test_split {
    ($py: ident, $timelex: ident, $s: expr, $expected: expr) => {
        let f = $timelex.call_method1($py, "split", $s).unwrap();
        let l: &PyList = f.extract($py).unwrap();
        let s: Vec<String> = l.iter().map(|i| format!("{}", i)).collect();

        assert_eq!(s, $expected);
    };
}

#[test]
fn test_split() {
    let gil = Python::acquire_gil();
    let py = gil.python();

    let module = py.import("dateutil.parser").unwrap();
    let t: PyObject = module.get("_timelex").unwrap().extract().unwrap();

    test_split!(py, t, "24, 50, ABC", vec!["24", ",", " ", "50", ",", " ", "ABC"]);
    test_split!(py, t, "2018.5.15", vec!["2018", ".", "5", ".", "15"]);
    test_split!(py, t, "May 5, 2018", vec!["May", " ", "5", ",", " ", "2018"]);
    test_split!(py, t, "Mar. 5, 2018", vec!["Mar", ".", " ", "5", ",", " ", "2018"]);
    test_split!(py, t, "19990101T23", vec!["19990101", "T", "23"]);
    test_split!(py, t, "19990101T2359", vec!["19990101", "T", "2359"]);
}