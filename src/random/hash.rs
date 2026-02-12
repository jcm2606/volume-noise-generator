pub fn pcg_11(v: u32) -> u32 {
    const A: u32 = 747796405u32;
    const B: u32 = 2891336453u32;
    const C: u32 = 4u32;
    const D: u32 = 277803737u32;

    let state = v.wrapping_mul(A).wrapping_add(B);
    let word =
        ((state >> ((state >> 28u32).wrapping_add(C))) ^ state).wrapping_mul(D);
    (word >> 22u32) ^ word
}

pub fn pcg_31(v: glam::UVec3) -> u32 {
    pcg_11(v.x.wrapping_add(pcg_11(v.y.wrapping_add(pcg_11(v.z)))))
}

pub fn pcg_33(mut v: glam::UVec3) -> glam::UVec3 {
    const A: glam::UVec3 = glam::UVec3::splat(1664525u32);
    const B: glam::UVec3 = glam::UVec3::splat(1013904223u32);

    v = v.wrapping_mul(A).wrapping_add(B);

    v.x = v.x.wrapping_add(v.y.wrapping_mul(v.z));
    v.y = v.y.wrapping_add(v.z.wrapping_mul(v.x));
    v.z = v.z.wrapping_add(v.x.wrapping_mul(v.y));

    v ^= v >> 16u32;

    v.x = v.x.wrapping_add(v.y.wrapping_mul(v.z));
    v.y = v.y.wrapping_add(v.z.wrapping_mul(v.x));
    v.z = v.z.wrapping_add(v.x.wrapping_mul(v.y));

    v
}

pub fn pcg_41(v: glam::UVec4) -> u32 {
    pcg_11(v.x.wrapping_add(pcg_11(v.y.wrapping_add(pcg_11(v.z.wrapping_add(pcg_11(v.w)))))))
}

pub fn pcg_44(mut v: glam::UVec4) -> glam::UVec4 {
    const A: glam::UVec4 = glam::UVec4::splat(1664525u32);
    const B: glam::UVec4 = glam::UVec4::splat(1013904223u32);

    v = v.wrapping_mul(A).wrapping_add(B);

    v.x = v.x.wrapping_add(v.y.wrapping_mul(v.w));
    v.y = v.y.wrapping_add(v.z.wrapping_mul(v.x));
    v.z = v.z.wrapping_add(v.x.wrapping_mul(v.y));
    v.w = v.w.wrapping_add(v.y.wrapping_mul(v.z));

    v ^= v >> 16u32;

    v.x = v.x.wrapping_add(v.y.wrapping_mul(v.w));
    v.y = v.y.wrapping_add(v.z.wrapping_mul(v.x));
    v.z = v.z.wrapping_add(v.x.wrapping_mul(v.y));
    v.w = v.w.wrapping_add(v.y.wrapping_mul(v.z));

    v
}