fn main() {
    let source = "#version 450

layout(local_size_x = 64, local_size_y = 1, local_size_z = 1) in;

layout(set = 0, binding = 0) buffer Data {
    uint data[];
} buf;

void main() {
    uint idx = gl_GlobalInvocationID.x;
    buf.data[idx] += 50;
}";

    let mut compiler = shaderc::Compiler::new().unwrap();
    let artifact = compiler
        .compile_into_spirv(
            source,
            shaderc::ShaderKind::Compute,
            "shader.glsl",
            "main",
            None,
        )
        .unwrap();
    let spirv = artifact.as_binary_u8();
    let r = vulk::gpu_compute(vec![4, 5, 0, 1, 2, 2].into_iter(), spirv);
    dbg!(&r);
}
