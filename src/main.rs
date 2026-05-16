use macroquad::prelude::*;

#[derive(Copy, Clone)]
struct Camera {
    x: f32,
    y: f32,
    z: f32,
    yaw: f32,
    pitch: f32,
    fov: f32
}

fn point(pos: Vec2) -> () {
    draw_circle(pos.x, pos.y, 5.0, DARKGREEN);
}

fn line(p1: Vec2, p2: Vec2, camera: Camera, end1: Vec3, end2: Vec3) -> () {
    let distance1: f32 = (
        (camera.x - end1.x).powi(2) +
        (camera.y - end1.y).powi(2) +
        (camera.z - end1.z).powi(2)
    ).sqrt();
    let distance2: f32 = (
        (camera.x - end2.x).powi(2) +
        (camera.y - end2.y).powi(2) +
        (camera.z - end2.z).powi(2)
    ).sqrt();
    let mean_distance = (distance1 + distance2) / 2.0;
    draw_line(p1.x, p1.y, p2.x, p2.y, (10.0/mean_distance).clamp(1.5, 3.0), GREEN);
}

fn screen(point: Vec2) -> Vec2 {
    Vec2::new(
        (point.x + 1.0) / 2.0 * screen_width(),
        (1.0 - (point.y + 1.0) / 2.0) * screen_height(),
    )
}

fn project(vertice: Vec3, fov: f32) -> Vec2 {
    Vec2::new((vertice.x / vertice.z) * fov, (vertice.y / vertice.z) * fov)
}

fn translate_z(vertice: Vec3, dz: f32) -> Vec3 {
    Vec3 {
        z: vertice.z + dz,
        ..vertice
    }
}

fn rotate_xz(vertice: Vec3, angle: f32) -> Vec3 {
    let sin = angle.sin();
    let cos = angle.cos();
    Vec3::new(
        vertice.x * cos - vertice.z * sin,
        vertice.y,
        vertice.x * sin + vertice.z * cos,
    )
}

fn rotate_yz(vertice: Vec3, angle: f32) -> Vec3 {
    let sin = angle.sin();
    let cos = angle.cos();
    Vec3::new(
        vertice.x,
        vertice.y * cos - vertice.z * sin,
        vertice.y * sin + vertice.z * cos,
    )
}

fn transform_camera(vertice: Vec3, camera: Camera) -> Vec3 {
    let sin_xz = camera.yaw.sin();
    let cos_xz = camera.yaw.cos();
    let sin_yz = camera.pitch.sin();
    let cos_yz = camera.pitch.cos();
    let translated: Vec3 = Vec3::new(
        vertice.x - camera.x,
        vertice.y - camera.y,
        vertice.z + camera.z
    );
    let rotated_xz: Vec3 = Vec3::new(
        translated.x * cos_xz - translated.z * sin_xz,
        translated.y,
        translated.x * sin_xz + translated.z * cos_xz,
    );
    Vec3::new(
        rotated_xz.x,
        rotated_xz.y * cos_yz - rotated_xz.z * sin_yz,
        rotated_xz.y * sin_yz + rotated_xz.z * cos_yz,
    )
}

fn transform(vertice: Vec3, camera: Camera, yaw: f32, pitch: f32) -> Vec2 {
    screen(project(transform_camera(
        rotate_yz(rotate_xz(vertice, yaw), pitch),
        camera,
    ), camera.fov))
}

