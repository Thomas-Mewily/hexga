use crate::*;


impl<S, T, const N : usize> ArrayLikeExtension<T,N> for S where S : ArrayLike<T,N> {}
pub trait ArrayLikeExtension<T, const N : usize> : ArrayLike<T,N>
{
    fn min_element(&self) -> &T where T: PartialOrd { self.array().iter().min_by(|a, b| a.partial_cmp(b).unwrap()).expect("size can't be empty") }
    fn max_element(&self) -> &T where T: PartialOrd { self.array().iter().max_by(|a, b| a.partial_cmp(b).unwrap()).expect("size can't be empty") }

    fn min_element_idx(&self) -> usize where T: PartialOrd
    {
        self.array().iter().enumerate()
             .min_by(|(_,a), (_,b)| a.partial_cmp(b).unwrap()).map(|(idx,_)| idx).unwrap()
    }
    fn max_element_idx(&self) -> usize where T: PartialOrd
    {
        self.array().iter().enumerate()
             .max_by(|(_,a), (_,b)| a.partial_cmp(b).unwrap()).map(|(idx,_)| idx).unwrap()
    }

    /// Fill non existing component with [Default]
    fn into_array1(self) -> [T; 1] where T: Default
    {
        let mut it = <Self as Into<[T;N]>>::into(self).into_iter();
        let _0 = it.next().unwrap_or_default();
        [_0]
    }
    /// Fill non existing component with the [Clone]d given value
    fn into_array1_filled(self, fill : T) -> [T; 1] where T: Clone
    {
        let mut it = <Self as Into<[T;N]>>::into(self).into_iter();
        let _0 = it.next().unwrap_or_else(|| fill.clone());
        [_0]
    }

    /// Fill non existing component with [Default]
    fn into_array2(self) -> [T; 2] where T: Default
    {
        let mut it = <Self as Into<[T;N]>>::into(self).into_iter();
        let _0 = it.next().unwrap_or_default();
        let _1 = it.next().unwrap_or_default();
        [_0, _1]
    }
    /// Fill non existing component with the [Clone]d given value
    fn into_array2_filled(self, fill : T) -> [T; 2] where T: Clone
    {
        let mut it = <Self as Into<[T;N]>>::into(self).into_iter();
        let _0 = it.next().unwrap_or_else(|| fill.clone());
        let _1 = it.next().unwrap_or_else(|| fill.clone());
        [_0, _1]
    }

    /// Fill non existing component with [Default]
    fn into_array3(self) -> [T; 3] where T: Default
    {
        let mut it = <Self as Into<[T;N]>>::into(self).into_iter();
        let _0 = it.next().unwrap_or_default();
        let _1 = it.next().unwrap_or_default();
        let _2 = it.next().unwrap_or_default();
        [_0, _1, _2]
    }
    /// Fill non existing component with the [Clone]d given value
    fn into_array3_filled(self, fill : T) -> [T; 3] where T: Clone
    {
        let mut it = <Self as Into<[T;N]>>::into(self).into_iter();
        let _0 = it.next().unwrap_or_else(|| fill.clone());
        let _1 = it.next().unwrap_or_else(|| fill.clone());
        let _2 = it.next().unwrap_or_else(|| fill.clone());
        [_0, _1, _2]
    }


    /// Fill non existing component with [Default]
    fn into_array4(self) -> [T; 4] where T: Default
    {
        let mut it = <Self as Into<[T;N]>>::into(self).into_iter();
        let _0 = it.next().unwrap_or_default();
        let _1 = it.next().unwrap_or_default();
        let _2 = it.next().unwrap_or_default();
        let _3 = it.next().unwrap_or_default();
        [_0, _1, _2, _3]
    }
    /// Fill non existing component with the [Clone]d given value
    fn into_array4_filled(self, fill : T) -> [T; 4] where T: Clone
    {
        let mut it = <Self as Into<[T;N]>>::into(self).into_iter();
        let _0 = it.next().unwrap_or_else(|| fill.clone());
        let _1 = it.next().unwrap_or_else(|| fill.clone());
        let _2 = it.next().unwrap_or_else(|| fill.clone());
        let _3 = it.next().unwrap_or_else(|| fill.clone());
        [_0, _1, _2, _3]
    }

