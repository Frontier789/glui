use graphics::{DrawShaderSelector, RenderCommand, RenderSequence};
use std::f32::consts::PI;
use tools::{Buffer, DrawMode, Mat4, Rect, Uniform, Vec2, Vec3, Vec4, VertexArray};

#[allow(non_snake_case)]
pub fn parsurf<T, F: Fn(f32, f32) -> T>(surf: F, N: usize, M: usize) -> Vec<T> {
    let N = N.max(2);
    let M = M.max(2);

    let mut pts = Vec::<T>::with_capacity(N * M);
    for i in 0..N {
        for j in 0..M {
            pts.push(surf(i as f32 / (N - 1) as f32, j as f32 / (M - 1) as f32));
        }
    }
    pts
}

#[allow(non_snake_case)]
pub fn parsurf_indices(N: usize, M: usize) -> Vec<u32> {
    let N = N.max(2) as u32;
    let M = M.max(2) as u32;

    let mut indices = Vec::with_capacity((N * (M - 1) * 2 + (M - 2)) as usize);
    for j in 0..M - 1 {
        for i in 0..N {
            indices.push(i + j * N + N);
            indices.push(i + j * N);
        }
        if j + 2 < M {
            indices.push(core::u32::MAX);
        }
    }
    indices
}

#[allow(non_snake_case)]
pub fn parsurf_indices_triangulated(N: usize, M: usize) -> Vec<u32> {
    let N = N.max(2) as u32;
    let M = M.max(2) as u32;

    let mut indices = Vec::with_capacity(((N - 1) * (M - 1) * 6) as usize);
    for j in 0..M - 1 {
        for i in 0..N - 1 {
            indices.push((i + 0) + (j + 0) * N);
            indices.push((i + 1) + (j + 0) * N);
            indices.push((i + 1) + (j + 1) * N);
            indices.push((i + 0) + (j + 0) * N);
            indices.push((i + 1) + (j + 1) * N);
            indices.push((i + 0) + (j + 1) * N);
        }
    }
    indices
}

pub fn parsurf_norms(pts: &Vec<Vec3>, indices: &Vec<u32>) -> Vec<Vec3> {
    let mut norms = vec![Vec3::zero(); pts.len()];
    if indices.len() < 3 {
        return norms;
    }

    let mut i = 0;
    let mut swap = false;
    while i + 2 < indices.len() {
        if indices[i] == core::u32::MAX {
            i += 1;
            swap = false;
        } else if indices[i + 1] == core::u32::MAX {
            i += 2;
            swap = false;
        } else if indices[i + 2] == core::u32::MAX {
            i += 3;
            swap = false;
        } else {
            let ai = indices[if swap { i + 1 } else { i }] as usize;
            let bi = indices[if swap { i } else { i + 1 }] as usize;
            let ci = indices[i + 2] as usize;
            let a = pts[ai];
            let b = pts[bi];
            let c = pts[ci];

            let n = -(a - b).cross(a - c);
            norms[ai] += n;
            norms[bi] += n;
            norms[ci] += n;
            i += 1;
            swap = !swap;
        }
    }

    for n in norms.iter_mut() {
        *n = n.sgn();
    }

    norms
}

#[allow(non_snake_case)]
pub fn loop_normals(norms: &mut Vec<Vec3>, N: usize, M: usize, loop_x: bool, loop_y: bool) {
    if loop_x {
        for i in 0..N {
            let n = (norms[i * M + 0] + norms[i * M + M - 1]).sgn();
            norms[i * M + 0] = n;
            norms[i * M + M - 1] = n;
        }
    }
    if loop_y {
        for j in 0..M {
            let n = (norms[j] + norms[j + (N - 1) * M]).sgn();
            norms[j] = n;
            norms[j + (N - 1) * M] = n;
        }
    }
}

