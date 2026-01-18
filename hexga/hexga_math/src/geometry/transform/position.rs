use super::*;

pub trait GetPosition<T=float,const N : usize=3> where Vector<T,N> : Copy, T : Copy
{
    fn pos(&self) -> Vector<T,N>;

    #[track_caller]
    fn x(&self) -> T where Vector<T,N> : HaveX<T> { *self.pos().get_or_panic(Vector::<T,N>::X_INDEX) }
    #[track_caller]
    fn y(&self) -> T where Vector<T,N> : HaveY<T> { *self.pos().get_or_panic(Vector::<T,N>::Y_INDEX) }
    #[track_caller]
    fn z(&self) -> T where Vector<T,N> : HaveZ<T> { *self.pos().get_or_panic(Vector::<T,N>::Z_INDEX) }
    #[track_caller]
    fn w(&self) -> T where Vector<T,N> : HaveW<T> { *self.pos().get_or_panic(Vector::<T,N>::W_INDEX) }
}


pub trait SetPosition<T=float,const N : usize=3> where Vector<T,N> : Copy, T : Copy
{
    fn set_pos(&mut self, pos : Vector<T,N>) -> &mut Self;

    fn set_x(&mut self, x : T) -> &mut Self where Vector<T,N> : HaveX<T>, Self: GetPosition<T,N> { let mut p = self.pos(); p.set(Vector::<T,N>::X_INDEX, x); self.set_pos(p); self }
    fn set_y(&mut self, y : T) -> &mut Self where Vector<T,N> : HaveY<T>, Self: GetPosition<T,N> { let mut p = self.pos(); p.set(Vector::<T,N>::Y_INDEX, y); self.set_pos(p); self }
    fn set_z(&mut self, z : T) -> &mut Self where Vector<T,N> : HaveZ<T>, Self: GetPosition<T,N> { let mut p = self.pos(); p.set(Vector::<T,N>::Z_INDEX, z); self.set_pos(p); self }
    fn set_w(&mut self, w : T) -> &mut Self where Vector<T,N> : HaveW<T>, Self: GetPosition<T,N> { let mut p = self.pos(); p.set(Vector::<T,N>::W_INDEX, w); self.set_pos(p); self }

    fn with_pos(mut self, pos : Vector<T,N>) -> Self where Self: Sized + GetPosition<T,N> + GetPosition<T,N> { self.set_pos(pos); self }
    fn with_x(mut self, x : T) -> Self where Vector<T,N> : HaveX<T>, Self: Sized + GetPosition<T,N>  { self.set_x(x); self }
    fn with_y(mut self, y : T) -> Self where Vector<T,N> : HaveY<T>, Self: Sized + GetPosition<T,N>  { self.set_y(y); self }
    fn with_z(mut self, z : T) -> Self where Vector<T,N> : HaveZ<T>, Self: Sized + GetPosition<T,N>  { self.set_z(z); self }
    fn with_w(mut self, w : T) -> Self where Vector<T,N> : HaveW<T>, Self: Sized + GetPosition<T,N>  { self.set_w(w); self }




    fn move_by    (&mut self, v : impl Into<Vector<T,N>>) -> &mut Self where Vector<T,N> : Add<Vector<T,N>,Output = Vector<T,N>>, Self: GetPosition<T,N> { self.set_pos(self.pos() + v.into()) }
    fn move_neg_by(&mut self, v : impl Into<Vector<T,N>>) -> &mut Self where Vector<T,N> : Sub<Vector<T,N>,Output = Vector<T,N>>, Self: GetPosition<T,N> { self.set_pos(self.pos() - v.into()) }

    fn move_x(&mut self, x : T) -> &mut Self where Vector<T,N> : HaveX<T> + Zero + Add<Vector<T,N>,Output = Vector<T,N>>, Self: GetPosition<T,N> { self.move_by(Vector::<T,N>::ZERO.with_x(x)) }
    fn move_y(&mut self, y : T) -> &mut Self where Vector<T,N> : HaveY<T> + Zero + Add<Vector<T,N>,Output = Vector<T,N>>, Self: GetPosition<T,N> { self.move_by(Vector::<T,N>::ZERO.with_y(y)) }
    fn move_z(&mut self, z : T) -> &mut Self where Vector<T,N> : HaveZ<T> + Zero + Add<Vector<T,N>,Output = Vector<T,N>>, Self: GetPosition<T,N> { self.move_by(Vector::<T,N>::ZERO.with_z(z)) }
    fn move_w(&mut self, w : T) -> &mut Self where Vector<T,N> : HaveW<T> + Zero + Add<Vector<T,N>,Output = Vector<T,N>>, Self: GetPosition<T,N> { self.move_by(Vector::<T,N>::ZERO.with_w(w)) }

