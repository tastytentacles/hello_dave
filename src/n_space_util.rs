use vecmath::Vector2;


pub fn n_space_unwrap(n: usize, width: usize) -> Vector2<u32>
    { return [n as u32 % width as u32, f32::floor(n as f32 / width as f32) as u32] }

pub fn n_space_uv(n: usize, width: usize) -> Vector2<f32> {
    let n_space = n_space_unwrap(n, width);
    
    return [
        n_space[0] as f32 / width as f32,
        n_space[0] as f32 / width as f32 ];
}