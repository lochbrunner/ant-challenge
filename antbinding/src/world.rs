use crate::{PyFrame, PyMap, PyPose};
use nalgebra::Isometry2;
use pyo3::exceptions::{PyLookupError, PyReferenceError};
use pyo3::prelude::*;
use std::sync::Arc;

use engine::World;

#[pyclass(name = "World", subclass)]
pub struct PyWorld {
    pub(crate) inner: Arc<World>,
}

impl PyWorld {
    fn try_get_mut(&mut self) -> PyResult<&mut World> {
        if let Some(world) = Arc::get_mut(&mut self.inner) {
            Ok(world)
        } else {
            Err(PyErr::new::<PyReferenceError, _>(
                "map was borrowed elsewhere",
            ))
        }
    }
}

#[pymethods]
impl PyWorld {
    #[new]
    pub(crate) fn py_new(width: Option<f32>, height: Option<f32>) -> Self {
        Self {
            inner: Arc::new(World::new(width.unwrap_or(32.), height.unwrap_or(32.))),
        }
    }

    #[getter]
    pub fn get_map(&self) -> PyResult<PyMap> {
        Ok(PyMap::from(&self.inner.map))
    }

    pub fn update(&mut self) -> PyResult<()> {
        self.try_get_mut()?.update();
        Ok(())
    }

    pub fn add_ant_hill(&mut self, position: &PyPose, team: u8) -> PyResult<()> {
        self.try_get_mut()?
            .add_ant_hill(Isometry2::from(position), team);
        Ok(())
    }

    pub fn try_add_ant_hill(&mut self, position: &PyPose, team: u8) -> PyResult<()> {
        if let Some(_) = self
            .try_get_mut()?
            .try_add_ant_hill(Isometry2::from(position), team)
        {
            Ok(())
        } else {
            Err(PyErr::new::<PyLookupError, _>(
                "Can not place ant hill here",
            ))
        }
    }

    pub fn add_ant_hill_mirrored(&mut self, position: &PyPose, team: u8) -> PyResult<()> {
        if let Some(_) = self
            .try_get_mut()?
            .add_ant_hill_mirrored(Isometry2::from(position), team)
        {
            Ok(())
        } else {
            Err(PyErr::new::<PyLookupError, _>(
                "Can not place ant hill here",
            ))
        }
    }

    pub fn add_sugar_hill(&mut self, position: &PyPose) -> PyResult<()> {
        self.try_get_mut()?
            .add_sugar_hill(Isometry2::from(position));
        Ok(())
    }

    pub fn try_add_sugar_hill(&mut self, position: &PyPose) -> PyResult<()> {
        if let Some(_) = self
            .try_get_mut()?
            .try_add_sugar_hill(Isometry2::from(position))
        {
            Ok(())
        } else {
            Err(PyErr::new::<PyLookupError, _>(
                "Can not place sugar hill here",
            ))
        }
    }

    pub fn add_sugar_hill_mirrored(&mut self, position: &PyPose) -> PyResult<()> {
        if let Some(_) = self
            .try_get_mut()?
            .add_sugar_hill_mirrored(Isometry2::from(position))
        {
            Ok(())
        } else {
            Err(PyErr::new::<PyLookupError, _>(
                "Can not place sugar hill mirrored here",
            ))
        }
    }

    pub fn add_raspberry(&mut self, position: &PyPose) -> PyResult<()> {
        self.try_get_mut()?.add_raspberry(Isometry2::from(position));
        Ok(())
    }

    pub fn try_add_raspberry(&mut self, position: &PyPose) -> PyResult<()> {
        if let Some(_) = self
            .try_get_mut()?
            .try_add_raspberry(Isometry2::from(position))
        {
            Ok(())
        } else {
            Err(PyErr::new::<PyLookupError, _>(
                "Can not place raspberry here",
            ))
        }
    }

    pub fn add_raspberry_mirrored(&mut self, position: &PyPose) -> PyResult<()> {
        if let Some(_) = self
            .try_get_mut()?
            .add_raspberry_mirrored(Isometry2::from(position))
        {
            Ok(())
        } else {
            Err(PyErr::new::<PyLookupError, _>(
                "Can not place raspberry here",
            ))
        }
    }

    pub fn snapshot(&self) -> PyResult<PyFrame> {
        Ok(PyFrame::from(self.inner.snapshot()))
    }
}
