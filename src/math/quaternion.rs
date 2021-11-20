
use std::ops;
use crate::math::vec3::Vec3;
use crate::math::vec3::Vec3Ops;
#[derive(Copy,Clone,Debug)]
pub struct Quaternion{
    pub v: Vec3,
    pub s: f64
}

impl ops::Add<Quaternion> for Quaternion{
    type Output = Quaternion;
    fn add(self,q:Quaternion)->Quaternion{
        Quaternion{
            v: self.v+q.v,
            s:self.s+q.s}
    }
}

impl ops::Sub<Quaternion> for Quaternion{
    type Output = Quaternion;
    fn sub(self,q:Quaternion)->Quaternion{
        Quaternion{
            v: self.v-q.v,
            s:self.s-q.s}
    }
}

impl ops::Mul<Quaternion> for Quaternion{
    type Output = Quaternion;
    fn mul(self,q:Quaternion)->Quaternion{
        Quaternion{
            v:self.v*q.s+q.v*self.s+self.v.cross(&q.v),
            s:self.s*q.s-self.v*q.v}
    }
}

impl ops::Mul<f64> for Quaternion{
    type Output = Quaternion;
    fn mul(self,f:f64)->Quaternion{
        Quaternion{v:self.v*f,s:self.s*f}
    }
}

impl ops::Div<f64> for Quaternion{
    type Output = Quaternion;
    fn div(self,f:f64)->Quaternion{
        self*(1.0/f)
    }
}

impl ops::AddAssign<Quaternion> for Quaternion{
    fn add_assign(&mut self,q:Quaternion){
        self.v+=q.v;
        self.s+=q.s;
    }
}

impl ops::SubAssign<Quaternion> for Quaternion{
    fn sub_assign(&mut self,q:Quaternion){
        self.v-=q.v;
        self.s-=q.s;
    }
}

impl ops::MulAssign<Quaternion> for Quaternion{
    fn mul_assign(&mut self,q:Quaternion){
        *self=*self*q;
    }
}

impl ops::MulAssign<f64> for Quaternion{
    fn mul_assign(&mut self,f:f64){
        *self=*self*f;
    }
}

impl ops::DivAssign<f64> for Quaternion{
    fn div_assign(&mut self,f:f64){
        *self*=1.0/f;
    }
}

impl Quaternion{
    pub fn length(&self)->f64{
        (self.v*self.v+self.s*self.s).sqrt()
    }

    pub fn normalize(&mut self){
        *self/=self.length();
    }

    pub fn conjugate(&self)->Quaternion{
        Quaternion{v:self.v*(-1.0),s:self.s}
    }

    pub fn _conjugate(&mut self){
        self.v=self.v*(-1.0);
    }

    pub fn inverse(&self)->Quaternion{
        let norm=self.length();
        self.conjugate()/(norm*norm)
    }

    pub fn invert(&mut self){
        let norm=self.length();
        if norm !=0.0{
            self._conjugate();
            *self/=norm*norm;
        }
        
    }

    pub fn rotate_by_vector(&mut self,v:&Vec3){
        let q = Quaternion{v:*v,s:0.0};
        *self*=q;
    }
}