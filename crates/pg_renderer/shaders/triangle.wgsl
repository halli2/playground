alias Vec4 = vec4<f32>;

@vertex
fn vs_main(@builtin(vertex_index) in_vertex_index: u32) -> @builtin(position) Vec4 {
    let x = f32(i32(in_vertex_index) - 1);
    let y = f32(i32(in_vertex_index & 1u) * 2 - 1);
    return Vec4(x, y, 0.0, 1.0);
}

@fragment
fn fs_main() -> @location(0) Vec4 {
    return Vec4(1.0, 0.0, 0.0, 1.0);
}
