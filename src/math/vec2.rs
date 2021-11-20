
use std::ops;
#[derive(Copy, Clone,Debug)]
pub struct Vec2{
    pub x: f64,
    pub y: f64,
}

impl ops::Add<Vec2> for Vec2{
    type Output = Vec2;
    fn add(self,v:Vec2)->Vec2{
        Vec2{x:self.x+v.x,y:self.y+v.y} 
    }
}

impl ops::Sub<Vec2> for Vec2{
    type Output = Vec2;
    fn sub(self,v:Vec2)->Vec2{
        Vec2{x:self.x-v.x,y:self.y-v.y}
    }
}

impl ops::Mul<Vec2> for Vec2{
    type Output = f64;
    fn mul(self,v:Vec2)->f64{
        self.x*v.x+self.y*v.y
    }
}

impl ops::Mul<f64> for Vec2{
    type Output = Vec2;
    fn mul(self,f:f64)->Vec2{
        Vec2{x:self.x*f,y:self.y*f}
    }
}

impl ops::Div<f64> for Vec2{
    type Output = Vec2;
    fn div(self,f:f64)->Vec2{
        self*(1.0/f)
    }
}

impl ops::AddAssign<Vec2> for Vec2{
    fn add_assign(&mut self,v:Vec2){
        self.x+=v.x;
        self.y+=v.y;
    }
}

impl ops::SubAssign<Vec2> for Vec2{
    fn sub_assign(&mut self,v:Vec2){
        self.x-=v.x;
        self.y-=v.y;
    }
}

impl ops::MulAssign<f64> for Vec2{
    fn mul_assign(&mut self,f:f64){
        self.x*=f;
        self.y*=f;
    }
}

impl ops::DivAssign<f64> for Vec2{
    fn div_assign(&mut self,f:f64){
        let quotient=1.0/f;
        self.x*=quotient;
        self.y*=quotient;
    }
}



impl Vec2{
    pub fn length(&self)->f64{
        (*self*(*self)).sqrt()
    }

    pub fn normalize(&mut self){
        let f=self.length();
        *self/=f;
    }
}