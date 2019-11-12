use math_utils::*;
use rand::prelude::*;

//calculate circumstance illumination
pub fn random_in_sphere() -> Vector3 {
    let mut rng = rand::thread_rng();
    let mut p = 2.0 * Vector3::new(rng.gen(),rng.gen(),rng.gen()) - Vector3::broadcast(1.0);
    while p.length_square() >= 1.0 {
        p = 2.0 * Vector3::new(rng.gen(),rng.gen(),rng.gen()) - Vector3::broadcast(1.0);
    }
    p
}

//reflection
pub fn reflect(v: Vector3, n: Vector3) -> Vector3 {
    v - 2.0 * dot(v,n) * n
}

pub fn refract(v: Vector3, n: Vector3, ni_over_nt: f32,reflected: &mut Vector3) -> bool {
    let uv = v.normalized();
    let dt = dot(uv, n);
    let discriminant = 1.0 - ni_over_nt * ni_over_nt * (1.0 - dt * dt);
    if discriminant > 0.0 {
        *reflected = ni_over_nt * (uv - dt * n) - discriminant.sqrt() * n;
        true
    }
    else {
        false
    }
}

pub fn shlick(cosine: f32, refract_coe: f32) -> f32 {
    let mut r = (1.0 - refract_coe) / (1.0 + refract_coe);
    r = r * r;
    r + (1.0 - r) * (1.0 - cosine).powf(5.0)
}