    fn move_neg_x(&mut self, x : T) -> &mut Self where Vector<T,N> : HaveX<T> + Zero + Add<Vector<T,N>,Output = Vector<T,N>>, T : Neg<Output = T>, Self: GetPosition<T,N> { self.move_x(-x) }
    fn move_neg_y(&mut self, y : T) -> &mut Self where Vector<T,N> : HaveY<T> + Zero + Add<Vector<T,N>,Output = Vector<T,N>>, T : Neg<Output = T>, Self: GetPosition<T,N> { self.move_y(-y) }
    fn move_neg_z(&mut self, z : T) -> &mut Self where Vector<T,N> : HaveZ<T> + Zero + Add<Vector<T,N>,Output = Vector<T,N>>, T : Neg<Output = T>, Self: GetPosition<T,N> { self.move_z(-z) }
    fn move_neg_w(&mut self, w : T) -> &mut Self where Vector<T,N> : HaveW<T> + Zero + Add<Vector<T,N>,Output = Vector<T,N>>, T : Neg<Output = T>, Self: GetPosition<T,N> { self.move_w(-w) }



    fn moved_by    (mut self, v : impl Into<Vector<T,N>>) -> Self where Vector<T,N> : Add<Vector<T,N>,Output = Vector<T,N>>, Self: Sized + GetPosition<T,N> { self.move_by(v); self }
    fn moved_neg_by(mut self, v : impl Into<Vector<T,N>>) -> Self where Vector<T,N> : Sub<Vector<T,N>,Output = Vector<T,N>>, Self: Sized + GetPosition<T,N> { self.move_neg_by(v); self }

    fn moved_x(mut self, x : T) -> Self where Vector<T,N> : HaveX<T> + Zero + Add<Vector<T,N>,Output = Vector<T,N>>, Self: Sized + GetPosition<T,N>  { self.move_x(x); self }
    fn moved_y(mut self, y : T) -> Self where Vector<T,N> : HaveY<T> + Zero + Add<Vector<T,N>,Output = Vector<T,N>>, Self: Sized + GetPosition<T,N>  { self.move_y(y); self }
    fn moved_z(mut self, z : T) -> Self where Vector<T,N> : HaveZ<T> + Zero + Add<Vector<T,N>,Output = Vector<T,N>>, Self: Sized + GetPosition<T,N>  { self.move_z(z); self }
    fn moved_w(mut self, w : T) -> Self where Vector<T,N> : HaveW<T> + Zero + Add<Vector<T,N>,Output = Vector<T,N>>, Self: Sized + GetPosition<T,N>  { self.move_w(w); self }

    fn moved_neg_x(mut self, x : T) -> Self where Vector<T,N> : HaveX<T> + Zero + Add<Vector<T,N>,Output = Vector<T,N>>, T : Neg<Output = T>, Self: Sized + GetPosition<T,N> { self.move_neg_x(x); self }
    fn moved_neg_y(mut self, y : T) -> Self where Vector<T,N> : HaveY<T> + Zero + Add<Vector<T,N>,Output = Vector<T,N>>, T : Neg<Output = T>, Self: Sized + GetPosition<T,N> { self.move_neg_y(y); self }
    fn moved_neg_z(mut self, z : T) -> Self where Vector<T,N> : HaveZ<T> + Zero + Add<Vector<T,N>,Output = Vector<T,N>>, T : Neg<Output = T>, Self: Sized + GetPosition<T,N> { self.move_neg_z(z); self }
    fn moved_neg_w(mut self, w : T) -> Self where Vector<T,N> : HaveW<T> + Zero + Add<Vector<T,N>,Output = Vector<T,N>>, T : Neg<Output = T>, Self: Sized + GetPosition<T,N> { self.move_neg_w(w); self }
}