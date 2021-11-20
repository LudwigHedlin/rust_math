
use std::ops;

use crate::math::vec3::Vec3;
use crate::math::quaternion::Quaternion;
#[derive(Copy, Clone,Debug)]
pub struct Mat3{
    pub x1: f64,
    pub x2: f64,
    pub x3: f64,
    pub x4: f64,
    pub x5: f64,
    pub x6: f64,
    pub x7: f64,
    pub x8: f64,
    pub x9: f64,
}

impl ops::Add<Mat3> for &Mat3{
    type Output = Mat3;
    fn add(self,m:Mat3)->Mat3{
        let mut m2 = Mat3{x1:0.0,x2:0.0,x3:0.0,x4:0.0,x5:0.0,x6:0.0,x7:0.0,x8:0.0,x9:0.0,};
        m2.x1=self.x1+m.x1;
        m2.x2=self.x2+m.x2;
        m2.x3=self.x3+m.x3;
        m2.x4=self.x4+m.x4;
        m2.x5=self.x5+m.x5;
        m2.x6=self.x6+m.x6;
        m2.x7=self.x7+m.x7;
        m2.x8=self.x8+m.x8;
        m2.x9=self.x9+m.x9;
        m2
    }
}

impl ops::Sub<Mat3> for Mat3{
    type Output = Mat3;
    fn sub(self,m:Mat3)->Mat3{
        let mut m2 = Mat3{x1:0.0,x2:0.0,x3:0.0,x4:0.0,x5:0.0,x6:0.0,x7:0.0,x8:0.0,x9:0.0,};
        m2.x1=self.x1-m.x1;
        m2.x2=self.x2-m.x2;
        m2.x3=self.x3-m.x3;
        m2.x4=self.x4-m.x4;
        m2.x5=self.x5-m.x5;
        m2.x6=self.x6-m.x6;
        m2.x7=self.x7-m.x7;
        m2.x8=self.x8-m.x8;
        m2.x9=self.x9-m.x9;
        m2
    }
}

impl ops::Mul<Vec3> for Mat3{
    type Output = Vec3;
    fn mul(self,v:Vec3)->Vec3{
        let mut v2 = Vec3{x:0.0,y:0.0,z:0.0};
        v2.x=v.x*self.x1+v.y*self.x2+v.z*self.x3;
        v2.y=v.x*self.x4+v.y*self.x5+v.z*self.x6;
        v2.z=v.x*self.x7+v.y*self.x8+v.z*self.x9;
        v2
    }
}
impl ops::Mul<& mut Vec3> for Mat3{
    type Output = Vec3;
    fn mul(self,v:& mut Vec3)->Vec3{
        let mut v2 = Vec3{x:0.0,y:0.0,z:0.0};
        v2.x=v.x*self.x1+v.y*self.x2+v.z*self.x3;
        v2.y=v.x*self.x4+v.y*self.x5+v.z*self.x6;
        v2.z=v.x*self.x7+v.y*self.x8+v.z*self.x9;
        v2
    }
}
impl ops::Mul<Mat3> for Mat3{
    type Output = Mat3;
    fn mul(self,m:Mat3)->Mat3{
        let mut m2=Mat3{x1:0.0,x2:0.0,x3:0.0,x4:0.0,x5:0.0,x6:0.0,x7:0.0,x8:0.0,x9:0.0,};
        m2.x1=m.x1*self.x1+m.x4*self.x2+m.x7*self.x3;
        m2.x2=m.x2*self.x1+m.x5*self.x2+m.x8*self.x3;
        m2.x3=m.x3*self.x1+m.x6*self.x2+m.x9*self.x3;
        m2.x4=m.x1*self.x4+m.x4*self.x5+m.x7*self.x6;
        m2.x5=m.x2*self.x4+m.x5*self.x5+m.x8*self.x6;
        m2.x6=m.x3*self.x4+m.x6*self.x5+m.x9*self.x6;
        m2.x7=m.x1*self.x7+m.x4*self.x8+m.x7*self.x9;
        m2.x8=m.x2*self.x7+m.x5*self.x8+m.x8*self.x9;
        m2.x9=m.x3*self.x7+m.x6*self.x8+m.x9*self.x9;
        m2
    }
}

