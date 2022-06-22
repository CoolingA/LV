mod cg_system;
mod objects;

use cg_system::CGExecutor;
use objects::Sphere;

type Vector3 = cgmath::Vector3<f32>;
type Vector4 = cgmath::Vector4<f32>;
type Vector2 = cgmath::Vector2<f32>;
type Point3 = cgmath::Point3<f32>;
type Matrix4 = cgmath::Matrix4<f32>;

fn main() {
    let window_width: u32 = 1920;
    let window_height: u32 = 1080;

    let mut vertex_array = vec![];
    for sphere_index in 0..5 {
        let sphere_center = Vector3::new(0.0, sphere_index as f32 * 3.0 - 9.0, 0.0);
        let sphere_radius = (sphere_index+1) as f32;
        let color = Vector4::new(sphere_index as f32 / 5.0, 0.0, 0.5, 1.0);
        let sphere = Sphere::new();
        sphere.translocate(sphere_center);
        sphere.rescale(sphere_radius);
        sphere.set_color(color);
        vertex_array.push(sphere.encode());
    }
    let camera_x = 20.0 as f32;
    let camera_y = -20.0 as f32;
    let camera_z = 20.0 as f32;
    let view_matrix = Matrix4::look_at(
        Point3::new(camera_x, camera_y, camera_z),
        Point3::new(0.0, 0.0, 0.0),
        Vector3::new(0.0, 0.0, 1.0),
    );
    let material_specular = Vector3::new(0.2, 0.2, 0.2);
    let material_shininess = 0.1 as f32;
    let light_direction = Vector3::new(1.0, 1.0, 0.0);
    let light_ambient = Vector3::new(0.3, 0.3, 0.3);
    let light_diffuse = Vector3::new(0.5, 0.5, 0.5);
    let light_specular = Vector3::new(0.2, 0.2, 0.2);

    let mut executor = CGExecutor::new(
        window_width,
        window_height,
        vertex_array,
        camera_x,
        camera_y,
        camera_z,
        view_matrix,
        material_specular,
        material_shininess,
        light_direction,
        light_ambient,
        light_diffuse,
        light_specular
    );
    executor.execute();

}
