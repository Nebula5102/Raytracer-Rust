use std::ops;

#[derive(Debug,Copy,Clone)]

pub struct Vec3 {
    pub e: (f64,f64,f64),
}

// type alias for code clarity
pub type Point3 = Vec3;
pub type Colour = Vec3;

impl Vec3 {
    pub fn new() -> Self {
        Vec3 { e: (0.0,0.0,0.0) }
    }

    pub fn Vec3(e0: f64, e1: f64, e2: f64) -> Self {
        Vec3 { e: (e0,e1,e2) }
    }

    pub fn x(&self) -> f64 {
        self.e.0
    }

    pub fn y(&self) -> f64 {
        self.e.1
    }

    pub fn z(&self) -> f64 {
        self.e.2
    }

    pub fn length_squared(&self) -> f64 {
        return self.x()*self.x() + self.y()*self.y() + self.z()*self.z();
    }

    pub fn length(&self) -> f64 {
        return f64::sqrt(self.length_squared());
    }

    pub fn unit_vector(&self) -> Vec3 {
        return *self/self.length();
    }
}

impl ops::Add<Vec3> for Vec3 {
    type Output = Vec3;

    fn add(self,_rhs: Vec3) -> Vec3 {
        return Vec3::Vec3(self.x() + _rhs.x(),self.y() + _rhs.y(),self.z() + _rhs.z());
    }
}

impl ops::Neg for Vec3 {
    type Output = Self;

    fn neg(mut self) -> Self::Output {
        self.e.0 = -self.x();
        self.e.1 = -self.y();
        self.e.2 = -self.z();
        return self
    }
}

impl ops::Sub for Vec3 {
    type Output = Vec3;

    fn sub(self,_rhs: Vec3) -> Vec3 {
        return Vec3::Vec3(self.x() - _rhs.x(), self.y() - _rhs.y(), self.z() - _rhs.z())
    }
}

impl ops::Mul<f64> for Vec3 {
    type Output = Self;

    fn mul(mut self,_rhs: f64) -> Vec3 {
        self.e.0 = self.x() * _rhs;
        self.e.1 = self.y() * _rhs;
        self.e.2 = self.z() * _rhs;
        return self;
    }
}

impl ops::Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self,mut _rhs: Vec3) -> Vec3 {
        _rhs.e.0 = _rhs.x() * self;
        _rhs.e.1 = _rhs.y() * self;
        _rhs.e.2 = _rhs.z() * self;
        return _rhs;
    }
}

impl ops::Mul<Vec3> for Vec3 {
    type Output = Vec3;

    fn mul(self,_rhs: Vec3) -> Vec3 {
        return Vec3::Vec3(self.x() * _rhs.x(),self.y() * _rhs.y(),self.z() * _rhs.z());
    }
}

impl ops::Div<f64> for Vec3 {
    type Output = Self;

    fn div(mut self,_rhs: f64) -> Vec3 {
        self.e.0 = self.x() / _rhs;
        self.e.1 = self.y() / _rhs;
        self.e.2 = self.z() / _rhs;
        return self;
    }
}

pub fn dot(u:Vec3,v:Vec3) -> f64 {
    return u.x() * v.x() + u.y() * v.y() + u.z() * v.z();
}

pub fn cross(u:Vec3,v:Vec3) -> Vec3 {
    return Vec3::Vec3(u.y() * v.z() - u.z() * v.y(), u.z() * v.x() - u.x() * v.z(), u.x() * v.y() - u.y() * v.x())
}

pub fn colour_string(pixel_colour: Colour) -> String {
    let formatted = format!("{} {} {}\n",(pixel_colour.x()*255.99) as i64,(pixel_colour.y()*255.99) as i64 ,(pixel_colour.z()*255.99) as i64);
    return formatted;
}

fn main() {
    /*
    let basic_declaration = Vec3::new();
    let custom_declaration = Vec3::Vec3(2.0,0.0,0.0);
    println!("{:?}",basic_declaration);
    println!("{:?}",custom_declaration);

    println!("{:?}",custom_declaration.x());
    println!("{:?}",custom_declaration.y());
    println!("{:?}",custom_declaration.z());
    println!("{:?}",custom_declaration+basic_declaration);
    println!("{:?}", custom_declaration.length());
    println!("{:?}", -custom_declaration);
    
    let vec1 = Vec3::Vec3(2.0,2.0,2.0);
    let vec2 = Vec3::Vec3(1.0,2.0,3.0);
    let vec3 = vec1 * vec2;
    println!("{:?}",vec3.unit_vector());
    let dot = dot(vec1,vec2);
    let n = cross(vec1,vec2);
    println!("{:?}",n);
    println!("{}",dot);
    let point = Point3::Vec3(1.0,2.0,3.0);
    cross(point,vec1);
    println!("{:?}",point);
    let colour = Colour::Vec3(1.0,1.0,1.0);
    println!("{}",colour_string(colour));
    let vec1 = Vec3::Vec3(2.0,3.0,1.0);
    let vec2 = Vec3::Vec3(1.0,1.0,0.0);
    let vec3 = vec1 - vec2;
    println!("Answer should be (1.0,2.0,1.0) answer is: {:?}",vec3);
    let vec4 = vec1 * 2.0;
    println!("Answer should be (4.0,6.0,2.0) answer is: {:?}",vec4);
    let vec5 = 2.0 * vec1;
    println!("Answer should be (4.0,6.0,2.0) answer is: {:?}",vec5);
    let vec7 = Vec3::Vec3(4.0,8.0,12.0)/2.0;
    println!("{:?}",vec7);
    let vec6 = vec2.unit_vector();
    println!("Answer should be (0....,0....,0.0) answer is: {:?}",vec6);
    */
}