impl ops::Mul<f64> for Mat3{
    type Output = Mat3;
    fn mul(self,f:f64)->Mat3{
        Mat3{x1:self.x1*f,x2:self.x2*f,x3:self.x3*f,x4:self.x4*f,x5:self.x5*f,x6:self.x6*f,x7:self.x7*f,x8:self.x8*f,x9:self.x9*f}
    }
}

impl ops::Div<f64> for Mat3{
    type Output = Mat3;
    fn div(self,f:f64)->Mat3{
        self*(1.0/f)
    }
}

impl ops::MulAssign<Mat3> for Mat3{
    fn mul_assign(&mut self,m:Mat3){
        *self=*self*m;
    }
}

impl ops::MulAssign<f64> for Mat3{
    fn mul_assign(&mut self,f:f64){
        self.x1*=f;
        self.x2*=f;
        self.x3*=f;
        self.x4*=f;
        self.x5*=f;
        self.x6*=f;
        self.x7*=f;
        self.x8*=f;
        self.x9*=f;
    }
}

pub trait MatrixOperations{

    fn get_determinant(&self)->f64;

    fn set_inverse(&mut self,m:&Mat3);

    fn invert(&mut self);

    fn get_inverse(&self)->Mat3;

    fn get_transpose(&self)->Mat3;

    fn set_transpose(&mut self,m:&Mat3);

    fn transpose(&mut self);

    fn set_orientation(&mut self,q:&Quaternion);

}

impl MatrixOperations for Mat3{

    fn get_determinant(&self)->f64{
        self.x1*self.x5*self.x9+
        self.x2*self.x6*self.x5+
        self.x3*self.x4*self.x8-
        self.x7*self.x5*self.x3-
        self.x8*self.x6*self.x1-
        self.x9*self.x4*self.x2
    }

    fn get_inverse(&self)->Mat3{
        let det = self.get_determinant();
        Mat3{
            x1:self.x5*self.x9-self.x8*self.x6,
            x2:self.x2*self.x9-self.x8*self.x3,
            x3:self.x2*self.x6-self.x5*self.x3,
            x4:self.x4*self.x9-self.x7*self.x6,
            x5:self.x1*self.x9-self.x7*self.x3,
            x6:self.x1*self.x6-self.x4*self.x3,
            x7:self.x4*self.x8-self.x7*self.x5,
            x8:self.x1*self.x8-self.x7*self.x2,
            x9:self.x1*self.x5-self.x4*self.x2,
        }/det
    }

    fn set_inverse(&mut self,m:&Mat3){
        *self=m.get_inverse();
    }

    fn invert(&mut self){
        *self=self.get_inverse();
    }

    fn get_transpose(&self)->Mat3{
        Mat3{
            x1:self.x1,
            x2:self.x4,
            x3:self.x7,
            x4:self.x2,
            x5:self.x5,
            x6:self.x8,
            x7:self.x3,
            x8:self.x6,
            x9:self.x9,
        
        }
    }

    fn set_transpose(&mut self,m: &Mat3){
        *self=m.get_transpose();
    }

    fn transpose(&mut self){
        *self=self.get_transpose();
    }

    fn set_orientation(&mut self,q: &Quaternion){
        *self=Mat3{
            x1:1.0-2.0*(q.v.y*q.v.y+q.v.z*q.v.z),
            x2:2.0*(q.v.x*q.v.y+q.v.z*q.s),
            x3:2.0*(q.v.x*q.v.z-q.v.y*q.s),
            x4:2.0*(q.v.x*q.v.z-q.v.z*q.v.z),
            x5:1.0-2.0*(q.v.x*q.v.x+q.v.z*q.v.z),
            x6:2.0*(q.v.y*q.v.z+q.v.x*q.s),
            x7:2.0*(q.v.x*q.v.z+q.v.y*q.s),
            x8:2.0*(q.v.y*q.v.z-q.v.x*q.s),
            x9:1.0-2.0*(q.v.x*q.v.x+q.v.y*q.v.y),
        }
    }

    
}
pub fn zero_matrix3()->Mat3{
    Mat3{x1:0.0,x2:0.0,x3:0.0,x4:0.0,x5:0.0,x6:0.0,x7:0.0,x8:0.0,x9:0.0,}
}

pub fn identity_matrix3()->Mat3{
    Mat3{x1:1.0,x2:0.0,x3:0.0,x4:0.0,x5:1.0,x6:0.0,x7:0.0,x8:0.0,x9:1.0,}
}


