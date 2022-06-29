use cgmath::Transform;
use cgmath::Rad;
use cgmath::InnerSpace;

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

    fn translocate(&mut self, diff: Vector3) {
        self.coordinate += diff;
    }


    fn rotate_x(&mut self, theta_x: f32, center_y: f32, center_z: f32) {
        let diff = Vector3::new(0.0, center_y, center_z);
        self.translocate(-diff);
        let rad = Rad(theta_x);
        let rot_x = Matrix4::from_angle_x(rad);
        self.coordinate = rot_x.transform_vector(self.coordinate);
        self.normal = rot_x.transform_vector(self.normal);
        self.translocate(diff);
    }

    fn rotate_y(&mut self, theta_y: f32, center_x: f32, center_z: f32) {
        let diff = Vector3::new(center_x, 0.0, center_z);
        self.translocate(-diff);
        let rad = Rad(theta_y);
        let rot_y = Matrix4::from_angle_y(rad);
        self.coordinate = rot_y.transform_vector(self.coordinate);
        self.normal = rot_y.transform_vector(self.normal);
        self.translocate(diff);
    }

    fn rotate_z(&mut self, theta_z: f32, center_x: f32, center_y: f32) {
        let diff = Vector3::new(center_x, center_y, 0.0);
        self.translocate(-diff);
        let rad = Rad(theta_z);
        let rot_z = Matrix4::from_angle_z(rad);
        self.coordinate = rot_z.transform_vector(self.coordinate);
        self.normal = rot_z.transform_vector(self.normal);
        self.translocate(diff);
    }
}


struct PoolarVector {
    begin: Vector3,
    r: Vector3,
    theta: f32,
    phi: f32,
}

impl  PoolarVector{
    fn new(begin: Vector3, r: Vector3, theta: f32, phi: f32) -> PoolarVector {
        PoolarVector {
            begin,
            r,
            theta,
            phi,
        }
    }

    fn to_xyz(&self) -> Vector3 {
        let x = self.r.x*self.theta.sin()*self.phi.cos()+self.begin.x;
        let y = self.r.y*self.theta.sin()*self.phi.sin()+self.begin.y;
        let z = self.r.z*self.theta.cos()+self.begin.z;
        Vector3::new(x, y, z)
    }

    fn to_normal_xyz(&self) -> Vector3 {
        let r_norm = self.r.magnitude();
        let nx = (self.r.x/r_norm)*self.theta.sin()*self.phi.cos();
        let ny = (self.r.y/r_norm)*self.theta.sin()*self.phi.sin();
        let nz = (self.r.z/r_norm)*self.theta.cos();
        Vector3::new(nx, ny, nz)
    }
}

pub trait Object {
    fn new() -> Self;

    fn rescale_x(&mut self, scale: f32);
    
    fn rescale_y(&mut self, scale: f32);
    
    fn rescale_z(&mut self, scale: f32);

    fn recolor(&mut self, color: Vector4);

    fn generate_nodes(&mut self);
    
    fn translocate(&mut self, diff: Vector3);
    
    fn rotate_x(&mut self, theta_x: f32, center_y: f32, center_z: f32);
    
    fn rotate_y(&mut self, theta_y: f32, center_x: f32, center_z: f32);
    
    fn rotate_z(&mut self, theta_z: f32, center_x: f32, center_y: f32);    
    
    fn encode(self) -> Vec<f32>;
    // and so on
}

pub struct Sphere {
    pub center: Vector3,
    pub scale: Vector3,
    pub color: Vector4,
    pub nodes: Vec<Node>,
}

impl Object for Sphere {
    fn new() -> Sphere {
        Sphere {
            center: Vector3::new(0.0, 0.0, 0.0),
            scale: Vector3::new(0.0, 0.0, 0.0),
            color: Vector4::new(0.0, 0.0, 0.0, 1.0),
            nodes: Vec::new(),
        }
    }

    fn rescale_x(&mut self, scale: f32) {
        self.scale.x = scale;
    }

