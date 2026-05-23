use macroquad::prelude::*;
use macroquad::ui::{hash, root_ui, widgets};

use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

struct Camera {
    x: f32,
    y: f32,
    z: f32,
    yaw: f32,
    pitch: f32,
    fov: f32,
}

fn draw_point(pos: Vec2) -> () {
    draw_circle(pos.x, pos.y, 5.0, DARKGREEN);
}

fn draw_edge(p1: Vec2, p2: Vec2, camera: &Camera, end1: Vec3, end2: Vec3) -> () {
    let distance1: f32 =
        ((camera.x - end1.x).powi(2) + (camera.y - end1.y).powi(2) + (camera.z - end1.z).powi(2))
            .sqrt();
    let distance2: f32 =
        ((camera.x - end2.x).powi(2) + (camera.y - end2.y).powi(2) + (camera.z - end2.z).powi(2))
            .sqrt();
    let mean_distance = (distance1 + distance2) / 2.0;
    draw_line(
        p1.x,
        p1.y,
        p2.x,
        p2.y,
        (10.0 / mean_distance).clamp(1.5, 3.0),
        GREEN,
    );
}

fn screen(point: Vec2) -> Vec2 {
    let aspect = screen_width() / screen_height();
    Vec2::new(
        (point.x / aspect + 1.0) / 2.0 * screen_width(),
        (1.0 - (point.y + 1.0) / 2.0) * screen_height(),
    )
}

fn project(vertex: Vec3, fov: f32) -> Vec2 {
    Vec2::new((vertex.x / vertex.z) * fov, (vertex.y / vertex.z) * fov)
}

fn rotate_xz(vertex: Vec3, angle: f32) -> Vec3 {
    let sin = angle.sin();
    let cos = angle.cos();
    Vec3::new(
        vertex.x * cos - vertex.z * sin,
        vertex.y,
        vertex.x * sin + vertex.z * cos,
    )
}

fn rotate_yz(vertex: Vec3, angle: f32) -> Vec3 {
    let sin = angle.sin();
    let cos = angle.cos();
    Vec3::new(
        vertex.x,
        vertex.y * cos - vertex.z * sin,
        vertex.y * sin + vertex.z * cos,
    )
}

fn transform_camera(vertex: Vec3, camera: &Camera) -> Vec3 {
    let yaw_sin = camera.yaw.sin();
    let yaw_cos = camera.yaw.cos();
    let pitch_sin = camera.pitch.sin();
    let pitch_cos = camera.pitch.cos();
    let translated: Vec3 = Vec3::new(
        vertex.x - camera.x,
        vertex.y - camera.y,
        vertex.z + camera.z,
    );
    let rotated_xz: Vec3 = Vec3::new(
        translated.x * yaw_cos - translated.z * yaw_sin,
        translated.y,
        translated.x * yaw_sin + translated.z * yaw_cos,
    );
    Vec3::new(
        rotated_xz.x,
        rotated_xz.y * pitch_cos - rotated_xz.z * pitch_sin,
        rotated_xz.y * pitch_sin + rotated_xz.z * pitch_cos,
    )
}

fn transform(vertex: Vec3, camera: &Camera, yaw: f32, pitch: f32) -> Vec2 {
    screen(project(
        transform_camera(rotate_yz(rotate_xz(vertex, yaw), pitch), camera),
        camera.fov,
    ))
}

fn load_obj(path: &Path, vertex_offset: u16) -> (Vec<Vec3>, Vec<Vec<u16>>) {
    let mut vs: Vec<Vec3> = Vec::new();
    let mut fs: Vec<Vec<u16>> = Vec::new();
    let display = path.display();

    let mut file = match File::open(&path) {
        Err(why) => panic!("Couldn't open {}: {}", display, why),
        Ok(file) => file,
    };

    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("Couldn't read {}: {}", display, why),
        Ok(_) => println!("Successfully opened {}", display),
    }

    let mut file_iterator = s.split("\n");

    while let Some(line) = file_iterator.next() {
        let Some(line_type) = line.get(0..2) else {
            continue;
        };
        if line_type == "v " {
            let mut values = line.trim().split(" ").filter(|&x| !x.is_empty());
            values.next();
            vs.push(Vec3::new(
                values.next().unwrap().parse::<f32>().unwrap(),
                values.next().unwrap().parse::<f32>().unwrap(),
                values.next().unwrap().parse::<f32>().unwrap(),
            ));
        } else if line_type == "f " {
            let mut triangles = line.trim().split(" ").filter(|&x| !x.is_empty());
            let mut face: Vec<u16> = Vec::new();
            triangles.next();
            while let Some(index) = triangles.next() {
                face.push(
                    index.split("/").next().unwrap().parse::<u16>().unwrap() + vertex_offset - 1,
                )
            }
            fs.push(face);
        }
    }

    (vs, fs)
}

