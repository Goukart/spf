#[derive(Copy, Clone)]

pub struct Vertex{
    position:(f32, f32, f32)
}

implement_vertex!(Vertex, position);

pub const VERTICES: [Vertex; 4] = [
Vertex { position: (-10.0, -10.0, 0.0) },
Vertex { position: (-10.00, 10.00, 0.0) },
Vertex { position: (10.00, -10.00, 0.0) },
Vertex { position: (10.00, 10.00, 0.0) },
];

#[derive(Copy, Clone)]
pub struct Normal {
    normal: (f32, f32, f32)
}

implement_vertex!(Normal, normal);

pub const NORMALS: [Normal; 4] = [
    Normal { normal: (0.0, 0.0, 0.0) },
    Normal { normal: (-0.966742, -0.255752, 0.0) },
    Normal { normal: (-0.966824, 0.255443, 0.0) },
    Normal { normal: (-0.092052, 0.995754, 0.0) },
];

pub const INDICES: [u16; 6] = [
    0, 1, 2,
    1, 2, 3
];
