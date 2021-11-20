use std::ops;
use crate::math::vec2::Vec2;

#[derive(Copy, Clone,Debug)]
struct Mat2{
    pub x1:f64,
    pub x2:f64,
    pub x3:f64,
    pub x4:f64,
}

impl ops::Add<Mat2> for Mat2{
    type Output = Mat2;
    fn add(self,m:Mat2)->Mat2{
        Mat2{
            x1:self.x1+m.x1,
            x2:self.x2+m.x2,
            x3:self.x3+m.x3,
            x4:self.x4+m.x4,
        }
    }
}

impl ops::Sub<Mat2> for Mat2{
    type Output = Mat2;
    fn sub(self,m:Mat2)->Mat2{
        Mat2{
            x1:self.x1-m.x1,
            x2:self.x2-m.x2,
            x3:self.x3-m.x3,
            x4:self.x4-m.x4,
        }
    }
}

impl ops::Mul<Mat2> for Mat2{
    type Output = Mat2;
    fn mul(self,m:Mat2)->Mat2{
        Mat2{
            x1:self.x1*m.x1+self.x2*m.x3,
            x2:self.x1*m.x2+self.x2*m.x4,
            x3:self.x3*m.x1+self.x4*m.x2,
            x4:self.x3*m.x2+self.x4*m.x4,
        }
    }
}

impl ops::Mul<Vec2> for Mat2{
    type Output = Vec2;
    fn mul(self,v:Vec2)->Vec2{
        Vec2{
            x:v.x*self.x1+v.y*self.x2,
            y:v.x*self.x3+v.x*self.x4 
        }
    }
}

impl ops::Mul<f64> for Mat2{
    type Output = Mat2;
    fn mul(self,f:f64)->Mat2{
        Mat2{
            x1:self.x1*f,
            x2:self.x1*f,
            x3:self.x3*f,
            x4:self.x3*f,
        }
    }
}

impl ops::Div<f64> for Mat2{
    type Output = Mat2;
    fn div(self,f:f64)->Mat2{
        self*(1.0/f)
    }
}

impl ops::AddAssign<Mat2> for Mat2{
    fn add_assign(&mut self,m:Mat2){
        self.x1+=m.x1;
        self.x2+=m.x2;
        self.x3+=m.x3;
        self.x4+=m.x4;
    }
}

impl ops::SubAssign<Mat2> for Mat2{
    fn sub_assign(&mut self,m:Mat2){
        self.x1-=m.x1;
        self.x2-=m.x2;
        self.x3-=m.x3;
        self.x4-=m.x4;
    }
}

impl ops::MulAssign<f64> for Mat2{
    fn mul_assign(&mut self,f:f64){
        self.x1*=f;
        self.x2*=f;
        self.x3*=f;
        self.x4*=f;
    }
}

impl ops::MulAssign<Mat2> for Mat2{
    fn mul_assign(&mut self,m:Mat2){
        *self=*self*m
    }
}

impl Mat2{
    pub fn get_determinant(&self)->f64{
        self.x1*self.x4-self.x2*self.x3
    }

    pub fn get_inverse(&self)->Mat2{
        let det=self.get_determinant();
        Mat2{x1:self.x1,x2:self.x3,x3:self.x2,x4:self.x4}/det
    }

    pub fn set_inverse(&mut self,m:&Mat2){
        *self=m.get_inverse();
    }

    pub fn invert(&mut self){
        *self=self.get_inverse();
    }

    pub fn get_transpose(&self)->Mat2{
        Mat2{x1:self.x1,x2:self.x3,x3:self.x2,x4:self.x4}
    }

    pub fn set_transpose(&mut self,m:&Mat2){
        *self=m.get_transpose();
    }

    pub fn transpose(&mut self){
        *self=self.get_transpose();
    }
}