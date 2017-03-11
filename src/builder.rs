use std::marker::PhantomData;

use itertools;
use itertools::structs::Zip;

use attributes::*;
use generators::GeometryGenerator;


pub struct GeometryBuilder<G, HasNormals=No, HasUVs=No>
    where HasNormals: YesNo, HasUVs: YesNo
{
    pub generator: G,
    _n: PhantomData<HasNormals>,
    _u: PhantomData<HasUVs>
}


impl<G: GeometryGenerator> GeometryBuilder<G, No, No>
{
    pub fn new(generator: G) -> Self
    {
        GeometryBuilder
        {
            generator: generator,
            _n: PhantomData,
            _u: PhantomData
        }
    }
}


impl<G: GeometryGenerator, HasNormals: Default, HasUVs: Default> GeometryBuilder<G, HasNormals, HasUVs>
    where HasNormals: YesNo, HasUVs: YesNo
{
    pub fn with_normals(self) -> GeometryBuilder<G, Yes, HasUVs>
    {
        GeometryBuilder
        {
            generator: self.generator,
            _n: PhantomData,
            _u: PhantomData
        }
    }

    pub fn with_uvs(self) -> GeometryBuilder<G, HasNormals, Yes>
    {
        GeometryBuilder
        {
            generator: self.generator,
            _n: PhantomData,
            _u: PhantomData
        }
    }
}


impl<G: GeometryGenerator> IntoIterator for GeometryBuilder<G, No, No>
{
    type Item = (Position,);
    type IntoIter = Zip<(G::PositionIter,)>;

    fn into_iter(self) -> Self::IntoIter
    {
        itertools::multizip((self.generator.positions(),))
    }
}


impl<G: GeometryGenerator> IntoIterator for GeometryBuilder<G, Yes, No>
{
    type Item = (Position, Normal);
    type IntoIter = Zip<(G::PositionIter, G::NormalIter)>;

    fn into_iter(self) -> Self::IntoIter
    {
        itertools::multizip((self.generator.positions(), self.generator.normals()))
    }
}


impl<G: GeometryGenerator> IntoIterator for GeometryBuilder<G, No, Yes>
{
    type Item = (Position, UV);
    type IntoIter = Zip<(G::PositionIter, G::UVIter)>;

    fn into_iter(self) -> Self::IntoIter
    {
        itertools::multizip((self.generator.positions(), self.generator.uvs()))
    }
}


impl<G: GeometryGenerator> IntoIterator for GeometryBuilder<G, Yes, Yes>
{
    type Item = (Position, Normal, UV);
    type IntoIter = Zip<(G::PositionIter, G::NormalIter, G::UVIter)>;

    fn into_iter(self) -> Self::IntoIter
    {
        itertools::multizip((self.generator.positions(), self.generator.normals(), self.generator.uvs()))
    }
}


#[cfg(test)]
mod tests
{
    use super::*;

    use std::vec::IntoIter;


    pub struct TestGeometryGenerator;

    impl GeometryGenerator for TestGeometryGenerator
    {
        type PositionIter = IntoIter<Position>;
        type NormalIter = IntoIter<Normal>;
        type UVIter = IntoIter<UV>;

        fn positions(&self) -> Self::PositionIter
        {
            vec![[0.0, 0.0, 0.0]].into_iter()
        }

        fn normals(&self) -> Self::NormalIter
        {
            vec![[0.0, 0.0, 0.0]].into_iter()
        }

        fn uvs(&self) -> Self::UVIter
        {
            vec![[0.0, 0.0]].into_iter()
        }
    }


    #[test]
    fn build_positions()
    {
        let vertices: Vec<(Position,)> = GeometryBuilder::new(TestGeometryGenerator)
            .into_iter().collect();
        assert_eq!(vertices, vec![ ([0.0, 0.0, 0.0],) ]);
    }

    #[test]
    fn build_positions_and_normals()
    {
        let vertices: Vec<(Position, Normal)> = GeometryBuilder::new(TestGeometryGenerator)
            .with_normals().into_iter().collect();
        assert_eq!(vertices, vec![ ([0.0, 0.0, 0.0], [0.0, 0.0, 0.0]) ]);
    }

    #[test]
    fn build_positions_and_uvs()
    {
        let vertices: Vec<(Position, UV)> = GeometryBuilder::new(TestGeometryGenerator)
            .with_uvs().into_iter().collect();
        assert_eq!(vertices, vec![ ([0.0, 0.0, 0.0], [0.0, 0.0]) ]);
    }

    #[test]
    fn build_positions_and_normals_and_uvs()
    {
        let vertices: Vec<(Position, Normal, UV)> = GeometryBuilder::new(TestGeometryGenerator)
            .with_normals().with_uvs().into_iter().collect();
        assert_eq!(vertices, vec![ ([0.0, 0.0, 0.0], [0.0, 0.0, 0.0], [0.0, 0.0]) ]);
    }
}