    fn rescale_y(&mut self, scale: f32) {
        self.scale.y = scale;
    }

    fn rescale_z(&mut self, scale: f32) {
        self.scale.z = scale;
    }

    fn recolor(&mut self, color: Vector4) {
        self.color = color;
    }

    fn generate_nodes(&mut self){
        let texture = Vector2::new(0.0, 0.0);
        for slice in 0..32 {
            for stack in 0..32 {
                let theta = ((slice) as f32 / 32.0) * std::f32::consts::PI*2.0;
                let phi = ((stack) as f32 / 32.0) * std::f32::consts::PI*2.0;
                let poolar_vector = PoolarVector::new(self.center, self.scale , theta, phi);
                self.nodes.push(Node::new(poolar_vector.to_xyz(), poolar_vector.to_normal_xyz(), self.color, texture));

                // 2
                let theta = ((slice+1) as f32 / 32.0) * std::f32::consts::PI*2.0;
                let phi = ((stack) as f32 / 32.0) * std::f32::consts::PI*2.0;
                let poolar_vector = PoolarVector::new(self.center, self.scale , theta, phi);
                self.nodes.push(Node::new(poolar_vector.to_xyz(), poolar_vector.to_normal_xyz(), self.color, texture));

                // 3
                let theta = ((slice+1) as f32 / 32.0) * std::f32::consts::PI*2.0;
                let phi = ((stack+1) as f32 / 32.0) * std::f32::consts::PI*2.0;
                let poolar_vector = PoolarVector::new(self.center, self.scale , theta, phi);
                self.nodes.push(Node::new(poolar_vector.to_xyz(), poolar_vector.to_normal_xyz(), self.color, texture));

                // 4
                let theta = ((slice) as f32 / 32.0) * std::f32::consts::PI*2.0;
                let phi = ((stack) as f32 / 32.0) * std::f32::consts::PI*2.0;
                let poolar_vector = PoolarVector::new(self.center, self.scale , theta, phi);
                self.nodes.push(Node::new(poolar_vector.to_xyz(), poolar_vector.to_normal_xyz(), self.color, texture));

                // 5
                let theta = ((slice+1) as f32 / 32.0) * std::f32::consts::PI*2.0;
                let phi = ((stack+1) as f32 / 32.0) * std::f32::consts::PI*2.0;
                let poolar_vector = PoolarVector::new(self.center, self.scale , theta, phi);
                self.nodes.push(Node::new(poolar_vector.to_xyz(), poolar_vector.to_normal_xyz(), self.color, texture));

                // 6
                let theta = ((slice) as f32 / 32.0) * std::f32::consts::PI*2.0;
                let phi = ((stack+1) as f32 / 32.0) * std::f32::consts::PI*2.0;
                let poolar_vector = PoolarVector::new(self.center, self.scale , theta, phi);
                self.nodes.push(Node::new(poolar_vector.to_xyz(), poolar_vector.to_normal_xyz(), self.color, texture));
            }
        }
    }

    fn translocate(&mut self, diff: Vector3) {
        for node in &mut self.nodes {
            node.translocate(diff);
        }
    }

    fn rotate_x(&mut self, theta_x: f32, center_y: f32, center_z: f32) {
        for node in &mut self.nodes {
            node.rotate_x(theta_x, center_y, center_z);
        }
    }

    fn rotate_y(&mut self, theta_y: f32, center_x: f32, center_z: f32) {
        for node in &mut self.nodes {
            node.rotate_y(theta_y, center_x, center_z);
        }
    }

    fn rotate_z(&mut self, theta_z: f32, center_x: f32, center_y: f32) {
        for node in &mut self.nodes {
            node.rotate_z(theta_z, center_x, center_y);
        }
    }

    fn encode(self) -> Vec<f32> {
        let mut ret = vec![];
        for node in self.nodes {
            for val in node.encode() {
                ret.push(val);
            }
        }
        ret
    }
}

