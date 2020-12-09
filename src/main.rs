vulk::shader!(
    "
#version 450

layout(local_size_x = 64, local_size_y = 1, local_size_z = 1) in;

layout(set = 0, binding = 0) buffer Data {
    uint data[];
} buf;

void main() {
    uint idx = gl_GlobalInvocationID.x;
    buf.data[idx] *= 50;
}"
);

fn main() {
    let r = gpu_compute(vec![4, 5, 0, 1, 2, 2].into_iter());
    dbg!(&r);
}
