use wasm_bindgen::JsCast;
use web_sys::console::*;
use web_sys::WebGlRenderingContext as GL;
use web_sys::{HtmlCanvasElement, HtmlElement};
use yew::format::{Binary, Nothing};
use yew::prelude::*;
use yew::services::fetch::{FetchService, Request, Response};
use yew::services::resize::{ResizeService, ResizeTask};
use yew::services::{RenderService, Task};
use yew::{html, Component, ComponentLink, Html, NodeRef, ShouldRender};

use common;

use nalgebra::Vector3;

use crate::background::Background;
use crate::camera::Camera;
use crate::ground::Ground;

use crate::mesh::{SimpleMesh, Transformation};

#[inline]
fn log(text: &str) {
    log_1(&text.into());
}

#[derive(Debug)]
pub struct Vector2 {
    pub x: i32,
    pub y: i32,
}

impl Vector2 {
    pub fn from_event(event: &MouseEvent) -> Vector2 {
        Vector2 {
            x: event.screen_x(),
            y: event.screen_y(),
        }
    }
}
#[derive(Debug)]
pub enum MouseButton {
    Left,
    Right,
    Middle,
    Invalid,
}

impl From<i16> for MouseButton {
    fn from(orig: i16) -> MouseButton {
        match orig {
            0 => MouseButton::Left,
            1 => MouseButton::Middle,
            2 => MouseButton::Right,
            _ => MouseButton::Invalid,
        }
    }
}

#[derive(Debug)]
pub enum FetchErrorReason {
    Deserialize,
    NotOk,
    NotSuccess,
}

#[derive(Debug)]
pub struct FetchError {
    pub reason: FetchErrorReason,
    pub uri: &'static str,
}

#[derive(Debug)]
pub enum Msg {
    Render(f64),
    MouseDown(Vector2, MouseButton),
    MouseUp,
    MouseLeave,
    MouseMove(Vector2),
    Zoom(f64),
    Resize,
    FailedFetch(FetchError),
    FetchResponse(common::Recording),
}

impl Msg {
    pub fn mouse_up(event: MouseEvent) -> Msg {
        Msg::MouseDown(
            Vector2::from_event(&event),
            MouseButton::from(event.button()),
        )
    }
    pub fn mouse_move(event: MouseEvent) -> Msg {
        Msg::MouseMove(Vector2::from_event(&event))
    }
}

pub struct MouseAction {
    last_pos: Option<Vector2>,
    button: MouseButton,
}

pub struct Models {
    ant: Option<SimpleMesh>,
    raspberry: Option<SimpleMesh>,
    anthill: Option<SimpleMesh>,
    sugar_hill: Option<SimpleMesh>,
}

impl Models {
    pub fn empty() -> Self {
        Self {
            ant: None,
            raspberry: None,
            anthill: None,
            sugar_hill: None,
        }
    }
}

pub struct Scene {
    canvas: Option<HtmlCanvasElement>,
    container: Option<HtmlElement>,
    gl: Option<GL>,
    link: ComponentLink<Self>,
    canvas_ref: NodeRef,
    container_ref: NodeRef,
    render_loop: Option<Box<dyn Task>>,
    resize_service: Option<Box<ResizeTask>>,
    models: Models,
    recording: Option<common::Recording>,
    ground: Option<Ground>,
    camera: Camera,
    background: Option<Background>,
    mouse_action: MouseAction,
    fetch_task: Option<Box<dyn Task>>,
}

impl Scene {
    fn fetch_recording(&mut self) {
        let uri = "/recordings/second.bin";
        let request = Request::builder()
            .method("GET")
            .uri(uri)
            .body(Nothing)
            .unwrap();

        let task = FetchService::fetch_binary(
            request,
            self.link.callback(move |response: Response<Binary>| {
                let (meta, data) = response.into_parts();
                if meta.status.is_success() {
                    if let Ok(data) = data {
                        match common::Recording::load(&data[..]) {
                            Ok(data) => Msg::FetchResponse(data),
                            _ => Msg::FailedFetch(FetchError {
                                reason: FetchErrorReason::Deserialize,
                                uri,
                            }),
                        }
                    } else {
                        Msg::FailedFetch(FetchError {
                            reason: FetchErrorReason::NotOk,
                            uri,
                        })
                    }
                } else {
                    Msg::FailedFetch(FetchError {
                        reason: FetchErrorReason::NotSuccess,
                        uri,
                    })
                }
            }),
        );
        match task {
            Ok(task) => self.fetch_task = Some(Box::new(task)),
            Err(err) => log(&format!("Error fetching {} {}", uri, err)),
        }
    }
}

