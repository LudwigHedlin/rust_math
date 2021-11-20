#![allow(dead_code)]

mod math;
use math::vec3::Vec3;
use math::vec3::Vec3Ops;
use math::mat3::Mat3;
use math::mat3;


fn main() {
    let mut v = Vec3{x:1.0,y:1.0,z:0.0};
    let v2 = Vec3{x:0.0,y:0.0,z:1.0};
    let v3 = v.cross(&v2);

    let m = Mat3{x1:0.0,x2:2.0,x3:0.0,x4:0.0,x5:3.0,x6:0.0,x7:2.0,x8:1.0,x9:0.0,};
    let m2 = Mat3{x1:1.0,x2:4.0,x3:0.0,x4:0.0,x5:3.0,x6:0.0,x7:0.0,x8:0.0,x9:0.0,};
    v._cross(&v3);
    println!("{},{},{},{}",v*v2,v3.x,v3.y,v3.z);
    println!("{:?}",(m*m2)*3.0);
    println!("{:?}",v);
    let m3 = mat3::identity_matrix3();
    println!("{:?}",m3);
}