#[macroquad::main("BasicShapes")]
async fn main() {
    let mut yaw: f32 = 0.0;
    let mut pitch: f32 = 0.0;
    let mut auto_rotate_xz: bool = false;
    let mut edges: bool = true;
    let mut vertices: bool = false;
    let mut camera: Camera = Camera {
        x: 0.0,
        y: 0.0,
        z: 5.0,
        yaw: 0.0,
        pitch: 0.0,
        fov: 1.0
    };
    let vs: &[Vec3] = &[
        Vec3::new(3.0, 3.0, 3.0),
        Vec3::new(-3.0, 3.0, 3.0),
        Vec3::new(-3.0, -3.0, 3.0),
        Vec3::new(3.0, -3.0, 3.0),
        Vec3::new(3.0, 3.0, -3.0),
        Vec3::new(-3.0, 3.0, -3.0),
        Vec3::new(-3.0, -3.0, -3.0),
        Vec3::new(3.0, -3.0, -3.0),
        Vec3::new(1.0, 1.0, 1.0),
        Vec3::new(1.0, -1.0, -1.0),
        Vec3::new(-1.0, 1.0, -1.0),
        Vec3::new(-1.0, -1.0, 1.0),
    ];
    let fs: &[Vec<u8>] = &[
        vec![0, 1, 2, 3],
        vec![4, 5, 6, 7],
        vec![0, 4],
        vec![1, 5],
        vec![2, 6],
        vec![3, 7],
        vec![8, 9, 10],
        vec![9, 10, 11],
        vec![10, 11, 8],
        vec![11, 8, 9],
    ];
    // let vs: &[Vec3] = &[
    //     Vec3::new(0.01265248, 0.1958004, 0.3149356),
    //     Vec3::new(-0.01265248, 0.1958004, 0.3149356),
    //     Vec3::new(0.009372211, 0.1599542, 0.2375356),
    //     Vec3::new(-0.009372211, 0.1599542, 0.2375356),
    //     Vec3::new(-0.01874442, 0.1435366, 0.2375356),
    //     Vec3::new(-0.02530497, 0.1736366, 0.3149356),
    //     Vec3::new(0.01874442, 0.1435366, 0.2375356),
    //     Vec3::new(0.009372211, 0.127119, 0.2375356),
    //     Vec3::new(0.02530497, 0.1736366, 0.3149356),
    //     Vec3::new(0.01265248, 0.1514729, 0.3149356),
    //     Vec3::new(-0.01265248, 0.1514729, 0.3149356),
    //     Vec3::new(-0.009372211, 0.127119, 0.2375356),
    //     Vec3::new(-0.01192523, 0.1478274, -0.3149356),
    //     Vec3::new(-0.02385046, 0.1684825, -0.3149356),
    //     Vec3::new(-0.01327069, 0.126897, -0.2920356),
    //     Vec3::new(-0.02654139, 0.1498825, -0.2920356),
    //     Vec3::new(0.02385046, 0.1684825, -0.3149356),
    //     Vec3::new(0.02654139, 0.1498825, -0.2920356),
    //     Vec3::new(0.01192523, 0.1891377, -0.3149356),
    //     Vec3::new(0.01327069, 0.1728681, -0.2920356),
    //     Vec3::new(-0.01192523, 0.1891377, -0.3149356),
    //     Vec3::new(-0.01327069, 0.1728681, -0.2920356),
    //     Vec3::new(0.01327069, 0.126897, -0.2920356),
    //     Vec3::new(0.01192523, 0.1478274, -0.3149356),
    //     Vec3::new(-0.06879999, 0.05958255, -0.0272472),
    //     Vec3::new(-0.0344, 0.1191651, -0.0272472),
    //     Vec3::new(-0.060544, 0.06613663, 0.07048938),
    //     Vec3::new(-0.030272, 0.1191651, 0.07048938),
    //     Vec3::new(0.030272, 0.1191651, 0.07048938),
    //     Vec3::new(0.0344, 0.1191651, -0.0272472),
    //     Vec3::new(0.060544, 0.06613663, 0.07048938),
    //     Vec3::new(0.06879999, 0.05958255, -0.0272472),
    //     Vec3::new(0.030272, 0.01310816, 0.07048938),
    //     Vec3::new(0.0344, 0.0, -0.0272472),
    //     Vec3::new(-0.030272, 0.01310816, 0.07048938),
    //     Vec3::new(-0.0344, 0.0, -0.0272472),
    //     Vec3::new(0.032336, 0.1241901, -0.1304472),
    //     Vec3::new(0.06467199, 0.06818254, -0.1304472),
    //     Vec3::new(0.02179584, 0.1430171, 0.1796847),
    //     Vec3::new(0.04359168, 0.1048366, 0.1796847),
    //     Vec3::new(0.032336, 0.01217495, -0.1304472),
    //     Vec3::new(-0.032336, 0.1241901, -0.1304472),
    //     Vec3::new(0.02179584, 0.06665613, 0.1796847),
    //     Vec3::new(-0.02179584, 0.06665613, 0.1796847),
    //     Vec3::new(-0.02179584, 0.1430171, 0.1796847),
    //     Vec3::new(-0.04359168, 0.1048366, 0.1796847),
    //     Vec3::new(-0.02328192, 0.06225708, -0.2341808),
    //     Vec3::new(0.02328192, 0.06225708, -0.2341808),
    //     Vec3::new(-0.032336, 0.01217495, -0.1304472),
    //     Vec3::new(0.02328192, 0.142908, -0.2341808),
    //     Vec3::new(-0.02328192, 0.142908, -0.2341808),
    //     Vec3::new(-0.04656384, 0.1025825, -0.2341808),
    //     Vec3::new(-0.06467199, 0.06818254, -0.1304472),
    //     Vec3::new(0.04656384, 0.1025825, -0.2341808),
    // ];
    //
    // let fs: &[Vec<u8>] = &[
    //     vec![2, 1, 0],
    //     vec![1, 2, 3],
    //     vec![5, 3, 4],
    //     vec![3, 5, 1],
    //     vec![8, 7, 6],
    //     vec![7, 8, 9],
    //     vec![9, 5, 10],
    //     vec![5, 9, 8],
    //     vec![5, 8, 1],
    //     vec![1, 8, 0],
    //     vec![10, 7, 9],
    //     vec![7, 10, 11],
    //     vec![11, 5, 4],
    //     vec![5, 11, 10],
    //     vec![0, 6, 2],
    //     vec![6, 0, 8],
    //     vec![14, 13, 12],
    //     vec![13, 14, 15],
    //     vec![18, 17, 16],
    //     vec![17, 18, 19],
    //     vec![15, 20, 13],
    //     vec![20, 15, 21],
    //     vec![14, 23, 22],
    //     vec![23, 14, 12],
    //     vec![16, 22, 23],
    //     vec![22, 16, 17],
    //     vec![18, 21, 19],
    //     vec![21, 18, 20],
    //     vec![13, 23, 12],
    //     vec![23, 13, 16],
    //     vec![16, 13, 20],
    //     vec![16, 20, 18],
    //     vec![26, 25, 24],
    //     vec![26, 27, 25],
    //     vec![30, 29, 28],
    //     vec![30, 32, 31],
    //     vec![32, 33, 31],
    //     vec![26, 24, 34],
    //     vec![30, 31, 29],
    //     vec![34, 24, 35],
    //     vec![29, 37, 36],
    //     vec![37, 29, 31],
    //     vec![38, 30, 28],
    //     vec![30, 38, 39],
    //     vec![37, 33, 40],
    //     vec![33, 37, 31],
    //     vec![36, 25, 29],
    //     vec![25, 36, 41],
    //     vec![30, 42, 32],
    //     vec![42, 30, 39],
    //     vec![42, 11, 43],
    //     vec![11, 42, 7],
    //     vec![6, 42, 39],
    //     vec![42, 6, 7],
    //     vec![32, 35, 33],
    //     vec![35, 32, 34],
    //     vec![44, 2, 38],
    //     vec![2, 44, 3],
    //     vec![4, 44, 45],
    //     vec![44, 4, 3],
    //     vec![27, 38, 28],
    //     vec![38, 27, 44],
    //     vec![42, 34, 32],
    //     vec![34, 42, 43],
    //     vec![25, 28, 29],
    //     vec![28, 25, 27],
    //     vec![26, 44, 27],
    //     vec![44, 26, 45],
    //     vec![43, 26, 34],
    //     vec![26, 43, 45],
    //     vec![38, 6, 39],
    //     vec![6, 38, 2],
    //     vec![43, 4, 45],
    //     vec![4, 43, 11],
    //     vec![14, 47, 46],
    //     vec![47, 14, 22],
    //     vec![35, 40, 33],
    //     vec![40, 35, 48],
    //     vec![19, 50, 49],
    //     vec![50, 19, 21],
    //     vec![49, 41, 36],
    //     vec![41, 49, 50],
    //     vec![46, 52, 51],
    //     vec![52, 46, 48],
    //     vec![51, 21, 15],
    //     vec![21, 51, 50],
    //     vec![52, 50, 51],
    //     vec![50, 52, 41],
    //     vec![48, 47, 40],
    //     vec![47, 48, 46],
    //     vec![35, 52, 48],
    //     vec![52, 35, 24],
    //     vec![46, 15, 14],
    //     vec![15, 46, 51],
    //     vec![37, 47, 53],
    //     vec![47, 37, 40],
    //     vec![49, 37, 53],
    //     vec![37, 49, 36],
    //     vec![52, 25, 41],
    //     vec![25, 52, 24],
    //     vec![17, 47, 22],
    //     vec![47, 17, 53],
    //     vec![49, 17, 19],
    //     vec![17, 49, 53],
    // ];
    loop {
        clear_background(BLACK);
        draw_fps();
        draw_text(
            &format!(
                "CAM: {} {} {} {} {} {}",
                camera.x, camera.y, camera.z, camera.yaw, camera.pitch, camera.fov
            ),
            0.0,
            35.0,
            30.0,
            WHITE,
        );
        draw_text(
            &format!("AUTO ROTATE: {}", if auto_rotate_xz { "ON" } else { "OFF" }),
            0.0,
            55.0,
            30.0,
            WHITE,
        );
        draw_text(
            &format!("EDGES: {}", if edges { "ON" } else { "OFF" }),
            0.0,
        75.0,
            30.0,
            WHITE,
        );
        draw_text(
            &format!("VERTICES: {}", if vertices { "ON" } else { "OFF" }),
            0.0,
            95.0,
            30.0,
            WHITE,
        );
        
        camera.fov = (camera.fov + mouse_wheel().1 / 2.0).clamp(0.5, 10.0);

        if auto_rotate_xz {
            yaw += 0.01
        }

        if is_mouse_button_down(MouseButton::Right) {
            camera.yaw -= mouse_delta_position().x;
            camera.pitch += mouse_delta_position().y;
        }

        if is_key_pressed(KeyCode::Space) {
            auto_rotate_xz = !auto_rotate_xz;
        } else if is_key_pressed(KeyCode::L) {
            edges = !edges;
        } else if is_key_pressed(KeyCode::V) {
            vertices = !vertices;
        }

        if is_key_down(KeyCode::A) {
            camera.x -= 0.5 * camera.yaw.cos();
            camera.z -= 0.5 * camera.yaw.sin();
        } else if is_key_down(KeyCode::D) {
            camera.x += 0.5 * camera.yaw.cos();
            camera.z += 0.5 * camera.yaw.sin();
        } else if is_key_down(KeyCode::W) {
            camera.x += 0.5 * camera.pitch.cos() * camera.yaw.sin();
            camera.y += 0.5 * camera.pitch.sin();
            camera.z -= 0.5 * camera.pitch.cos() * camera.yaw.cos();
        } else if is_key_down(KeyCode::S) {
            camera.x -= 0.5 * camera.pitch.cos() * camera.yaw.sin();
            camera.y -= 0.5 * camera.pitch.sin();
            camera.z += 0.5 * camera.pitch.cos() * camera.yaw.cos();
        } else if is_key_down(KeyCode::Q) {
            camera.y -= 0.5
        } else if is_key_down(KeyCode::E) {
            camera.y += 0.5
        }

        if vertices {
            for v in vs {
                point(transform(*v, camera, yaw, pitch))
            }
        }

        if edges {
            for f in fs {
                for i in 0..f.len() {
                    let a = vs[f[i] as usize];
                    let b = vs[f[(i + 1) % f.len()] as usize];
                    line(
                        transform(a, camera, yaw, pitch),
                        transform(b, camera, yaw, pitch),
                        camera,
                        transform_camera(a, camera),
                        transform_camera(b, camera)
                    );
                }
            }
        }

        next_frame().await
    }
}
