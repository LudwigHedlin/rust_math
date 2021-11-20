
use crate::math::mat3::Mat3;

pub(crate) use std::ops;
#[derive(Copy, Clone,Debug)]
pub struct Vec3{
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl ops::Add<Vec3> for Vec3{
    type Output = Vec3;

    fn add(self, v:Vec3) -> Vec3 {
        Vec3{x:self.x+v.x,y:self.y+v.y,z:self.z+v.z}
    }
}

impl ops::Sub<Vec3> for Vec3{
    type Output = Vec3;

    fn sub(self, v:Vec3) -> Vec3 {
        Vec3{x:self.x-v.x,y:self.y-v.y,z:self.z-v.z}
    }
}

impl ops::AddAssign<Vec3> for Vec3{

    fn add_assign(&mut self,v:Vec3){
        self.x+=v.x;
        self.y+=v.y;
        self.z+=v.z;
    }
}

impl ops::SubAssign<Vec3> for Vec3{

    fn sub_assign(&mut self,v:Vec3){
        self.x-=v.x;
        self.y-=v.y;
        self.z-=v.z;
    }
}

impl ops::Mul<Vec3> for Vec3{
    type Output = f64;
    fn mul(self,v:Vec3)->f64{
        self.x*v.x+self.y*v.y+self.z*v.z
    }
}

impl ops::Mul<f64> for Vec3{
    type Output = Vec3;
    fn mul(self,f:f64)->Vec3{
        Vec3{x:self.x*f,y:self.y*f,z:self.z*f}
    }
}

impl ops::Div<f64> for Vec3{
    type Output = Vec3;
    fn div(self,f:f64)->Vec3{
        Vec3{x:self.x/f,y:self.y/f,z:self.z/f}
    }
}
impl ops::MulAssign<f64> for Vec3{
    
    fn mul_assign(&mut self,f:f64){
        self.x*=f;
        self.y*=f;
        self.z*=f;
    }
}
impl ops::MulAssign<Mat3> for Vec3{
    fn mul_assign(&mut self,m:Mat3){
        *self=m*(*self);
    }
}

impl ops::DivAssign<f64> for Vec3{
    fn div_assign(&mut self,f:f64){    
        *self*=1.0/f;
    }
}

pub trait Vec3Ops{
    fn cross(&self,v:&Vec3)->Vec3;

    fn _cross(&mut self,v:&Vec3);

    fn length(&self)->f64;

    fn normalize(&mut self);
}

impl Vec3Ops for Vec3{
    fn cross(&self,v:&Vec3)->Vec3{
        Vec3{x:v.y*self.z-self.y*v.z,
             y:v.z*self.x-self.z*v.x,
             z:v.x*self.y-self.x*v.y}
        
    }

    fn _cross(&mut self,v:&Vec3){
        *self=self.cross(v);
    }

    fn length(&self)->f64{
        (*self*(*self)).sqrt()
    }

    fn normalize(&mut self){
        let f = self.length();
        if f !=0.0{
            *self/=f;
        }
            
    }
}