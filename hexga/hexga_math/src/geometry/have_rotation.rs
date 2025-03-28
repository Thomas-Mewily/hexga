use super::*;

pub trait HaveRotationX<T>
{
    fn rotate_x(&mut self, angle : AngleOf<T>) -> &mut Self;
    fn rot_x(&mut self,  angle : AngleOf<T>) -> &mut Self { self.rotate_x(angle) }
    #[must_use]
    fn rotated_x(mut self, angle : AngleOf<T>) -> Self where Self : Sized { self.rotate_x(angle); self }
}

pub trait HaveRotationY<T>
{
    fn rotate_y(&mut self, angle : AngleOf<T>) -> &mut Self;
    fn rot_y(&mut self,  angle : AngleOf<T>) -> &mut Self { self.rotate_y(angle) }
    #[must_use]
    fn rotated_y(mut self, angle : AngleOf<T>) -> Self where Self : Sized { self.rotate_y(angle); self }
}

pub trait HaveRotationZ<T>
{
    fn rotate_z(&mut self, angle : AngleOf<T>) -> &mut Self;
    fn rot_z(&mut self,  angle : AngleOf<T>) -> &mut Self { self.rotate_z(angle) }
    #[must_use]
    fn rotated_z(mut self, angle : AngleOf<T>) -> Self where Self : Sized { self.rotate_z(angle); self }
}