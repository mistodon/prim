use std::marker::PhantomData;

use builder_traits::VertexBuilder;


pub trait YesNo {}

#[derive(Default)]
pub struct Yes;

#[derive(Default)]
pub struct No;

impl YesNo for Yes {}
impl YesNo for No {}


#[derive(Default)]
pub struct VertexFormat<N, U>
    where N: YesNo + Default, U: YesNo + Default
{
    _n: PhantomData<N>,
    _u: PhantomData<U>
}

impl<N, U> VertexFormat<N, U>
    where N: YesNo + Default, U: YesNo + Default
{
    pub fn new() -> Self
    {
        VertexFormat::default()
    }
}

impl<N, U> VertexBuilder for VertexFormat<N, U>
    where N: YesNo + Default, U: YesNo + Default
{
    type WithNormals = VertexFormat<Yes, U>;
    type WithUVs = VertexFormat<N, Yes>;

    fn with_normals(self) -> Self::WithNormals
    {
        Self::WithNormals::new()
    }

    fn with_uvs(self) -> Self::WithUVs
    {
        Self::WithUVs::new()
    }
}