#![allow(unreachable_code, unused_variables, dead_code)]

use nannou::prelude::*;
//use nannou::wgpu::{TextureFormat, Device, Color};
use nannou::image::{DynamicImage, RgbImage, Rgb};
use num::Complex;
use std::borrow::BorrowMut;

static W: u32 = 512;
static H: u32 = 512;

struct Model {
    texture: wgpu::Texture,
    mouse_pos: Point2,
    pt: Point2,
    scale: Point2<f64>,
}

fn main() {
    nannou::app(model).run();
}

fn escape_time(c: Complex<f64>, limit: u32) -> Option<u32> {
    let mut z = Complex { re: 0.0, im: 0.0 };
    for i in 0..limit {
        z = z * z + c;
        if z.norm_sqr() > 4.0 {
            return Some(i);
        }
    }

    None
}

fn gen_tex(app: &App, px: f64, py: f64, sw: f64, sh: f64) -> wgpu::Texture {

    let mut img = RgbImage::new(W, H);

    for x in 0..W {
        for y in 0..H {
            let cx = x as f64 / W as f64 - 0.5;
            let cy = y as f64 / H as f64 - 0.5;
            let c = Complex::new(px + sw * cx, py + sh * cy);

            let p = match escape_time(c, 255) {
                None => 0,
                Some(count) => 255 - count as u8,
            };

            img.put_pixel(x, y, Rgb([p, 0, 0]));
        }
    }

    let texture = wgpu::Texture::from_image(app, &DynamicImage::ImageRgb8(img));

    texture
}

fn model(app: &App) -> Model {

    // Create a new window!
    app.new_window()
        .size(W, H)
        .view(view)
        .event(event)
        .build()
        .unwrap();

    let texture = gen_tex(app,-0.0, -0.0, 4.0, 4.0);

    Model { texture, mouse_pos: Point2::zero(), pt: Point2::zero(), scale: Point2{x:4.0,y:4.0} }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let tex = &model.texture;

    let draw = app.draw();
    draw.background().color(BLACK);

    draw.texture(&model.texture);

    draw.to_frame(app, &frame).unwrap();
}

fn event(app: &App, model: &mut Model, event: WindowEvent) {
    match event {
        KeyPressed(_key) => {}
        KeyReleased(_key) => {}

        MouseMoved(pos) => {
            println!("mouse pos: {:?}", pos);
            model.mouse_pos = pos;
        }

        MousePressed(_button) => {
            println!("mouse pressed");
            model.pt = model.mouse_pos;
            model.scale = model.scale * 0.95;

            let x0 = model.pt.x as f64 / W as f64 * 2.0;
            let y0 = model.pt.y as f64 / H as f64 * 2.0;

            model.texture.borrow_mut().set()

            model.texture = gen_tex(app, x0, y0, model.scale.x, model.scale.y);
        }

        MouseReleased(_button) => {
            println!("mouse released");
        }

        MouseEntered => {}
        MouseExited => {}
        MouseWheel(_amount, _phase) => {}
        Moved(_pos) => {}
        Resized(_size) => {}
        Touch(_touch) => {}
        TouchPressure(_pressure) => {}
        HoveredFile(_path) => {}
        DroppedFile(_path) => {}
        HoveredFileCancelled => {}
        Focused => {}
        Unfocused => {}
        Closed => {}
    }
}

