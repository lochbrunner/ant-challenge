#![feature(get_mut_unchecked)]

use common;
use pyo3::class::basic::PyObjectProtocol;
use pyo3::exceptions::{PyFileNotFoundError, PyTypeError};
use pyo3::prelude::*;
use std::convert::From;
use std::fs::File;
use std::io::{BufReader, BufWriter};
use std::sync::Arc;

mod math;
mod world;
use math::{PyPose, PyVector2};

#[pyclass(name = "Map", subclass)]
#[derive(Clone)]
pub struct PyMap {
    pub inner: Arc<common::Map>,
}

#[pymethods]
impl PyMap {
    #[new]
    fn py_new() -> Self {
        Self {
            inner: Arc::new(common::Map {
                width: 32.,
                height: 32.,
            }),
        }
    }

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
            Arc::get_mut_unchecked(&mut self.inner).width = width;
        }
        // Save alternative
        // Rc::get_mut(&mut self.inner)
        //     .ok_or(PyErr::new::<PyTypeError, _>(
        //         "Could not lease mutable reference",
        //     ))?
        //     .width = width;
        Ok(())
    }

    #[setter]
    fn set_height(&mut self, height: f32) -> PyResult<()> {
        unsafe {
            Arc::get_mut_unchecked(&mut self.inner).height = height;
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

impl From<&common::Map> for PyMap {
    fn from(map: &common::Map) -> Self {
        PyMap {
            inner: Arc::new(map.clone()),
        }
    }
}

#[pyclass(name = "SugarHill", subclass)]
#[derive(Clone)]
pub struct PySugarHill {
    pose: PyPose,
    volume: f32,
}

#[pymethods]
impl PySugarHill {
    #[new]
    fn py_new(x: Option<f32>, y: Option<f32>, rotation: Option<f32>, volume: Option<f32>) -> Self {
        Self {
            pose: PyPose::py_new(x, y, rotation),
            volume: volume.unwrap_or(12.),
        }
    }

    #[getter]
    fn get_volume(&self) -> PyResult<f32> {
        Ok(self.volume)
    }

    #[getter]
    fn get_pose(&self) -> PyResult<PyPose> {
        Ok(self.pose.clone())
    }

    #[setter]
    fn set_volume(&mut self, volume: f32) -> PyResult<()> {
        self.volume = volume;
        Ok(())
    }

    #[setter]
    fn set_pose(&mut self, pose: PyPose) -> PyResult<()> {
        unsafe {
            Arc::get_mut_unchecked(&mut self.pose.inner).x = pose.inner.x;
            Arc::get_mut_unchecked(&mut self.pose.inner).y = pose.inner.y;
            Arc::get_mut_unchecked(&mut self.pose.inner).rotation = pose.inner.rotation;
        }
        Ok(())
    }
}

impl From<common::SugarHill> for PySugarHill {
    fn from(hill: common::SugarHill) -> Self {
        let common::SugarHill { pose, volume } = hill;
        PySugarHill {
            pose: PyPose::from(pose),
            volume,
        }
    }
}

impl From<&PySugarHill> for common::SugarHill {
    fn from(py_hill: &PySugarHill) -> Self {
        common::SugarHill {
            pose: common::Pose::from(&py_hill.pose),
            volume: py_hill.volume,
        }
    }
}

#[pyproto]
impl PyObjectProtocol for PySugarHill {
    fn __repr__(&self) -> PyResult<String> {
        Ok(format!("{:?}", common::SugarHill::from(self)))
    }
}

#[pyclass(name = "Ant", subclass)]
#[derive(Clone)]
pub struct PyAnt {
    pose: PyPose,
    team: u8,
    hp: f32,
    velocity: f32,
}

#[pymethods]
impl PyAnt {
    #[new]
    fn py_new(
        x: Option<f32>,
        y: Option<f32>,
        rotation: Option<f32>,
        team: Option<common::Team>,
        hp: Option<f32>,
        velocity: Option<f32>,
    ) -> Self {
        PyAnt {
            pose: PyPose::py_new(x, y, rotation),
            team: team.unwrap_or(0),
            hp: hp.unwrap_or(1.),
            velocity: velocity.unwrap_or(0.),
        }
    }

    #[getter]
    fn get_team(&self) -> PyResult<common::Team> {
        Ok(self.team)
    }

    #[getter]
    fn get_hp(&self) -> PyResult<f32> {
        Ok(self.hp)
    }

    #[getter]
    fn get_velocity(&self) -> PyResult<f32> {
        Ok(self.velocity)
    }

    #[getter]
    fn get_pose(&self) -> PyResult<PyPose> {
        Ok(self.pose.clone())
    }

    #[setter]
    fn set_team(&mut self, team: common::Team) -> PyResult<()> {
        self.team = team;
        Ok(())
    }

    #[setter]
    fn set_hp(&mut self, hp: f32) -> PyResult<()> {
        self.hp = hp;
        Ok(())
    }

    #[setter]
    fn set_velocity(&mut self, velocity: f32) -> PyResult<()> {
        self.velocity = velocity;
        Ok(())
    }

    #[setter]
    fn set_pose(&mut self, pose: PyPose) -> PyResult<()> {
        unsafe {
            Arc::get_mut_unchecked(&mut self.pose.inner).x = pose.inner.x;
            Arc::get_mut_unchecked(&mut self.pose.inner).y = pose.inner.y;
            Arc::get_mut_unchecked(&mut self.pose.inner).rotation = pose.inner.rotation;
        }
        Ok(())
    }
}

impl From<common::Ant> for PyAnt {
    fn from(hill: common::Ant) -> Self {
        let common::Ant {
            pose,
            team,
            hp,
            velocity,
        } = hill;
        PyAnt {
            pose: PyPose::from(pose),
            team,
            hp,
            velocity,
        }
    }
}

impl From<&PyAnt> for common::Ant {
    fn from(py_ant: &PyAnt) -> Self {
        common::Ant {
            pose: common::Pose::from(&py_ant.pose),
            team: py_ant.team,
            hp: py_ant.hp,
            velocity: py_ant.velocity,
        }
    }
}

#[pyproto]
impl PyObjectProtocol for PyAnt {
    fn __repr__(&self) -> PyResult<String> {
        Ok(format!("{:?}", common::Ant::from(self)))
    }
}

#[pyclass(name = "AntHill", subclass)]
#[derive(Clone)]
pub struct PyAntHill {
    pose: PyPose,
    team: u8,
}

#[pymethods]
impl PyAntHill {
    #[new]
    fn py_new(
        x: Option<f32>,
        y: Option<f32>,
        rotation: Option<f32>,
        team: Option<common::Team>,
    ) -> Self {
        PyAntHill {
            pose: PyPose::py_new(x, y, rotation),
            team: team.unwrap_or(0),
        }
    }

    #[getter]
    fn get_team(&self) -> PyResult<common::Team> {
        Ok(self.team)
    }

    #[getter]
    fn get_pose(&self) -> PyResult<PyPose> {
        Ok(self.pose.clone())
    }

    #[setter]
    fn set_team(&mut self, team: common::Team) -> PyResult<()> {
        self.team = team;
        Ok(())
    }

    #[setter]
    fn set_pose(&mut self, pose: PyPose) -> PyResult<()> {
        unsafe {
            Arc::get_mut_unchecked(&mut self.pose.inner).x = pose.inner.x;
            Arc::get_mut_unchecked(&mut self.pose.inner).y = pose.inner.y;
            Arc::get_mut_unchecked(&mut self.pose.inner).rotation = pose.inner.rotation;
        }
        Ok(())
    }
}

impl From<common::AntHill> for PyAntHill {
    fn from(hill: common::AntHill) -> Self {
        let common::AntHill { pose, team } = hill;
        PyAntHill {
            pose: PyPose::from(pose),
            team,
        }
    }
}

impl From<&PyAntHill> for common::AntHill {
    fn from(py_hill: &PyAntHill) -> Self {
        common::AntHill {
            pose: common::Pose::from(&py_hill.pose),
            team: py_hill.team,
        }
    }
}

#[pyproto]
impl PyObjectProtocol for PyAntHill {
    fn __repr__(&self) -> PyResult<String> {
        Ok(format!("{:?}", common::AntHill::from(self)))
    }
}

#[pyclass(name = "SmellCloud", subclass)]
#[derive(Clone)]
pub struct PySmellCloud {
    position: PyVector2,
    code: u32,
    age: u32,
    team: u8,
}

#[pymethods]
impl PySmellCloud {
    #[new]
    fn py_new(
        x: Option<f32>,
        y: Option<f32>,
        team: Option<common::Team>,
        age: Option<u32>,
        code: Option<u32>,
    ) -> Self {
        Self {
            position: PyVector2::py_new(x, y),
            team: team.unwrap_or(0),
            age: age.unwrap_or(100),
            code: code.unwrap_or(0),
        }
    }

    #[getter]
    fn get_team(&self) -> PyResult<common::Team> {
        Ok(self.team)
    }

    #[getter]
    fn get_position(&self) -> PyResult<PyVector2> {
        Ok(self.position.clone())
    }

    #[setter]
    fn set_team(&mut self, team: common::Team) -> PyResult<()> {
        self.team = team;
        Ok(())
    }

    #[setter]
    fn set_position(&mut self, position: PyVector2) -> PyResult<()> {
        unsafe {
            Arc::get_mut_unchecked(&mut self.position.inner).x = position.inner.x;
            Arc::get_mut_unchecked(&mut self.position.inner).y = position.inner.y;
        }
        Ok(())
    }
}

impl From<common::SmellCloud> for PySmellCloud {
    fn from(hill: common::SmellCloud) -> Self {
        let common::SmellCloud {
            position,
            team,
            age,
            code,
        } = hill;
        Self {
            position: PyVector2::from(position),
            team,
            age,
            code,
        }
    }
}

impl From<&PySmellCloud> for common::SmellCloud {
    fn from(py_cloud: &PySmellCloud) -> Self {
        Self {
            position: common::Vector2::from(&py_cloud.position),
            team: py_cloud.team,
            age: py_cloud.age,
            code: py_cloud.code,
        }
    }
}

#[pyproto]
impl PyObjectProtocol for PySmellCloud {
    fn __repr__(&self) -> PyResult<String> {
        Ok(format!("{:?}", common::SmellCloud::from(self)))
    }
}

#[pyclass(name = "Frame", subclass)]
#[derive(Clone)]
pub struct PyFrame {
    ants: Vec<PyAnt>,
    anthills: Vec<PyAntHill>,
    raspberries: Vec<PyPose>,
    sugar_hills: Vec<PySugarHill>,
    smells_clouds: Vec<PySmellCloud>,
}

impl From<&PyFrame> for common::Frame {
    fn from(py_frame: &PyFrame) -> Self {
        common::Frame {
            ants: py_frame.ants.iter().map(common::Ant::from).collect(),
            anthills: py_frame
                .anthills
                .iter()
                .map(common::AntHill::from)
                .collect(),
            raspberries: py_frame
                .raspberries
                .iter()
                .map(common::Pose::from)
                .collect(),
            sugar_hills: py_frame
                .sugar_hills
                .iter()
                .map(common::SugarHill::from)
                .collect(),
            smells_clouds: py_frame
                .smells_clouds
                .iter()
                .map(common::SmellCloud::from)
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
            smells_clouds,
        } = frame;
        PyFrame {
            ants: ants.into_iter().map(PyAnt::from).collect(),
            anthills: anthills.into_iter().map(PyAntHill::from).collect(),
            raspberries: raspberries.into_iter().map(PyPose::from).collect(),
            sugar_hills: sugar_hills.into_iter().map(PySugarHill::from).collect(),
            smells_clouds: smells_clouds.into_iter().map(PySmellCloud::from).collect(),
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
            smells_clouds: vec![],
        }
    }

    #[getter]
    fn get_ants(&self) -> PyResult<Vec<PyAnt>> {
        Ok(self.ants.clone())
    }

    #[getter]
    fn get_anthills(&self) -> PyResult<Vec<PyAntHill>> {
        Ok(self.anthills.clone())
    }

    #[getter]
    fn get_raspberries(&self) -> PyResult<Vec<PyPose>> {
        Ok(self.raspberries.clone())
    }

    #[getter]
    fn get_sugar_hills(&self) -> PyResult<Vec<PySugarHill>> {
        Ok(self.sugar_hills.clone())
    }

    fn add_ant(&mut self, ant: PyAnt) -> PyResult<()> {
        self.ants.push(ant);
        Ok(())
    }

    fn add_anthill(&mut self, hill: PyAntHill) -> PyResult<()> {
        self.anthills.push(hill);
        Ok(())
    }

    fn add_raspberry(&mut self, pose: PyPose) -> PyResult<()> {
        self.raspberries.push(pose);
        Ok(())
    }

    fn add_sugar_hill(&mut self, hill: PySugarHill) -> PyResult<()> {
        self.sugar_hills.push(hill);
        Ok(())
    }
}

#[pyclass(name = "Recording", subclass)]
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
                inner: Arc::new(common::Map {
                    width: 128.,
                    height: 128.,
                }),
            },
            frames: vec![],
        }
    }

    // props
    #[getter]
    fn get_map(&self) -> PyResult<PyMap> {
        Ok(PyMap {
            inner: self.map.inner.clone(),
        })
    }

    #[setter]
    fn set_map(&mut self, map: PyMap) -> PyResult<()> {
        self.map = map;
        Ok(())
    }

    #[getter]
    fn frames(&self) -> PyResult<Vec<PyFrame>> {
        Ok(self.frames.iter().cloned().collect())
    }

    #[pyo3(text_signature = "(self, frame, /)")]
    fn add_frame(&mut self, frame: PyFrame) -> PyResult<()> {
        self.frames.push(frame);
        Ok(())
    }

    // io
    #[staticmethod]
    #[pyo3(text_signature = "(filename, /)")]
    fn load(filename: &str) -> PyResult<PyRecording> {
        let file = File::open(filename)
            .map_err(|msg| PyErr::new::<PyFileNotFoundError, _>(msg.to_string()))?;
        let reader = BufReader::new(file);
        let common::Recording { map, frames } =
            common::Recording::load(reader).map_err(PyErr::new::<PyTypeError, _>)?;

        Ok(PyRecording {
            map: PyMap {
                inner: Arc::new(map),
            },
            frames: frames.into_iter().map(PyFrame::from).collect(),
        })
    }

    #[pyo3(text_signature = "(self, filename, /)")]
    fn dump(&self, filename: &str) -> PyResult<()> {
        let file = File::create(filename)
            .map_err(|msg| PyErr::new::<PyFileNotFoundError, _>(msg.to_string()))?;
        let writer = BufWriter::new(file);
        let recording = common::Recording {
            map: (*self.map.inner).clone(),
            frames: self
                .frames
                .iter()
                .map(|frame| common::Frame::from(frame))
                .collect(),
        };
        recording
            .dump(writer)
            .map_err(PyErr::new::<PyTypeError, _>)?;
        Ok(())
    }
}

#[pymodule]
fn antbinding(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_class::<PyPose>()?;
    m.add_class::<PyVector2>()?;
    m.add_class::<PyFrame>()?;
    m.add_class::<PyMap>()?;
    m.add_class::<PyAnt>()?;
    m.add_class::<PyAntHill>()?;
    m.add_class::<PySugarHill>()?;
    m.add_class::<PySmellCloud>()?;
    m.add_class::<PyRecording>()?;
    m.add_class::<world::PyWorld>()?;
    Ok(())
}
