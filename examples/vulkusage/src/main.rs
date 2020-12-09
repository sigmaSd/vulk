extern crate vulkusage;
use vulkusage::spirv;

fn main() {
    let spirv = spirv!();
    let r = vulk::gpu_compute(vec![4, 5, 0, 1, 2, 2].into_iter(), &spirv);
    assert_eq!(r, [54, 55, 50, 51, 52, 52,].to_vec());
}
