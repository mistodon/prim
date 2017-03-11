use std::iter::{self, Repeat};
use std::vec;

use attributes::*;
use generators::GeometryGenerator;


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