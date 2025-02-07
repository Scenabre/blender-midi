use core::f32;

const BLENDER_ZMAX: f32 = 10000.0;

pub fn add(a: f32, b: f32) -> f32 {
    a + b
}

pub fn multiply(a: f32, b: f32) -> f32 {
    a * b
}

pub fn divide(a: f32, b: f32) -> f32 {
    a / b
}

pub fn abs(a: f32) -> f32 {
    f32::abs(a)
}

pub fn mul_add(a: f32, b: f32, c: f32) -> f32 {
    f32::mul_add(a, b, c)
}

pub fn pow(a: f32, n: f32) -> f32 {
    f32::powf(a, n)
}

pub fn log(a: f32, n: f32) -> f32 {
    f32::log(a, n)
}

pub fn exp(a: f32) -> f32 {
    f32::exp(a)
}

pub fn sqrt(a: f32) -> f32 {
    f32::sqrt(a)
}

pub fn inv_sqrt(a: f32) -> f32 {
    1.0 / f32::sqrt(a)
}

pub fn min(a: f32, b: f32) -> f32 {
    f32::min(a, b)
}

pub fn max(a: f32, b: f32) -> f32 {
    f32::max(a, b)
}

pub fn compare(a: f32, b: f32, e: f32) -> bool {
    let abs_a = a.abs();
    let abs_b = b.abs();
    let diff = f32::abs(a - b);

    if a == b {
        return true;
    } else if a == 0.0 || b == 0.0 || (abs_a + abs_b) < BLENDER_ZMAX {
        return diff < (e * f32::MIN_POSITIVE);
    } else {
        return diff / f32::min(abs_a + abs_b, f32::MAX) < e;
    }
}

pub fn lt(a: f32, b: f32) -> bool {
    a < b
}

pub fn gt(a: f32, b: f32) -> bool {
    a > b
}

pub fn le(a: f32, b: f32) -> bool {
    a <= b
}

pub fn ge(a: f32, b: f32) -> bool {
    a >= b
}

//pub fn

pub fn map_range(
    value: f32,
    from_min: f32,
    from_max: f32,
    to_min: f32,
    to_max: f32,
    clamp: bool,
) -> f32 {
    if f32::abs(from_max - from_min) < 1e-6 {
        return 0.0;
    }

    let mut result: f32;

    if (-BLENDER_ZMAX..=BLENDER_ZMAX).contains(&value) {
        result = (value - from_min) / (from_max - from_min);
        result = to_min + result * (to_max - to_min);
    } else if value > BLENDER_ZMAX {
        result = to_max;
    } else {
        result = to_min;
    }

    if clamp {
        result = result.clamp(to_min, to_max);
    }

    result
}
