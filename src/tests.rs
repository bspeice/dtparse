use pyo3::ObjectProtocol;
use pyo3::PyDict;
use pyo3::PyList;
use pyo3::PyObject;
use pyo3::Python;
use pyo3::FromPyObject;

use tokenize;
use parse;

macro_rules! test_split {
    ($py: ident, $timelex: ident, $s: expr) => {
        let f = $timelex.call_method1($py, "split", $s).unwrap();
        let l: &PyList = f.extract($py).unwrap();
        let s: Vec<String> = l.iter().map(|i| format!("{}", i)).collect();

        assert_eq!(s, tokenize($s));
    };
}

#[test]
fn test_split() {
    let gil = Python::acquire_gil();
    let py = gil.python();

    let module = py.import("dateutil.parser").unwrap();
    let t: PyObject = module.get("_timelex").unwrap().extract().unwrap();

    // TODO: Fix disagreement about whether or not to replace commas with periods
    // test_split!(py, t, "24, 50, ABC");
    test_split!(py, t, "2018.5.15");
    test_split!(py, t, "May 5, 2018");
    test_split!(py, t, "Mar. 5, 2018");
    test_split!(py, t, "19990101T23");
    test_split!(py, t, "19990101T2359");
}

macro_rules! test_parse_naive {
    ($py: ident, $parser: ident, $s: expr) => {
        let dt: PyObject = $parser.call_method1("parse", $s).unwrap().extract().unwrap();
        let dt_s: String = dt.call_method1($py, "isoformat", " ").unwrap().extract($py).unwrap();
        let s = format!("{}", dt_s);

        println!("{}", s);

        let r_rs = parse($s);
        if r_rs.is_err() {
            println!("{:?}", r_rs);
            assert!(false);
        }

        let rs = r_rs.unwrap();
        assert_eq!(rs.1, None);
        assert_eq!(s, format!("{}", rs.0));
    };
}

#[test]
fn test_parse() {
    let gil = Python::acquire_gil();
    let py = gil.python();
    let parser = py.import("dateutil.parser").unwrap();

    test_parse_naive!(py, parser, "2018.5.15");
    test_parse_naive!(py, parser, "May 5, 2018");
    test_parse_naive!(py, parser, "Mar. 5, 2018");
    test_parse_naive!(py, parser, "19990101T23");
    test_parse_naive!(py, parser, "19990101T2359");
}