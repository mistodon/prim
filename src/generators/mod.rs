use attributes::*;


pub mod quad;


pub trait GeometryGenerator
{
    type PositionIter: Iterator<Item=Position>;
    type NormalIter: Iterator<Item=Normal>;
    type UVIter: Iterator<Item=UV>;

    fn positions(&self) -> Self::PositionIter;
    fn normals(&self) -> Self::NormalIter;
    fn uvs(&self) -> Self::UVIter;
}