use std::marker::PhantomData;

use itertools;
use itertools::structs::Zip;

use attributes::*;
use geometry_generator::GeometryGenerator;
use vertex_format::{VertexFormat, Yes, No};


pub trait MeshBuilder<G, V>
{
    fn build(self) -> GeometryBuilder<G, V>;
}


pub struct GeometryBuilder<G, V>
{
    pub generator: G,
    _v: PhantomData<V>
}


impl<G: GeometryGenerator, V> GeometryBuilder<G, V>
{
    pub fn new(generator: G) -> Self
    {
        GeometryBuilder
        {
            generator: generator,
            _v: PhantomData
        }
    }
}


impl<G: GeometryGenerator> IntoIterator for GeometryBuilder<G, VertexFormat<No, No>>
{
    type Item = (Position,);
    type IntoIter = Zip<(G::PositionIter,)>;

    fn into_iter(self) -> Self::IntoIter
    {
        itertools::multizip((self.generator.positions(),))
    }
}


impl<G: GeometryGenerator> IntoIterator for GeometryBuilder<G, VertexFormat<Yes, No>>
{
    type Item = (Position, Normal);
    type IntoIter = Zip<(G::PositionIter, G::NormalIter)>;

    fn into_iter(self) -> Self::IntoIter
    {
        itertools::multizip((self.generator.positions(), self.generator.normals()))
    }
}


impl<G: GeometryGenerator> IntoIterator for GeometryBuilder<G, VertexFormat<No, Yes>>
{
    type Item = (Position, UV);
    type IntoIter = Zip<(G::PositionIter, G::UVIter)>;

    fn into_iter(self) -> Self::IntoIter
    {
        itertools::multizip((self.generator.positions(), self.generator.uvs()))
    }
}


impl<G: GeometryGenerator> IntoIterator for GeometryBuilder<G, VertexFormat<Yes, Yes>>
{
    type Item = (Position, Normal, UV);
    type IntoIter = Zip<(G::PositionIter, G::NormalIter, G::UVIter)>;

    fn into_iter(self) -> Self::IntoIter
    {
        itertools::multizip((self.generator.positions(), self.generator.normals(), self.generator.uvs()))
    }
}
