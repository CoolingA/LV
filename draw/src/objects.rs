use cgmath::Transform;
use cgmath::Rad;

type Vector3 = cgmath::Vector3<f32>;
type Matrix3 = cgmath::Matrix3<f32>;
type Matrix4 = cgmath::Matrix4<f32>;
type Vector4 = cgmath::Vector4<f32>;
type Vector2 = cgmath::Vector2<f32>;

pub struct Node {
    coordinate: Vector3,
    normal: Vector3,
    color: Vector4,
    texture: Vector2,
}

impl Node {
    fn new(coordinate: Vector3, normal: Vector3, color: Vector4, texture: Vector2) -> Self {
        Node {
            coordinate,
            normal,
            color,
            texture,
        }
    }

    fn encode(self) -> Vec<f32> {
        vec![
            self.coordinate.x, self.coordinate.y, self.coordinate.z,
            self.normal.x, self.normal.y, self.normal.z,
            self.color.x, self.color.y, self.color.z, self.color.w,
            self.texture.x, self.texture.y
        ]
    }
}


struct PoolarVector {
    begin: Vector3,
    r: f32,
    theta: f32,
    phi: f32,
}

impl PoolarVector {
    fn new(begin: Vector3, r: f32, theta: f32, phi: f32) -> PoolarVector {
        PoolarVector {
            begin,
            r,
            theta,
            phi,
        }
    }

    fn to_xyz(&self) -> Vector3 {
        let x = self.r*self.theta.sin()*self.phi.cos()+self.begin.x;
        let y = self.r*self.theta.sin()*self.phi.sin()+self.begin.y;
        let z = self.r*self.theta.cos()+self.begin.z;
        Vector3::new(x, y, z)
    }

    fn to_normal_xyz(&self) -> Vector3 {
        let nx = self.theta.sin()*self.phi.cos();
        let ny = self.theta.sin()*self.phi.sin();
        let nz = self.theta.cos();
        Vector3::new(nx, ny, nz)
    }
}

pub trait Object {
    fn new() -> Self;

    fn encode(self) -> Vec<f32>;

    fn translocate(&mut self, diff: Vector3);

    fn rotate_x(&mut self, theta_x: f32, center_y: f32, center_z: f32);

    fn rotate_y(&mut self, theta_y: f32, center_x: f32, center_z: f32);

    fn rotate_z(&mut self, theta_z: f32, center_x: f32, center_y: f32);
    
    fn set_color(&mut self, color: Vector4) {
        self.color = color;
    }

    fn rescale(&mut self, scale: f32) {
        self.scale = scale;
    }

    // and so on
}

pub struct Sphere {
    pub center: Vector3,
    pub scale: f32,
    pub color: Vector4,
}

impl Object for Sphere {
    fn new() -> Sphere {
        Sphere {
            center: Vector3::new(0.0, 0.0, 0.0),
            scale: 1.0,
            color: Vector4::new(0.0, 0.0, 0.0),
        }
    }

    fn encode(self) -> Vec<f32> {
        let mut nodes = vec![];

        let texture = Vector2::new(0.0, 0.0);
        for slice in 0..32 {
            for stack in 0..32 {
                // 1
                let theta = ((slice) as f32 / 32.0) * std::f32::consts::PI*2.0;
                let phi = ((stack) as f32 / 32.0) * std::f32::consts::PI*2.0;
                let poolar_vector = PoolarVector::new(self.center, self.scale, theta, phi);
                nodes.push(Node::new(poolar_vector.to_xyz(), poolar_vector.to_normal_xyz(), self.color, texture));

                // 2
                let theta = ((slice+1) as f32 / 32.0) * std::f32::consts::PI*2.0;
                let phi = ((stack) as f32 / 32.0) * std::f32::consts::PI*2.0;
                let poolar_vector = PoolarVector::new(self.center, self.scale, theta, phi);
                nodes.push(Node::new(poolar_vector.to_xyz(), poolar_vector.to_normal_xyz(), self.color, texture));

                // 3
                let theta = ((slice+1) as f32 / 32.0) * std::f32::consts::PI*2.0;
                let phi = ((stack+1) as f32 / 32.0) * std::f32::consts::PI*2.0;
                let poolar_vector = PoolarVector::new(self.center, self.scale, theta, phi);
                nodes.push(Node::new(poolar_vector.to_xyz(), poolar_vector.to_normal_xyz(), self.color, texture));

                // 4
                let theta = ((slice) as f32 / 32.0) * std::f32::consts::PI*2.0;
                let phi = ((stack) as f32 / 32.0) * std::f32::consts::PI*2.0;
                let poolar_vector = PoolarVector::new(self.center, self.scale, theta, phi);
                nodes.push(Node::new(poolar_vector.to_xyz(), poolar_vector.to_normal_xyz(), self.color, texture));

                // 5
                let theta = ((slice+1) as f32 / 32.0) * std::f32::consts::PI*2.0;
                let phi = ((stack+1) as f32 / 32.0) * std::f32::consts::PI*2.0;
                let poolar_vector = PoolarVector::new(self.center, self.scale, theta, phi);
                nodes.push(Node::new(poolar_vector.to_xyz(), poolar_vector.to_normal_xyz(), self.color, texture));

                // 6
                let theta = ((slice) as f32 / 32.0) * std::f32::consts::PI*2.0;
                let phi = ((stack+1) as f32 / 32.0) * std::f32::consts::PI*2.0;
                let poolar_vector = PoolarVector::new(self.center, self.scale, theta, phi);
                nodes.push(Node::new(poolar_vector.to_xyz(), poolar_vector.to_normal_xyz(), self.color, texture));
            }
        }

        let mut ret = vec![];
        for node in nodes {
            for val in node.encode() {
                ret.push(val);
            }
        }
        ret
    }

    fn translocate(&mut self, diff: Vector3) {
        self.center += diff;
    }


    fn rotate_x(&mut self, theta_x: f32, center_y: f32, center_z: f32) {
        let diff = Vector3::new(0.0, center_y, center_z);
        self.translocate(-diff);
        let rad = Rad(theta_x);
        let rot_x = Matrix4::from_angle_x(rad);
        self.center = rot_x.transform_vector(self.center);
        self.translocate(diff);
    }

    fn rotate_y(&mut self, theta_y: f32, center_x: f32, center_z: f32) {
        let diff = Vector3::new(0.0, center_x, center_z);
        self.translocate(-diff);
        let rad = Rad(theta_y);
        let rot_y = Matrix4::from_angle_x(rad);
        self.center = rot_y.transform_vector(self.center);
        self.translocate(diff);
    }

    fn rotate_z(&mut self, theta_z: f32, center_x: f32, center_y: f32) {
        let diff = Vector3::new(0.0, center_x, center_y);
        self.translocate(-diff);
        let rad = Rad(theta_z);
        let rot_z = Matrix4::from_angle_x(rad);
        self.center = rot_z.transform_vector(self.center);
        self.translocate(diff);
    }
}

