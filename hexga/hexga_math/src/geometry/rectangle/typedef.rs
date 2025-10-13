use super::*;

pub type Rectangle1<T> = Rectangle<T, 1>;
pub const fn rectangle1<T>(pos_x: T, size_x: T) -> Rectangle1<T> { Rectangle1::new(Vector1::new(pos_x), Vector1::new(size_x)) }
pub type Rect1 = Rectangle1<float>;
pub const fn rect1(pos_x: float, size_x: float) -> Rect1 { rectangle1(pos_x, size_x) }
pub type Rect1i = Rectangle1<int>;
pub const fn rect1i(pos_x: int, size_x: int) -> Rect1i { rectangle1(pos_x, size_x) }

pub type Rectangle2<T> = Rectangle<T, 2>;
pub const fn rectangle2<T>(pos_x: T, pos_y: T, size_x: T, size_y: T) -> Rectangle2<T> { Rectangle2::new(Vector2::new(pos_x, pos_y), Vector2::new(size_x, size_y)) }
pub type Rect2 = Rectangle2<float>;
pub const fn rect2(pos_x: float, pos_y: float, size_x: float, size_y: float) -> Rect2 { rectangle2(pos_x, pos_y, size_x, size_y) }
pub type Rect2i = Rectangle2<int>;
pub const fn rect2i(pos_x: int, pos_y: int, size_x: int, size_y: int) -> Rect2i { rectangle2(pos_x, pos_y, size_x, size_y) }

pub type Rectangle3<T> = Rectangle<T, 3>;
pub const fn rectangle3<T>(pos_x: T, pos_y: T, pos_z: T, size_x: T, size_y: T, size_z: T) -> Rectangle3<T> { Rectangle3::new(Vector3::new(pos_x, pos_y, pos_z), Vector3::new(size_x, size_y, size_z)) }
pub type Rect3 = Rectangle3<float>;
pub const fn rect3(pos_x: float, pos_y: float, pos_z: float, size_x: float, size_y: float, size_z: float) -> Rect3 { rectangle3(pos_x, pos_y, pos_z, size_x, size_y, size_z) }
pub type Rect3i = Rectangle3<int>;
pub const fn rect3i(pos_x: int, pos_y: int, pos_z: int, size_x: int, size_y: int, size_z: int) -> Rect3i { rectangle3(pos_x, pos_y, pos_z, size_x, size_y, size_z) }

pub type Rectangle4<T> = Rectangle<T, 4>;
pub const fn rectangle4<T>(pos_x: T, pos_y: T, pos_z: T, pos_w: T, size_x: T, size_y: T, size_z: T, size_w: T) -> Rectangle4<T> { Rectangle4::new(Vector4::new(pos_x, pos_y, pos_z, pos_w), Vector4::new(size_x, size_y, size_z, size_w)) }
pub type Rect4 = Rectangle4<float>;
pub const fn rect4(pos_x: float, pos_y: float, pos_z: float, pos_w: float, size_x: float, size_y: float, size_z: float, size_w: float) -> Rect4 { rectangle4(pos_x, pos_y, pos_z, pos_w, size_x, size_y, size_z, size_w) }
pub type Rect4i = Rectangle4<int>;
pub const fn rect4i(pos_x: int, pos_y: int, pos_z: int, pos_w: int, size_x: int, size_y: int, size_z: int, size_w: int) -> Rect4i { rectangle4(pos_x, pos_y, pos_z, pos_w, size_x, size_y, size_z, size_w) }