impl Component for Scene {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Scene {
            canvas: None,
            container: None,
            gl: None,
            link,
            canvas_ref: NodeRef::default(),
            container_ref: NodeRef::default(),
            render_loop: None,
            resize_service: None,
            models: Models::empty(),
            ground: None,
            camera: Camera::new(),
            background: None,
            mouse_action: MouseAction {
                last_pos: None,
                button: MouseButton::Left,
            },
            recording: None,
            // fetch_service: FetchService::new(),
            fetch_task: None,
        }
    }

    fn rendered(&mut self, first_render: bool) {
        // Once rendered, store references for the canvas and GL context. These can be used for
        // resizing the rendering area when the window or canvas element are resized, as well as
        // for making GL calls.

        let canvas = self.canvas_ref.cast::<HtmlCanvasElement>().unwrap();
        let container = self.container_ref.cast::<HtmlElement>().unwrap();
        self.container = Some(container);

        let gl: GL = canvas
            .get_context("webgl")
            .unwrap()
            .unwrap()
            .dyn_into()
            .unwrap();
        gl.enable(GL::DEPTH_TEST);

        self.canvas = Some(canvas);
        self.fetch_recording();

        self.models.ant = Some(SimpleMesh::mesh(
            &gl,
            "Ant.Released",
            "./assets/ant-texture.png",
        ));
        self.models.raspberry = Some(SimpleMesh::mesh(
            &gl,
            "raspberry",
            "./assets/raspberry_paint.png",
        ));
        self.models.anthill = Some(SimpleMesh::mesh(
            &gl,
            "anthill",
            "./assets/anthill_paint.png",
        ));
        self.models.sugar_hill = Some(SimpleMesh::mesh(
            &gl,
            "sugar_hill",
            "./assets/sugar_paint.png",
        ));
        self.ground = Some(Ground::new(&gl, 128., 128.));
        self.background = Some(Background::new(&gl));
        self.gl = Some(gl);

        // In a more complex use-case, there will be additional WebGL initialization that should be
        // done here, such as enabling or disabling depth testing, depth functions, face
        // culling etc.

        if first_render {
            let gl = self.gl.as_ref().unwrap();
            log_1(
                &format!(
                    "rendering buffer: {}x{}",
                    gl.drawing_buffer_width(),
                    gl.drawing_buffer_height()
                )
                .into(),
            );
            // The callback to request animation frame is passed a time value which can be used for
            // rendering motion independent of the framerate which may vary.
            let render_frame = self.link.callback(Msg::Render);
            let handle = RenderService::request_animation_frame(render_frame);
            let resize_service = ResizeService::register(self.link.callback(|_| Msg::Resize));

            // A reference to the handle must be stored, otherwise it is dropped and the render won't
            // occur.
            self.render_loop = Some(Box::new(handle));
            self.resize_service = Some(Box::new(resize_service));
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Render(timestamp) => {
                // Render functions are likely to get quite large, so it is good practice to split
                // it into it's own function rather than keeping it inline in the update match
                // case. This also allows for updating other UI elements that may be rendered in
                // the DOM like a framerate counter, or other overlaid textual elements.
                self.render_gl(timestamp);
            }
            Msg::MouseMove(new_pos) => {
                if let Some(last_pos) = &self.mouse_action.last_pos {
                    let delta_x = new_pos.x - last_pos.x;
                    let delta_y = new_pos.y - last_pos.y;
                    match self.mouse_action.button {
                        MouseButton::Left => {
                            self.camera.orbit_left_right(-delta_x as f32 / 100.0);
                            self.camera.orbit_up_down(delta_y as f32 / 100.0);
                        }
                        MouseButton::Middle => {
                            self.camera.move_left_right(-delta_x as f32 / 600.0);
                            self.camera.move_up_down(delta_y as f32 / 600.0);
                        }
                        _ => (),
                    }
                    self.mouse_action.last_pos = Some(new_pos);
                }
            }
            Msg::MouseDown(pos, button) => {
                self.mouse_action.last_pos = Some(pos);
                self.mouse_action.button = button;
            }
            Msg::MouseUp | Msg::MouseLeave => {
                self.mouse_action.last_pos = None;
            }
            Msg::Zoom(amount) => {
                // Chrome or firefox?
                if amount.abs() >= 50.0 {
                    // log(&format!("chrome zoom: {}", amount));
                    self.camera.zoom(amount as f32 / 53.);
                } else {
                    // log(&format!("firefox zoom: {}", amount));
                    self.camera.zoom(amount as f32 / 3.);
                }
            }
            Msg::Resize => (),
            Msg::FailedFetch(error) => log(&format!(
                "Fetching {} failed: {:?}",
                error.uri, error.reason
            )),
            Msg::FetchResponse(recording) => {
                let gl = self.gl.as_ref().expect("GL Context not initialized!");
                self.ground = Some(Ground::new(&gl, recording.map.width, recording.map.height));
                self.recording = Some(recording)
            }
        }
        false
    }

    fn view(&self) -> Html {
        html! {
            <div class="scene" ref={self.container_ref.clone()}>
            <canvas
                onmousedown=self.link.callback(Msg::mouse_up)
                onmouseup=self.link.callback(|_| Msg::MouseUp)
                onmousemove=self.link.callback(Msg::mouse_move)
                onmouseleave=self.link.callback(|_|Msg::MouseLeave)
                onwheel=self.link.callback(|e: WheelEvent| Msg::Zoom(e.delta_y()))
                ref={self.canvas_ref.clone()} />
            </div>
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }
}

