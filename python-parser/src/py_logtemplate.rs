use syslog_ng_common::LogTemplate;
use syslog_ng_common::LogMessage;

use cpython::{Python, ToPyObject, NoArgs, PyObject, PyResult, PyString};
use cpython::rustobject::{TypeBuilder, PyRustObject};

fn format(py: Python,
          slf: &PyRustObject<LogTemplate>,
          msg: &PyRustObject<LogMessage>)
          -> PyResult<PyString> {

    Ok(PyString::new(py, "kittens"))
}

pub struct PyLogTemplate(PyRustObject<LogTemplate>);

impl PyLogTemplate {
    pub fn new(py: Python, logmsg: LogTemplate) -> PyResult<PyLogTemplate> {
        let mut b = TypeBuilder::<LogTemplate>::new(py, "PyLogTemplate");
        b.add("format", py_method!(format(msg: &PyRustObject<LogMessage>)));
        trace!("Trying to finish construction PyLogTemplate");
        let built_type = try!(b.finish());
        let instance = built_type.create_instance(py, logmsg, ());
        Ok(PyLogTemplate(instance))
    }
}

impl ToPyObject for PyLogTemplate {
    type ObjectType = PyObject;
    fn to_py_object(&self, py: Python) -> Self::ObjectType {
        self.0.to_py_object(py)
    }
    fn into_py_object(self, _py: Python) -> PyObject {
        self.0.into_py_object(_py)
    }
}
