use super::*;

pub trait GetScale<T=float,const N : usize=3> where Vector<T,N> : Copy, T : Copy
{
    fn scale(&self) -> Vector<T,N>;
}

pub trait SetScale<T=float,const N : usize=3> : GetScale<T,N> where Vector<T,N> : Copy, T : Copy
{
    fn set_scale(&mut self, scale : Vector<T,N>) -> &mut Self;
}