impl Scene {
    fn resize(&mut self) {
        let container = self.container.as_ref().unwrap();
        let canvas = self.canvas.as_ref().unwrap();
        let c_width = container.client_width() as u32;
        let c_height = container.client_height() as u32;
        let b_width = canvas.width();
        let b_height = canvas.height();
        if c_width != b_width || c_height != b_height {
            canvas.set_width(c_width);
            canvas.set_height(c_height);

            let gl = self.gl.as_ref().unwrap();
            gl.viewport(0, 0, c_width as i32, c_height as i32);
        }
    }

    fn render_gl(&mut self, timestamp: f64) {
        self.resize();
        let gl = self.gl.as_ref().expect("GL Context not initialized!");
        gl.clear(GL::COLOR_BUFFER_BIT);
        let canvas = self.canvas.as_ref().unwrap();
        let c_width = canvas.client_width();
        let c_height = canvas.client_height();
        let ratio = c_width as f32 / c_height as f32;
        // log_1(&format!("aspect ratio: {}", ratio).into());
        self.camera.update_aspect(ratio);
        gl.viewport(0, 0, c_width, c_height);

        gl.disable(GL::DEPTH_TEST);
        if let Some(background) = &self.background {
            background.render(&gl, timestamp);
        }
        gl.enable(GL::DEPTH_TEST);

        if let Some(recording) = &self.recording {
            if let Some(ant) = &self.models.ant {
                for inst in recording.frames[0].ants.iter() {
                    let rotation = Vector3::new(0.0f32, 0.0f32, inst.pose.rotation);
                    let translation = Vector3::new(inst.pose.x, inst.pose.y, 0.8f32);
                    ant.render(
                        &gl,
                        &self.camera,
                        &Transformation {
                            rotation,
                            translation,
                            scale: 0.5,
                        },
                    );
                }
            }
            if let Some(raspberry) = &self.models.raspberry {
                for inst in recording.frames[0].raspberries.iter() {
                    let rotation = Vector3::new(0.0f32, 0.0f32, inst.rotation);
                    let translation = Vector3::new(inst.x, inst.y, 0.8f32);
                    raspberry.render(
                        &gl,
                        &self.camera,
                        &Transformation {
                            rotation,
                            translation,
                            scale: 10.0,
                        },
                    );
                }
            }

            if let Some(anthill) = &self.models.anthill {
                for inst in recording.frames[0].anthills.iter() {
                    let rotation = Vector3::new(0.0f32, 0.0f32, inst.pose.rotation);
                    let translation = Vector3::new(inst.pose.x, inst.pose.y, 0.);
                    anthill.render(
                        &gl,
                        &self.camera,
                        &Transformation {
                            rotation,
                            translation,
                            scale: 5.0,
                        },
                    );
                }
            }

            if let Some(sugar_hill) = &self.models.sugar_hill {
                for inst in recording.frames[0].sugar_hills.iter() {
                    let rotation = Vector3::new(0.0f32, 0.0f32, inst.pose.rotation);
                    let translation = Vector3::new(inst.pose.x, inst.pose.y, 0.);
                    sugar_hill.render(
                        &gl,
                        &self.camera,
                        &Transformation {
                            rotation,
                            translation,
                            scale: 10.0,
                        },
                    );
                }
            }
        }
        if let Some(ground) = &self.ground {
            ground.render(&gl, &self.camera);
        }

        gl.clear_color(0., 0.0, 0.0, 1.0);

        let render_frame = self.link.callback(Msg::Render);
        let handle = RenderService::request_animation_frame(render_frame);

        // A reference to the new handle must be retained for the next render to run.
        self.render_loop = Some(Box::new(handle));
    }
}
