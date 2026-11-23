/*
use super::*;

pub type DrawError = std::convert::Infallible;
pub type DrawResult<T = ()> = Result<T, DrawError>;

/// A target that accepts 3D geometry for rendering.
pub trait Drawer: BuilderMesh<3>
{
}

impl<T> Drawer for T
where
    T: BuilderMesh<3>,
{

}

/// Something that can submit its geometry to a drawer.
pub trait Drawable
{
    fn draw(&mut self) -> DrawResult
    where
        Self: Sized,
    {
        self.draw_in(&mut Pen)
    }

    fn draw_in<D>(&mut self, drawer: &mut D) -> DrawResult
    where
        D: Drawer;
}




/// A filled circle in the XY plane.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Circle
{
    pub center: Vec3,
    pub radius: float,
    pub color : Color,
    pub segments: usize,
}

impl Circle
{
    pub fn new(center: Vec3, radius: float, color: Color) -> Self
    {
        Self {
            center,
            radius,
            color,
            segments: 32,
        }
    }

    pub fn with_segments(mut self, segments: usize) -> Self
    {
        self
    }
}

impl Drawable for Circle
{
    fn draw_in<D>(&mut self, drawer: &mut D) -> DrawResult
    where
        D: Drawer,
    {
        let segments = std::cmp::max(self.segments, 3);
        let mut vertices = Vec::with_capacity(segments + 1);
        vertices.push(Vertex::new().with_position(self.center).with_color(self.color));

        for i in 0..segments
        {
            let angle = std::f32::consts::TAU * (i as float) / (segments as float);
            vertices.push(
                Vertex::new()
                    .with_position(vec3(self.center.x + self.radius * angle.cos(), self.center.y + self.radius * angle.sin(), self.center.z))
                    .with_color(self.color),
            );
        }

        let mut indices = Vec::with_capacity(segments * 3);
        for i in 0..segments
        {
            indices.extend([0, (i + 1) as VertexIndex, ((i + 1) % segments + 1) as VertexIndex]);
        }

        BuilderMesh::geometry(drawer, &vertices, &indices);
        Ok(())
    }
}

pub trait SetBackgroundColor
{
    fn set_background_color(&mut self, color: Color) -> &mut Self;
}
*/