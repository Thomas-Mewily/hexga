use super::*;

pub trait GetScale<T,const N : usize> where Vector<T,N> : Copy, T : Copy
{
    fn scale(&self) -> Vector<T,N>;
}

pub trait SetScale<T,const N : usize> : GetScale<T,N> where Vector<T,N> : Copy, T : Copy
{
    fn set_scale(&mut self, scale : Vector<T,N>) -> &mut Self;
}