#![feature(get_mut_unchecked)]

use common;
use pyo3::class::basic::PyObjectProtocol;
use pyo3::exceptions::{FileNotFoundError, TypeError};
use pyo3::prelude::*;
use std::convert::From;
use std::fs::File;
use std::io::{BufReader, BufWriter};
use std::rc::Rc;

#[pyclass(name=Pose,subclass)]
#[derive(Clone)]
pub struct PyPose {
    pub inner: Rc<common::Pose>,
}

#[pymethods]
impl PyPose {
    #[new]
    fn py_new(x: Option<f32>, y: Option<f32>, rotation: Option<f32>) -> Self {
        PyPose {
            inner: Rc::new(common::Pose {
                x: x.unwrap_or(0.),
                y: y.unwrap_or(0.),
                rotation: rotation.unwrap_or(0.),
            }),
        }
    }

    #[getter]
    fn get_x(&self) -> PyResult<f32> {
        Ok(self.inner.x)
    }

    #[getter]
    fn get_y(&self) -> PyResult<f32> {
        Ok(self.inner.y)
    }

    #[getter]
    fn get_rotation(&self) -> PyResult<f32> {
        Ok(self.inner.rotation)
    }
}

#[pyproto]
impl PyObjectProtocol for PyPose {
    fn __repr__(&self) -> PyResult<String> {
        Ok(format!("{:?}", self.inner))
    }
}

impl From<&PyPose> for common::Pose {
    fn from(py_pose: &PyPose) -> Self {
        let pose = py_pose.inner.as_ref();
        common::Pose {
            x: pose.x,
            y: pose.y,
            rotation: pose.rotation,
        }
    }
}

impl From<common::Pose> for PyPose {
    fn from(pose: common::Pose) -> Self {
        PyPose {
            inner: Rc::new(pose),
        }
    }
}

#[pyclass(name=Map,subclass)]
pub struct PyMap {
    pub inner: Rc<common::Map>,
}

#[pymethods]
impl PyMap {
    #[getter]
    fn get_width(&self) -> PyResult<f32> {
        Ok(self.inner.width)
    }

    #[getter]
    fn get_height(&self) -> PyResult<f32> {
        Ok(self.inner.height)
    }

    #[setter]
    fn set_width(&mut self, width: f32) -> PyResult<()> {
        unsafe {
            Rc::get_mut_unchecked(&mut self.inner).width = width;
        }
        // Save alternative
        // Rc::get_mut(&mut self.inner)
        //     .ok_or(PyErr::new::<TypeError, _>(
        //         "Could not lease mutable reference",
        //     ))?
        //     .width = width;
        Ok(())
    }

    #[setter]
    fn set_height(&mut self, height: f32) -> PyResult<()> {
        unsafe {
            Rc::get_mut_unchecked(&mut self.inner).height = height;
        }
        Ok(())
    }
}

#[pyproto]
impl PyObjectProtocol for PyMap {
    fn __repr__(&self) -> PyResult<String> {
        Ok(format!("{:?}", self.inner))
    }
}

#[pyclass(name=Frame,subclass)]
#[derive(Clone)]
pub struct PyFrame {
    ants: Vec<PyPose>,
    anthills: Vec<PyPose>,
    raspberries: Vec<PyPose>,
    sugar_hills: Vec<PyPose>,
}

impl From<&PyFrame> for common::Frame {
    fn from(py_frame: &PyFrame) -> Self {
        common::Frame {
            ants: py_frame.ants.iter().map(common::Pose::from).collect(),
            anthills: py_frame.anthills.iter().map(common::Pose::from).collect(),
            raspberries: py_frame
                .raspberries
                .iter()
                .map(common::Pose::from)
                .collect(),
            sugar_hills: py_frame
                .sugar_hills
                .iter()
                .map(common::Pose::from)
                .collect(),
        }
    }
}

