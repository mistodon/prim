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

```rust
// Primitives
fn quad()
    .with_center(center: vec3)
    .with_forwards_and_up(normal: vec3, up: vec3)
    .with_size(size: vec2)
    .with_normals()
    .with_uvs()
    .with_indices()
    .into_iter()
    .collect();

fn box()
    .with_center(center: vec3)
    .with_forwards_and_up(forwards: vec3, up: vec3)
    .with_size(size: vec3)
    ...

fn fan(edges: u64)
    .with_center(center: vec3)
    .with_forwards_and_up(forwards: vec3, up: vec3)
    .with_radius(radius: f32)
    ...

fn lines(points: Iterator<vec3>)
    .with_thickness(thickness: f32)
    .with_view_direction(view_dir: vec3)
    ...
```