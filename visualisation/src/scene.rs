use wasm_bindgen::JsCast;
use web_sys::console::*;
use web_sys::WebGlRenderingContext as GL;
use web_sys::{HtmlCanvasElement, HtmlElement};
use yew::prelude::*;
use yew::services::resize::{ResizeService, ResizeTask};
use yew::services::{RenderService, Task};
use yew::{html, Component, ComponentLink, Html, NodeRef, ShouldRender};

use nalgebra::Vector3;

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
pub enum Msg {
    Render(f64),
    MouseDown(Vector2, MouseButton),
    MouseUp,
    MouseLeave,
    MouseMove(Vector2),
    Zoom(f64),
    Resize,
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

pub struct Scene {
    canvas: Option<HtmlCanvasElement>,
    container: Option<HtmlElement>,
    gl: Option<GL>,
    link: ComponentLink<Self>,
    canvas_ref: NodeRef,
    container_ref: NodeRef,
    render_loop: Option<Box<dyn Task>>,
    resize_service: Option<Box<ResizeTask>>,
    cube: Option<SimpleMesh>,
    ant: Option<SimpleMesh>,
    raspberry: Option<SimpleMesh>,
    anthill: Option<SimpleMesh>,
    sugar_hill: Option<SimpleMesh>,
    ground: Option<Ground>,
    camera: Camera,
    mouse_action: MouseAction,
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
            cube: None,
            ant: None,
            raspberry: None,
            anthill: None,
            sugar_hill: None,
            ground: None,
            camera: Camera::new(),
            mouse_action: MouseAction {
                last_pos: None,
                button: MouseButton::Left,
            },
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

        self.cube = Some(SimpleMesh::cube(&gl));
        self.ant = Some(SimpleMesh::mesh(&gl, "Ant.Released", "./ant-texture.png"));
        self.raspberry = Some(SimpleMesh::mesh(&gl, "raspberry", "./raspberry_paint.png"));
        self.anthill = Some(SimpleMesh::mesh(&gl, "anthill", "./anthill_paint.png"));
        self.sugar_hill = Some(SimpleMesh::mesh(&gl, "sugar_hill", "./sugar_paint.png"));
        self.ground = Some(Ground::new(&gl));
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
            let handle = RenderService::new().request_animation_frame(render_frame);
            let resize_service = ResizeService::new().register(self.link.callback(|_| Msg::Resize));

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
                onmousewheel=self.link.callback(|e: WheelEvent| Msg::Zoom(e.delta_y()))
                ref={self.canvas_ref.clone()} />
            </div>
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }
}

struct AntInfo {
    pos_x: f32,
    pos_y: f32,
    rot: f32,
}

fn render_background(gl: &GL, timestamp: f64) {
    let vert_code = include_str!("./basic.vert");
    let frag_code = include_str!("./basic.frag");

    // This list of vertices will draw two triangles to cover the entire canvas.
    let vertices: Vec<f32> = vec![
        -1.0, -1.0, 1.0, -1.0, -1.0, 1.0, -1.0, 1.0, 1.0, -1.0, 1.0, 1.0,
    ];
    let vertex_buffer = gl.create_buffer().unwrap();
    let verts = js_sys::Float32Array::from(vertices.as_slice());

    gl.bind_buffer(GL::ARRAY_BUFFER, Some(&vertex_buffer));
    gl.buffer_data_with_array_buffer_view(GL::ARRAY_BUFFER, &verts, GL::STATIC_DRAW);

    let vert_shader = gl.create_shader(GL::VERTEX_SHADER).unwrap();
    gl.shader_source(&vert_shader, &vert_code);
    gl.compile_shader(&vert_shader);

    let frag_shader = gl.create_shader(GL::FRAGMENT_SHADER).unwrap();
    gl.shader_source(&frag_shader, &frag_code);
    gl.compile_shader(&frag_shader);

    let shader_program = gl.create_program().unwrap();
    gl.attach_shader(&shader_program, &vert_shader);
    gl.attach_shader(&shader_program, &frag_shader);
    gl.link_program(&shader_program);

    gl.use_program(Some(&shader_program));

    // Attach the position vector as an attribute for the GL context.
    let position = gl.get_attrib_location(&shader_program, "a_position") as u32;
    gl.vertex_attrib_pointer_with_i32(position, 2, GL::FLOAT, false, 0, 0);
    gl.enable_vertex_attrib_array(position);

    // Attach the time as a uniform for the GL context.
    let time = gl.get_uniform_location(&shader_program, "u_time");
    gl.uniform1f(time.as_ref(), timestamp as f32);

    gl.draw_arrays(GL::TRIANGLES, 0, 6);
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
            // log_1(
            //     &format!(
            //         "client_height: {}x{} -> {}x{}",
            //         b_width, b_height, c_width, c_height
            //     )
            //     .into(),
            // );
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
        render_background(gl, timestamp);
        gl.enable(GL::DEPTH_TEST);

        let ants = vec![
            AntInfo {
                pos_x: 12.,
                pos_y: 4.,
                rot: 1.2,
            },
            AntInfo {
                pos_x: 0.,
                pos_y: 0.,
                rot: 0.,
            },
        ];

        let raspberries = vec![
            AntInfo {
                pos_x: 3.,
                pos_y: -3.,
                rot: 1.2,
            },
            AntInfo {
                pos_x: 4.,
                pos_y: 0.,
                rot: 2.,
            },
        ];

        let anthills = vec![
            AntInfo {
                pos_x: -2.,
                pos_y: 30.,
                rot: 1.2,
            },
            AntInfo {
                pos_x: -20.,
                pos_y: -30.,
                rot: 2.7,
            },
        ];

        let sugar_hills = vec![
            AntInfo {
                pos_x: -12.,
                pos_y: 2.3,
                rot: 1.2,
            },
            AntInfo {
                pos_x: 21.,
                pos_y: 3.,
                rot: 2.7,
            },
        ];

        if let Some(ant) = &self.ant {
            for inst in ants.iter() {
                let rotation = Vector3::new(0.0f32, 0.0f32, inst.rot);
                let translation = Vector3::new(inst.pos_x, inst.pos_y, 0.8f32);
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
        if let Some(raspberry) = &self.raspberry {
            for inst in raspberries.iter() {
                let rotation = Vector3::new(0.0f32, 0.0f32, inst.rot);
                let translation = Vector3::new(inst.pos_x, inst.pos_y, 0.8f32);
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

        if let Some(anthill) = &self.anthill {
            for inst in anthills.iter() {
                let rotation = Vector3::new(0.0f32, 0.0f32, inst.rot);
                let translation = Vector3::new(inst.pos_x, inst.pos_y, 0.);
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

        if let Some(sugar_hill) = &self.sugar_hill {
            for inst in sugar_hills.iter() {
                let rotation = Vector3::new(0.0f32, 0.0f32, inst.rot);
                let translation = Vector3::new(inst.pos_x, inst.pos_y, 0.);
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

        if let Some(ground) = &self.ground {
            ground.render(&gl, &self.camera);
        }

        gl.clear_color(0., 0.0, 0.0, 1.0);

        let render_frame = self.link.callback(Msg::Render);
        let handle = RenderService::new().request_animation_frame(render_frame);

        // A reference to the new handle must be retained for the next render to run.
        self.render_loop = Some(Box::new(handle));
    }
}