impl From<common::Frame> for PyFrame {
    fn from(frame: common::Frame) -> Self {
        let common::Frame {
            ants,
            anthills,
            raspberries,
            sugar_hills,
        } = frame;
        PyFrame {
            ants: ants.into_iter().map(PyPose::from).collect(),
            anthills: anthills.into_iter().map(PyPose::from).collect(),
            raspberries: raspberries.into_iter().map(PyPose::from).collect(),
            sugar_hills: sugar_hills.into_iter().map(PyPose::from).collect(),
        }
    }
}

#[pymethods]
impl PyFrame {
    #[new]
    fn py_new() -> Self {
        PyFrame {
            ants: vec![],
            anthills: vec![],
            raspberries: vec![],
            sugar_hills: vec![],
        }
    }

    #[getter]
    fn get_ants(&self) -> PyResult<Vec<PyPose>> {
        Ok(self.ants.clone())
    }

    #[getter]
    fn get_anthills(&self) -> PyResult<Vec<PyPose>> {
        Ok(self.anthills.clone())
    }

    #[getter]
    fn get_raspberries(&self) -> PyResult<Vec<PyPose>> {
        Ok(self.raspberries.clone())
    }

    #[getter]
    fn get_sugar_hills(&self) -> PyResult<Vec<PyPose>> {
        Ok(self.sugar_hills.clone())
    }

    fn add_ant(&mut self, pose: PyPose) -> PyResult<()> {
        self.ants.push(pose);
        Ok(())
    }

    fn add_anthill(&mut self, pose: PyPose) -> PyResult<()> {
        self.anthills.push(pose);
        Ok(())
    }

    fn add_raspberry(&mut self, pose: PyPose) -> PyResult<()> {
        self.raspberries.push(pose);
        Ok(())
    }

    fn add_sugar_hill(&mut self, pose: PyPose) -> PyResult<()> {
        self.sugar_hills.push(pose);
        Ok(())
    }
}

#[pyclass(name=Recording,subclass)]
pub struct PyRecording {
    map: PyMap,
    frames: Vec<PyFrame>,
}

#[pymethods]
impl PyRecording {
    #[new]
    fn py_new() -> Self {
        PyRecording {
            map: PyMap {
                inner: Rc::new(common::Map {
                    width: 128.,
                    height: 128.,
                }),
            },
            frames: vec![],
        }
    }

    // props
    #[getter]
    fn map(&self) -> PyResult<PyMap> {
        Ok(PyMap {
            inner: self.map.inner.clone(),
        })
    }

    #[getter]
    fn frames(&self) -> PyResult<Vec<PyFrame>> {
        Ok(self.frames.iter().cloned().collect())
    }

    #[text_signature = "(self, frame, /)"]
    fn add_frame(&mut self, frame: PyFrame) -> PyResult<()> {
        self.frames.push(frame);
        Ok(())
    }

    // io
    #[staticmethod]
    #[text_signature = "(filename, /)"]
    fn load(filename: &str) -> PyResult<PyRecording> {
        let file = File::open(filename)
            .map_err(|msg| PyErr::new::<FileNotFoundError, _>(msg.to_string()))?;
        let reader = BufReader::new(file);
        let common::Recording { map, frames } =
            common::Recording::load(reader).map_err(PyErr::new::<TypeError, _>)?;

        Ok(PyRecording {
            map: PyMap {
                inner: Rc::new(map),
            },
            frames: frames.into_iter().map(PyFrame::from).collect(),
        })
    }

    #[text_signature = "(self, filename, /)"]
    fn dump(&self, filename: &str) -> PyResult<()> {
        let file = File::create(filename)
            .map_err(|msg| PyErr::new::<FileNotFoundError, _>(msg.to_string()))?;
        let writer = BufWriter::new(file);
        let recording = common::Recording {
            map: (*self.map.inner).clone(),
            frames: self
                .frames
                .iter()
                .map(|frame| common::Frame::from(frame))
                .collect(),
        };
        recording.dump(writer).map_err(PyErr::new::<TypeError, _>)?;
        Ok(())
    }
}

#[pymodule]
fn antbinding(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_class::<PyPose>()?;
    m.add_class::<PyFrame>()?;
    m.add_class::<PyMap>()?;
    m.add_class::<PyRecording>()?;
    Ok(())
}