#[macroquad::main("BasicShapes")]
async fn main() {
    let mut yaw: f32 = 0.0;
    let mut pitch: f32 = 0.0;
    let mut rotate: bool = false;
    let mut edges: bool = true;
    let mut vertices: bool = false;
    let mut hud: bool = true;
    let mut camera: Camera = Camera {
        x: 0.0,
        y: 0.0,
        z: 5.0,
        yaw: 0.0,
        pitch: 0.0,
        fov: 1.0,
    };
    let mut command: String = String::new();
    let mut typing_command: bool = false;

    let mut vs: Vec<Vec3> = Vec::new();
    let mut fs: Vec<Vec<u16>> = Vec::new();

    loop {
        clear_background(Color::from_hex(0x0d0d0d));
        let hud_items: &[String] = &[
            format!("X: {}", camera.x),
            format!("Y: {}", camera.y),
            format!("Z: {}", camera.z),
            format!("YAW: {}", camera.yaw),
            format!("PITCH: {}", camera.pitch),
            format!("FOV: {}", camera.fov),
            format!("ROTATE: {}", if rotate { "ON" } else { "OFF" }),
            format!("EDGES: {}", if edges { "ON" } else { "OFF" }),
            format!("VERTICES: {}", if vertices { "ON" } else { "OFF" }),
        ];
        if hud {
            draw_fps();
            for (i, item) in hud_items.iter().enumerate() {
                draw_text(item, 0.0, 35.0 + (20.0 * i as f32), 30.0, WHITE);
            }
        }

        let yaw_sin = camera.yaw.sin();
        let yaw_cos = camera.yaw.cos();
        let pitch_sin = camera.pitch.sin();
        let pitch_cos = camera.pitch.cos();

        if is_key_pressed(KeyCode::Enter) {
            typing_command = !typing_command;
        }

        if typing_command {
            widgets::InputText::new(hash!())
                .label("Enter Command")
                .size(Vec2::new(screen_width(), 30.0))
                .position(Vec2::new(screen_width() / 4.0, screen_height() - 30.0))
                .ui(&mut root_ui(), &mut command);
        } else {
            if !command.is_empty() {
                let (mut new_vs, mut new_fs) = load_obj(&Path::new(&command), vs.len() as u16);
                vs.append(&mut new_vs);
                fs.append(&mut new_fs);
                command = String::new();
            }

            if is_key_pressed(KeyCode::Space) {
                rotate = !rotate;
            }
            if is_key_pressed(KeyCode::L) {
                edges = !edges;
            }
            if is_key_pressed(KeyCode::V) {
                vertices = !vertices;
            }
            if is_key_pressed(KeyCode::H) {
                hud = !hud;
            }

            if is_key_down(KeyCode::A) {
                camera.x -= 0.5 * yaw_cos;
                camera.z -= 0.5 * yaw_sin;
            }
            if is_key_down(KeyCode::D) {
                camera.x += 0.5 * yaw_cos;
                camera.z += 0.5 * yaw_sin;
            }
            if is_key_down(KeyCode::W) {
                camera.x += 0.5 * pitch_cos * yaw_sin;
                camera.y += 0.5 * pitch_sin;
                camera.z -= 0.5 * pitch_cos * yaw_cos;
            }
            if is_key_down(KeyCode::S) {
                camera.x -= 0.5 * pitch_cos * yaw_sin;
                camera.y -= 0.5 * pitch_sin;
                camera.z += 0.5 * pitch_cos * yaw_cos;
            }
            if is_key_down(KeyCode::Q) {
                camera.y -= 0.5;
            }
            if is_key_down(KeyCode::E) {
                camera.y += 0.5;
            }
        }

        camera.fov = (camera.fov + mouse_wheel().1 / 2.0).clamp(0.5, 10.0);

        if rotate {
            yaw += 0.01;
        }

        if is_mouse_button_down(MouseButton::Right) {
            camera.yaw -= mouse_delta_position().x;
            camera.pitch += mouse_delta_position().y;
        }

        if vertices {
            for v in &vs {
                draw_point(transform(*v, &camera, yaw, pitch));
            }
        }

        if edges {
            for f in &fs {
                for i in 0..f.len() {
                    let a = vs[f[i] as usize];
                    let b = vs[f[(i + 1) % f.len()] as usize];
                    draw_edge(
                        transform(a, &camera, yaw, pitch),
                        transform(b, &camera, yaw, pitch),
                        &camera,
                        transform_camera(a, &camera),
                        transform_camera(b, &camera),
                    );
                }
            }
        }

        next_frame().await
    }
}
