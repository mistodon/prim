API
===

(Mostly non-existing at the moment.)

```rust
// Choose attributes/properties with builder functions
let vertices: Vec<(Position, Normal, UV)> = shape()
    .with_normals()
    .with_uvs()
    .flat_shaded()
    .into_iter()
    .collect();
```

```rust
// Can generate indexed meshes
// Vertex is None if the index has already been emitted
let vertices: Vec<(Index, Option<(Position, Normal)>)> = shape()
    .with_normals()
    .with_indices()
    .into_iter()
    .collect();
```

```rust
struct MyVertex
{
    pos: [f32; 3],
    uv: [f32; 2]
}

// Map over vertices
let vertices: Vec<MyVertex> = shape()
    .with_uvs()
    .into_iter()
    .map(|(pos, uv)| MyVertex { pos: pos, uv: uv })
    .collect();
```