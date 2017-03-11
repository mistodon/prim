extern crate itertools;


mod attributes;
mod builder;
pub mod generators;


pub use self::builder::GeometryBuilder;
use generators::quad::QuadGeometryGenerator;


pub fn quad() -> GeometryBuilder<QuadGeometryGenerator>
{
    GeometryBuilder::new(QuadGeometryGenerator { })
}