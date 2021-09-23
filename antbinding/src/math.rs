use nalgebra::Isometry2;
use pyo3::class::basic::PyObjectProtocol;
use pyo3::prelude::*;
use std::sync::Arc;

#[pyclass(name = "Vector2", subclass)]
#[derive(Clone)]
pub struct PyVector2 {
    pub inner: Arc<common::Vector2>,
}

#[pymethods]
impl PyVector2 {
    #[new]
    pub(crate) fn py_new(x: Option<f32>, y: Option<f32>) -> Self {
        Self {
            inner: Arc::new(common::Vector2 {
                x: x.unwrap_or(0.),
                y: y.unwrap_or(0.),
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

    #[setter]
    fn set_x(&mut self, x: f32) -> PyResult<()> {
        unsafe {
            Arc::get_mut_unchecked(&mut self.inner).x = x;
        }
        Ok(())
    }

    #[setter]
    fn set_y(&mut self, y: f32) -> PyResult<()> {
        unsafe {
            Arc::get_mut_unchecked(&mut self.inner).y = y;
        }
        Ok(())
    }
}

#[pyproto]
impl PyObjectProtocol for PyVector2 {
    fn __repr__(&self) -> PyResult<String> {
        Ok(format!("{:?}", self.inner))
    }
}

impl From<&PyVector2> for common::Vector2 {
    fn from(py_vector: &PyVector2) -> Self {
        py_vector.inner.as_ref().clone()
    }
}

impl From<common::Vector2> for PyVector2 {
    fn from(vector: common::Vector2) -> Self {
        PyVector2 {
            inner: Arc::new(vector),
        }
    }
}

#[pyclass(name = "Pose", subclass)]
#[derive(Clone)]
pub struct PyPose {
    pub inner: Arc<common::Pose>,
}

#[pymethods]
impl PyPose {
    #[new]
    pub(crate) fn py_new(x: Option<f32>, y: Option<f32>, rotation: Option<f32>) -> Self {
        PyPose {
            inner: Arc::new(common::Pose {
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

    #[setter]
    fn set_x(&mut self, x: f32) -> PyResult<()> {
        unsafe {
            Arc::get_mut_unchecked(&mut self.inner).x = x;
        }
        Ok(())
    }

    #[setter]
    fn set_y(&mut self, y: f32) -> PyResult<()> {
        unsafe {
            Arc::get_mut_unchecked(&mut self.inner).y = y;
        }
        Ok(())
    }

    #[setter]
    fn set_rotation(&mut self, rotation: f32) -> PyResult<()> {
        unsafe {
            Arc::get_mut_unchecked(&mut self.inner).rotation = rotation;
        }
        Ok(())
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
        py_pose.inner.as_ref().clone()
    }
}

impl From<&PyPose> for Isometry2<f32> {
    fn from(py_pose: &PyPose) -> Self {
        let pose = py_pose.inner.as_ref();
        Isometry2::new(nalgebra::Vector2::new(pose.x, pose.y), pose.rotation)
    }
}

impl From<common::Pose> for PyPose {
    fn from(pose: common::Pose) -> Self {
        PyPose {
            inner: Arc::new(pose),
        }
    }
}
