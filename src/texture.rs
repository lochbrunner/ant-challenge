use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::console;
use web_sys::HtmlImageElement;
use web_sys::WebGlRenderingContext as GL;
use web_sys::WebGlTexture;

use std::cell::RefCell;
use std::rc::Rc;

pub struct Texture {
    pub texture: Rc<WebGlTexture>,
}

impl Texture {
    pub fn new(gl: &GL, src: &str) -> Result<Texture, JsValue> {
        let texture = gl.create_texture().expect("Cannot create gl texture");
        gl.bind_texture(GL::TEXTURE_2D, Some(&texture));
        let level = 0;
        let internal_format = GL::RGBA;
        let width = 1;
        let height = 1;
        let border = 0;
        let src_format = GL::RGBA;
        let src_type = GL::UNSIGNED_BYTE;
        // Now upload single pixel.
        let pixel: [u8; 4] = [0, 0, 255, 255];
        gl.tex_image_2d_with_i32_and_i32_and_i32_and_format_and_type_and_opt_u8_array(
            GL::TEXTURE_2D,
            level,
            internal_format as i32,
            width,
            height,
            border,
            src_format,
            src_type,
            Some(&pixel),
        )?;
        let img = HtmlImageElement::new().unwrap();
        img.set_cross_origin(Some(""));

        let imgrc = Rc::new(img);

        let texture = Rc::new(texture);

        {
            let img = imgrc.clone();
            let texture = texture.clone();
            let gl = Rc::new(gl.clone());
            let a = Closure::wrap(Box::new(move || {
                gl.bind_texture(GL::TEXTURE_2D, Some(&texture));

                if let Err(e) = gl.tex_image_2d_with_u32_and_u32_and_image(
                    GL::TEXTURE_2D,
                    level,
                    internal_format as i32,
                    src_format,
                    src_type,
                    &img,
                ) {
                    // TODO better error handling...
                    console::log_1(&e);
                    return;
                }

                // different from webgl1 where we need the pic to be power of 2
                gl.generate_mipmap(GL::TEXTURE_2D);
            }) as Box<dyn FnMut()>);
            imgrc.set_onload(Some(a.as_ref().unchecked_ref()));

            // Normally we'd store the handle to later get dropped at an appropriate
            // time but for now we want it to be a global handler so we use the
            // forget method to drop it without invalidating the closure. Note that
            // this is leaking memory in Rust, so this should be done judiciously!
            a.forget();
        }

        imgrc.set_src(src);
        Ok(Texture { texture })
    }
}
