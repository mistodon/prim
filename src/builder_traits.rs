pub trait VertexBuilder
{
    type WithNormals;
    type WithUVs;

    fn with_normals(self) -> Self::WithNormals;
    fn with_uvs(self) -> Self::WithUVs;
}

