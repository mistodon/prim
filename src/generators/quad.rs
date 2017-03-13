use std::iter::{self, Repeat};
use std::marker::PhantomData;
use std::vec;

use attributes::*;
use builder_traits::*;
use geometry_generator::GeometryGenerator;
use geometry_builder::{GeometryBuilder, MeshBuilder};


pub struct QuadGeometryGenerator
{

}


impl GeometryGenerator for QuadGeometryGenerator
{
    type PositionIter = vec::IntoIter<Position>;
    type NormalIter = Repeat<Normal>;
    type UVIter = vec::IntoIter<UV>;

    fn positions(&self) -> Self::PositionIter
    {
        vec![
            [-0.5, -0.5, 0.0],
            [0.5, -0.5, 0.0],
            [0.5, 0.5, 0.0],
            [-0.5, -0.5, 0.0],
            [0.5, 0.5, 0.0],
            [-0.5, 0.5, 0.0]].into_iter()
    }

    fn normals(&self) -> Self::NormalIter
    {
        iter::repeat([0.0, 0.0, -0.5])
    }

    fn uvs(&self) -> Self::UVIter
    {
        vec![
            [0.0, 0.0],
            [1.0, 0.0],
            [1.0, 1.0],
            [0.0, 0.0],
            [1.0, 1.0],
            [0.0, 1.0]].into_iter()
    }
}


pub struct QuadBuilder<V>
{
    _v: PhantomData<V>
}

impl<V> VertexBuilder for QuadBuilder<V>
    where V: VertexBuilder
{
    type WithNormals = QuadBuilder<V::WithNormals>;
    type WithUVs = QuadBuilder<V::WithUVs>;

    fn with_normals(self) -> Self::WithNormals
    {
        QuadBuilder {_v: PhantomData }
    }

    fn with_uvs(self) -> Self::WithUVs
    {
        QuadBuilder {_v: PhantomData }
    }
}

impl<V> MeshBuilder<QuadGeometryGenerator, V> for QuadBuilder<V>
{
    fn build(self) -> GeometryBuilder<QuadGeometryGenerator, V>
    {
        GeometryBuilder::new(QuadGeometryGenerator{})
    }
}