    /// Fill non existing component with [Default]
    fn to_point1(self) -> Point1 where T: ToInt<Output = int> + Default { self.into_array1().map(|v| v.to_int()).into() }
    /// Fill non existing component with [Default]
    fn to_point2(self) -> Point2 where T: ToInt<Output = int> + Default { self.into_array2().map(|v| v.to_int()).into() }
    /// Fill non existing component with [Default]
    fn to_point3(self) -> Point3 where T: ToInt<Output = int> + Default { self.into_array3().map(|v| v.to_int()).into() }
    /// Fill non existing component with [Default]
    fn to_point4(self) -> Point4 where T: ToInt<Output = int> + Default { self.into_array4().map(|v| v.to_int()).into() }


    /// Fill non existing component with the given value
    fn to_point1_filled(self, fill : int) -> Point1 where T : ToInt<Output = int> { self.map(|v| v.to_int()).into_array1_filled(fill).into() }
    /// Fill non existing component with the given value
    fn to_point2_filled(self, fill : int) -> Point2 where T : ToInt<Output = int> { self.map(|v| v.to_int()).into_array2_filled(fill).into() }
    /// Fill non existing component with the given value
    fn to_point3_filled(self, fill : int) -> Point3 where T : ToInt<Output = int> { self.map(|v| v.to_int()).into_array3_filled(fill).into() }
    /// Fill non existing component with the given value
    fn to_point4_filled(self, fill : int) -> Point4 where T : ToInt<Output = int> { self.map(|v| v.to_int()).into_array4_filled(fill).into() }

    /// Fill non existing component with [Default]
    fn to_vec1(self) -> Vec1 where T: ToFloat<Output = float> + Default { self.into_array1().map(|v| v.to_float()).into() }
    /// Fill non existing component with [Default]
    fn to_vec2(self) -> Vec2 where T: ToFloat<Output = float> + Default { self.into_array2().map(|v| v.to_float()).into() }
    /// Fill non existing component with [Default]
    fn to_vec3(self) -> Vec3 where T: ToFloat<Output = float> + Default { self.into_array3().map(|v| v.to_float()).into() }
    /// Fill non existing component with [Default]
    fn to_vec4(self) -> Vec4 where T: ToFloat<Output = float> + Default { self.into_array4().map(|v| v.to_float()).into() }

    /// Fill non existing component with the given value
    fn to_vec1_filled(self, fill : float) -> Vec1 where T: ToFloat<Output = float> { self.map(|v| v.to_float()).into_array1_filled(fill).into() }
    /// Fill non existing component with the given value
    fn to_vec2_filled(self, fill : float) -> Vec2 where T: ToFloat<Output = float> { self.map(|v| v.to_float()).into_array2_filled(fill).into() }
    /// Fill non existing component with the given value
    fn to_vec3_filled(self, fill : float) -> Vec3 where T: ToFloat<Output = float> { self.map(|v| v.to_float()).into_array3_filled(fill).into() }
    /// Fill non existing component with the given value
    fn to_vec4_filled(self, fill : float) -> Vec4 where T: ToFloat<Output = float> { self.map(|v| v.to_float()).into_array4_filled(fill).into() }

    /// Fill non existing component with [Default]
    fn to_vector1(self) -> Vector1<T> where T: Default { self.into_array1().into() }
    /// Fill non existing component with [Default]
    fn to_vector2(self) -> Vector2<T> where T: Default { self.into_array2().into() }
    /// Fill non existing component with [Default]
    fn to_vector3(self) -> Vector3<T> where T: Default { self.into_array3().into() }
    /// Fill non existing component with [Default]
    fn to_vector4(self) -> Vector4<T> where T: Default { self.into_array4().into() }

    /// Fill non existing component with the given value
    fn to_vector1_filled(self, fill : T) -> Vector1<T> where T: Clone { self.into_array1_filled(fill).into() }
    /// Fill non existing component with the given value
    fn to_vector2_filled(self, fill : T) -> Vector2<T> where T: Clone { self.into_array2_filled(fill).into() }
    /// Fill non existing component with the given value
    fn to_vector3_filled(self, fill : T) -> Vector3<T> where T: Clone { self.into_array3_filled(fill).into() }
    /// Fill non existing component with the given value
    fn to_vector4_filled(self, fill : T) -> Vector4<T> where T: Clone { self.into_array4_filled(fill).into() }
}

/*
pub trait ArrayLikeExtension1D<T> : ArrayLike<T,1>
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