#[allow(non_snake_case)]
pub fn parsurf_triangles<T, F: Fn(f32, f32) -> T>(surf: F, N: usize, M: usize) -> Vec<T> {
    let mut pts = Vec::<T>::with_capacity(N * M * 6);
    for i in 0..N {
        for j in 0..M {
            for (di, dj) in [(0, 0), (0, 1), (1, 1), (0, 0), (1, 1), (1, 0)].iter() {
                pts.push(surf(
                    (i + *di as usize) as f32 / N as f32,
                    (j + *dj as usize) as f32 / M as f32,
                ));
            }
        }
    }
    pts
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct MeshFace {
    a: u32,
    b: u32,
    c: u32,
}

impl From<(u32, u32, u32)> for MeshFace {
    fn from((a, b, c): (u32, u32, u32)) -> Self {
        MeshFace { a, b, c }
    }
}
impl From<(usize, usize, usize)> for MeshFace {
    fn from((a, b, c): (usize, usize, usize)) -> Self {
        MeshFace {
            a: a as u32,
            b: b as u32,
            c: c as u32,
        }
    }
}
impl From<(i32, i32, i32)> for MeshFace {
    fn from((a, b, c): (i32, i32, i32)) -> Self {
        MeshFace {
            a: a as u32,
            b: b as u32,
            c: c as u32,
        }
    }
}

impl MeshFace {
    pub fn new(a: u32, b: u32, c: u32) -> MeshFace {
        MeshFace { a, b, c }
    }
    pub fn from_vec(mut vec: Vec<u32>) -> Vec<MeshFace> {
        unsafe {
            vec.shrink_to_fit();

            let ptr = vec.as_mut_ptr() as *mut MeshFace;
            let len = vec.len();
            let cap = vec.capacity();

            assert_eq!(len % 3, 0);
            assert_eq!(cap % 3, 0);

            std::mem::forget(vec);

            Vec::from_raw_parts(ptr, len / 3, cap / 3)
        }
    }
}

#[derive(Clone)]
pub struct Mesh {
    pub points: Vec<Vec3>,
    pub normals: Option<Vec<Vec3>>,
    pub uvcoords: Option<Vec<Vec2>>,
    pub faces: Vec<MeshFace>,
}

impl Mesh {
    pub fn upload_to_gpu(&self) -> MeshOnGPU {
        let pbuf = Buffer::from_vec(&self.points);
        let nbuf = match &self.normals {
            Some(normals) => Some(Buffer::from_vec(&normals)),
            None => None,
        };
        let ibuf = Buffer::<u32>::from_vec_reinterpret(&self.faces);

        MeshOnGPU {
            points: pbuf,
            normals: nbuf,
            indices: ibuf,
        }
    }

    pub fn as_render_seq(
        &self,
        shader: DrawShaderSelector,
        uniforms: Vec<Uniform>,
    ) -> RenderSequence {
        self.upload_to_gpu().into_render_seq(shader, uniforms)
    }

    pub fn transform_points<F>(mut self, f: F) -> Mesh
    where
        F: Fn(Vec3) -> Vec3,
    {
        for p in self.points.iter_mut() {
            *p = f(*p);
        }
        self
    }
    pub fn linear_transform(mut self, mat: Mat4) -> Mesh {
        for p in self.points.iter_mut() {
            *p = (mat * Vec4::from_vec3(*p, 1.0)).rgb();
        }
        if let Some(normals) = &mut self.normals {
            let norm_mat = mat.inverse().transpose();
            for p in normals.iter_mut() {
                *p = (norm_mat * Vec4::from_vec3(*p, 1.0)).rgb();
            }
        }

        self
    }

    pub fn from_rect(rect: Rect) -> Mesh {
        Mesh {
            points: rect.corners_3d(),
            normals: Some(vec![Vec3::new(0.0, 0.0, 1.0); 4]),
            faces: vec![(0, 1, 2).into(), (0, 2, 3).into()],
            uvcoords: Some(Rect::unit().corners()),
        }
    }

    pub fn screen_quad() -> Mesh {
        let screenrct = Rect::from_min_max(Vec2::new_xy(-1.0), Vec2::new_xy(1.0));
        Mesh::from_rect(screenrct)
    }

    #[allow(non_snake_case)]
    pub fn parsurf<F>(surface: F, N: usize, M: usize, loop_x: bool, loop_y: bool) -> Mesh
    where
        F: Fn(f32, f32) -> Vec3,
    {
        let pts = parsurf(surface, N, M);
        let inds = parsurf_indices(N, M);
        let mut nrms = parsurf_norms(&pts, &inds);

        // let i2 = parsurf_indices_triangulated(N, M);
        // for i in i2 {
        //     println!("{}", i);
        // }

        loop_normals(&mut nrms, N, M, loop_x, loop_y);

        Mesh {
            points: pts,
            normals: Some(nrms),
            faces: MeshFace::from_vec(parsurf_indices_triangulated(N, M)),
            uvcoords: Some(parsurf(|x, y| Vec2::new(x, y), N, M)),
        }
    }

    #[allow(non_snake_case)]
    pub fn unit_sphere(N: usize, M: usize) -> Mesh {
        Mesh::parsurf(
            |x, y| Vec3::pol(1.0, x * PI - PI / 2.0, y * PI * 2.0),
            N,
            M,
            true,
            true,
        )
    }

    #[allow(non_snake_case)]
    pub fn torus(torus_radius: f32, tube_radius: f32, N: usize, M: usize) -> Mesh {
        Mesh::parsurf(
            |x, y| {
                Vec3::new(
                    (torus_radius + tube_radius * (x * 2.0 * PI).cos()) * (y * 2.0 * PI).cos(),
                    (torus_radius + tube_radius * (x * 2.0 * PI).cos()) * (y * 2.0 * PI).sin(),
                    tube_radius * (x * 2.0 * PI).sin(),
                )
            },
            N,
            M,
            true,
            true,
        )
    }

    pub fn unit_cylinder(n: usize) -> Mesh {
        let mut pts = Vec::with_capacity(n * 6);
        let mut faces = vec![];

        let mut body_pts = parsurf_triangles(
            |x, y| Vec3::pol(1.0, 0.0, y * 2.0 * PI) + Vec3::new(0.0, 0.0, x * 2.0 - 1.0),
            2,
            n,
        );
        let cap_pts1 = parsurf_triangles(
            |x, y| Vec3::pol(x, 0.0, -y * 2.0 * PI) + Vec3::new(0.0, 0.0, 1.0),
            2,
            n,
        );
        let cap_pts2 = parsurf_triangles(
            |x, y| Vec3::pol(x, 0.0, y * 2.0 * PI) + Vec3::new(0.0, 0.0, -1.0),
            2,
            n,
        );
        let pts_n = body_pts.len();

        pts.extend(body_pts.drain(0..));
        pts.extend(cap_pts1);
        pts.extend(cap_pts2);
        faces.extend(0..pts_n as u32 * 3);

        let mut m = Mesh {
            points: pts,
            normals: None,
            faces: MeshFace::from_vec(faces),
            uvcoords: None,
        };
        m.generate_flat_normals();
        m
    }

    pub fn unit_cube() -> Mesh {
        let mut m = Mesh {
            points: vec![
                Vec3::new(-1.0, -1.0, -1.0),
                Vec3::new(-1.0, -1.0, 1.0),
                Vec3::new(-1.0, 1.0, 1.0),
                Vec3::new(-1.0, 1.0, -1.0),
                Vec3::new(1.0, -1.0, -1.0),
                Vec3::new(1.0, 1.0, -1.0),
                Vec3::new(1.0, 1.0, 1.0),
                Vec3::new(1.0, -1.0, 1.0),
                Vec3::new(-1.0, -1.0, -1.0),
                Vec3::new(1.0, -1.0, -1.0),
                Vec3::new(1.0, -1.0, 1.0),
                Vec3::new(-1.0, -1.0, 1.0),
                Vec3::new(-1.0, 1.0, -1.0),
                Vec3::new(-1.0, 1.0, 1.0),
                Vec3::new(1.0, 1.0, 1.0),
                Vec3::new(1.0, 1.0, -1.0),
                Vec3::new(-1.0, -1.0, -1.0),
                Vec3::new(-1.0, 1.0, -1.0),
                Vec3::new(1.0, 1.0, -1.0),
                Vec3::new(1.0, -1.0, -1.0),
                Vec3::new(-1.0, -1.0, 1.0),
                Vec3::new(1.0, -1.0, 1.0),
                Vec3::new(1.0, 1.0, 1.0),
                Vec3::new(-1.0, 1.0, 1.0),
            ],
            normals: None,
            faces: MeshFace::from_vec(vec![
                0, 1, 2, 0, 2, 3, 4, 5, 6, 4, 6, 7, 8, 9, 10, 8, 10, 11, 12, 13, 14, 12, 14, 15,
                16, 17, 18, 16, 18, 19, 20, 21, 22, 20, 22, 23,
            ]),
            uvcoords: None, // TODO
        };
        m.generate_flat_normals();
        m
    }
    pub fn new() -> Mesh {
        Mesh {
            points: vec![],
            normals: None,
            faces: MeshFace::from_vec(vec![]),
            uvcoords: None,
        }
    }

    pub fn generate_flat_normals(&mut self) {
        let mut normals = vec![Vec3::zero(); self.points.len()];

        for face in self.faces.iter() {
            let a = self.points[face.a as usize];
            let b = self.points[face.b as usize];
            let c = self.points[face.c as usize];
            let n = (a - b).cross(a - c).sgn();
            normals[face.a as usize] = n;
            normals[face.b as usize] = n;
            normals[face.c as usize] = n;
        }

        self.normals = Some(normals)
    }

    pub fn aabb(&self) -> (Vec3, Vec3) {
        let p0 = self.points[0];

        let mn = self.points.iter().fold(p0, |p, q| {
            Vec3::new(p.x.min(q.x), p.y.min(q.y), p.z.min(q.z))
        });
        let mx = self.points.iter().fold(p0, |p, q| {
            Vec3::new(p.x.max(q.x), p.y.max(q.y), p.z.max(q.z))
        });

        (mn, mx)
    }

    pub fn fit_into_aabb_into_unit_cube(mut self, aabb: (Vec3, Vec3)) -> Mesh {
        let (mn, mx) = aabb;

        let span = mx - mn;
        for p in self.points.iter_mut() {
            *p = (*p - mn) / span * 2.0 - Vec3::new(1.0, 1.0, 1.0);
        }

        self
    }

    pub fn extend_aabb(&self, aabb: (Vec3, Vec3)) -> (Vec3, Vec3) {
        let maabb = self.aabb();
        (
            Vec3::new(
                aabb.0.x.min(maabb.0.x),
                aabb.0.y.min(maabb.0.y),
                aabb.0.z.min(maabb.0.z),
            ),
            Vec3::new(
                aabb.1.x.max(maabb.1.x),
                aabb.1.y.max(maabb.1.y),
                aabb.1.z.max(maabb.1.z),
            ),
        )
    }

    pub fn fit_into_unit_cube(self) -> Mesh {
        let aabb = self.aabb();
        self.fit_into_aabb_into_unit_cube(aabb)
    }
}

pub struct MeshOnGPU {
    pub points: Buffer<Vec3>,
    pub normals: Option<Buffer<Vec3>>,
    pub indices: Buffer<u32>,
}

impl MeshOnGPU {
    pub fn into_render_seq(
        self,
        shader: DrawShaderSelector,
        uniforms: Vec<Uniform>,
    ) -> RenderSequence {
        let mut vao = VertexArray::new();
        let mut render_seq = RenderSequence::new();

        vao.attrib_buffer(0, &self.points);
        render_seq.add_buffer(self.points.into_base_type());

        if let Some(nbuf) = self.normals {
            vao.attrib_buffer(3, &nbuf);
            render_seq.add_buffer(nbuf.into_base_type());
        }

        vao.set_indices_buffer(&self.indices);
        render_seq.add_index_buffer(self.indices);

        render_seq.add_command(RenderCommand::new_uniforms(
            vao,
            DrawMode::Triangles,
            shader,
            uniforms,
        ));

        render_seq
    }
    pub fn non_owning_render_seq(
        &self,
        shader: DrawShaderSelector,
        uniforms: Vec<Uniform>,
    ) -> RenderSequence {
        let mut vao = VertexArray::new();
        let mut render_seq = RenderSequence::new();

        vao.attrib_buffer(0, &self.points);

        if let Some(nbuf) = &self.normals {
            vao.attrib_buffer(3, &nbuf);
        }

        vao.set_indices_buffer(&self.indices);

        render_seq.add_command(RenderCommand::new_uniforms(
            vao,
            DrawMode::Triangles,
            shader,
            uniforms,
        ));

        render_seq
    }
    pub fn as_render_command(
        &self,
        shader: DrawShaderSelector,
        uniforms: Vec<Uniform>,
    ) -> RenderCommand {
        let mut vao = VertexArray::new();

        vao.attrib_buffer(0, &self.points);

        if let Some(nbuf) = &self.normals {
            vao.attrib_buffer(3, &nbuf);
        }

        vao.set_indices_buffer(&self.indices);

        RenderCommand::new_uniforms(vao, DrawMode::Triangles, shader, uniforms)
    }
}
