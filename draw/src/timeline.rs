pub enum ObjectKind {
    Sphere,
    Cube,
    Tetrahedron,
    Circle,
    Square,
    Triangle,
    Image,
    Str,
}

pub struct Object {
    kind: ObjectKind,
    start: i32,
    end: i32,
    x_start: f32,
    x_end: f32,
    y_start: f32,
    y_end: f32,
    z_start: f32,
    z_end: f32,
    rx_start: f32,
    rx_end: f32,
    ry_start: f32,
    ry_end: f32,
    rz_start: f32,
    rz_end: f32,
}

pub struct Layer {
    objects: Vec<Object>,
}

pub struct TimeLine {
    layers: Vec<Layer>,
}

impl Object {
    pub draw(self);
}
