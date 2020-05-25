#[allow(non_snake_case)]
pub fn parsurf<T, F: Fn(f32, f32) -> T>(surf: F, N: usize, M: usize) -> Vec<T> {
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
