use super::*;

impl<S, T, const N : usize> ToDimension<T,N> for S where S : ArrayWithSize<T,N> {}
pub trait ToDimension<T, const N : usize> : ArrayWithSize<T,N>
{
    fn to_1d(self) -> Self::WithSize<1> where T: Default { self.resize::<1>() }
    fn to_2d(self) -> Self::WithSize<2> where T: Default { self.resize::<2>() }
    fn to_3d(self) -> Self::WithSize<3> where T: Default { self.resize::<3>() }
    fn to_4d(self) -> Self::WithSize<4> where T: Default { self.resize::<4>() }
}

// TODO: fix it
/* 
impl<S, T, const N : usize> ArrayExtension<T,N> for S where S : Array<T,N> {}
pub trait ArrayExtension<T, const N : usize> : Array<T,N>
{
    /// Fill non existing component with the [Clone]d given value
    fn to_point_with_size_filled_with_value<const M: usize>(self, value: int) -> Point<M> where T: ToInt { Point::from_array(self.to_array().map(|v| v.to_int()).to_array_with_size_filled_with_value(value)) }
    /// Fill non existing component with [Default]
    fn to_point_with_size<const M: usize>(self) -> Point<M> where T: ToInt + Default { Point::from_array(self.to_array().map(|v| v.to_int()).to_array_with_size()) }
    /// Cast the value to int and return a Point
    fn to_point(self) -> Point<N> where T: ToInt { Point::from_array(self.to_array().map(|v| v.to_int())) }

    /// Fill non existing component with [Default]
    fn to_point1(self) -> Point1 where T: ToInt + Default { self.to_point_with_size() }
    /// Fill non existing component with [Default]
    fn to_point2(self) -> Point2 where T: ToInt + Default { self.to_point_with_size() }
    /// Fill non existing component with [Default]
    fn to_point3(self) -> Point3 where T: ToInt + Default { self.to_point_with_size() }
    /// Fill non existing component with [Default]
    fn to_point4(self) -> Point4 where T: ToInt + Default { self.to_point_with_size() }


    /// Fill non existing component with the [Clone]d given value
    fn to_vector_with_size_filled_with_value<const M: usize>(self, value: float) -> Vector<float,M> where T: ToFloat { Vector::from_array(self.to_array().map(|v| v.to_float()).to_array_with_size_filled_with_value(value)) }
    /// Fill non existing component with [Default]
    fn to_vector_with_size<const M: usize>(self) -> Vector<float,M> where T: ToFloat + Default { Vector::from_array(self.to_array().map(|v| v.to_float()).to_array_with_size()) }
    /// Cast the value to float and return a Vector
    fn to_vector(self) -> Vector<float,N> where T: ToFloat { Vector::from_array(self.to_array().map(|v| v.to_float())) }


    /// Fill non existing component with [Default]
    fn to_vec1(self) -> Vec1 where T: ToFloat + Default { self.to_vector_with_size() }
    /// Fill non existing component with [Default]
    fn to_vec2(self) -> Vec2 where T: ToFloat + Default { self.to_vector_with_size() }
    /// Fill non existing component with [Default]
    fn to_vec3(self) -> Vec3 where T: ToFloat + Default { self.to_vector_with_size() }
    /// Fill non existing component with [Default]
    fn to_vec4(self) -> Vec4 where T: ToFloat + Default { self.to_vector_with_size() }
}
*/

/*
pub trait ArrayExtension1D<T> : Array<T,1>
{
    fn to_array_2d_filled(self, fill : T) where T: Copy;
    fn to_array_3d_filled(self, fill : T) where T: Copy;
}
    */



/*
pub trait ToVectorFilled<T, const N : usize> : Sized where T: Copy
{
    fn to_vector_filled(self, fill : T) -> Vector<T,N>;
}

impl<T> ToVectorFilled<T, 1> for Vector<T, 1>  where T: Copy { fn to_vector_filled(self, fill : T) -> Vector<T,1> { self.to_vector1_filled(fill) } }

impl<T> ToVectorFilled<T, 2> for Vector<T, 1>  where T: Copy { fn to_vector_filled(self, fill : T) -> Vector<T,2> { self.to_vector2_filled(fill) } }
impl<T> ToVectorFilled<T, 2> for Vector<T, 2>  where T: Copy { fn to_vector_filled(self, fill : T) -> Vector<T,2> { self.to_vector2_filled(fill) } }

impl<T> ToVectorFilled<T, 3> for Vector<T, 1>  where T: Copy { fn to_vector_filled(self, fill : T) -> Vector<T,3> { self.to_vector3_filled(fill) } }
impl<T> ToVectorFilled<T, 3> for Vector<T, 2>  where T: Copy { fn to_vector_filled(self, fill : T) -> Vector<T,3> { self.to_vector3_filled(fill) } }
impl<T> ToVectorFilled<T, 3> for Vector<T, 3>  where T: Copy { fn to_vector_filled(self, fill : T) -> Vector<T,3> { self.to_vector3_filled(fill) } }

impl<T> ToVectorFilled<T, 4> for Vector<T, 1>  where T: Copy { fn to_vector_filled(self, fill : T) -> Vector<T,4> { self.to_vector4_filled(fill) } }
impl<T> ToVectorFilled<T, 4> for Vector<T, 2>  where T: Copy { fn to_vector_filled(self, fill : T) -> Vector<T,4> { self.to_vector4_filled(fill) } }
impl<T> ToVectorFilled<T, 4> for Vector<T, 3>  where T: Copy { fn to_vector_filled(self, fill : T) -> Vector<T,4> { self.to_vector4_filled(fill) } }
impl<T> ToVectorFilled<T, 4> for Vector<T, 4>  where T: Copy { fn to_vector_filled(self, fill : T) -> Vector<T,4> { self.to_vector4_filled(fill) } }

impl<T, const N : usize> ToVectorFilled<T, N> for T  where T: Copy { fn to_vector_filled(self, _ : T) -> Vector<T,N> { Vector::splat(self) } }
*/