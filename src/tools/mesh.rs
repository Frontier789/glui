use tools::Vec3;

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
            indices.push(i + j * N);
            indices.push(i + j * N + N);
        }
        if j + 2 < M {
            indices.push(core::u32::MAX);
        }
    }
    indices
}

#[allow(non_snake_case)]
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

            let n = (a - b).cross(a - c);
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
pub fn parsurf_triangles<T, F: Fn(f32, f32) -> T>(surf: F, N: usize, M: usize) -> Vec<T> {
    let mut pts = Vec::<T>::with_capacity((N - 1) * (M - 1) * 6);
    for i in 0..N {
        for j in 0..M {
            for (di, dj) in [(0, 0), (1, 0), (1, 1), (0, 0), (1, 1), (0, 1)].iter() {
                pts.push(surf(
                    (i + *di as usize) as f32 / N as f32,
                    (j + *dj as usize) as f32 / M as f32,
                ));
            }
        }
    }
    pts
}
