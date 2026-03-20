# Numix

A small, ergonomic, zero-dependency math library for 2-D, 3-D and 4-D game and graphics work, written in Rust.

The entire public surface is two files: `types.rs` declares the plain structs, and `vector.rs` implements all the methods and operator overloads on them. There are no allocations, no generics, and no trait soup — just `f32` and `i32` vectors that do exactly what the names say.

---

## Types

| Type    | Fields              | Use case                                  |
|---------|---------------------|-------------------------------------------|
| `Vec2`  | `x/y: T`            | 2-D positions, directions, UVs            |
| `Vec3`  | `x/y/z: T`          | 3-D positions, normals, RGB colours       |
| `Vec4`  | `x/y/z/w: T`        | 4-D positions, Matrics                    |
| `Mat3x4`| `[Vec3; 4]`         | Four sets of Vec3's                       |
| `Mat4x4`| `[Vec4; 4]`         | Four sets of Vec4's                       |

---

## Installation

Add the crate to your `Cargo.toml`:

```toml
[dependencies]
numix = { path = "../numix" }
```

Then import what you need:

```rust
use numix::types::{Vec2, Vec3, Vec4, Mat4x4};
```

---

## Vec2

### Construction

```rust
Vec2::new(3.0, 4.0)      // from components
Vec2::zero()             // (0, 0)
Vec2::one()              // (1, 1)
Vec2::splat(2.5)         // (2.5, 2.5)
Vec2::from_angle(angle)  // unit vector at `angle` radians from +X
```

### Arithmetic

All the standard operators are implemented, plus scalar multiplication from both sides:

```rust
let a = Vec2::new(1.0, 2.0);
let b = Vec2::new(3.0, 4.0);

a + b        // (4, 6)
b - a        // (2, 2)
a * 2.0      // (2, 4)
2.0 * a      // (2, 4)  — scalar on the left also works
a / 2.0      // (0.5, 1)
-a           // (-1, -2)

// Compound assignment
let mut v = Vec2::new(1.0, 0.0);
v += Vec2::new(0.5, 0.5);
v -= Vec2::new(0.1, 0.1);
v *= 3.0;
v /= 2.0;
```

### Geometry

```rust
let a = Vec2::new(1.0, 0.0);
let b = Vec2::new(0.0, 1.0);

a.dot(b)                  // 0.0
a.cross(b)                // 1.0  (scalar z of 3-D cross)
a.length()                // 1.0
a.length_squared()        // 1.0  (avoids sqrt)
a.distance(b)             // √2
a.distance_squared(b)     // 2.0

Vec2::new(3.0, 4.0).normalize()         // (0.6, 0.8)
Vec2::new(3.0, 4.0).clamp_length(2.5)  // same direction, length = 2.5

Vec2::new(0.0, 1.0).angle()            // π/2  (radians from +X axis)
a.angle_to(b)                          // signed angle, positive = CCW
```

### Rotation and reflection

```rust
use std::f32::consts::FRAC_PI_2;

let v = Vec2::new(1.0, 0.0);

v.rotate(FRAC_PI_2)      // (0, 1) — rotate 90° CCW
v.perp()                 // (-0, 1) — CCW perpendicular (= rotate 90°)
-v.perp()                // (0, -1) — CW perpendicular (strafe-right direction)

let n = Vec2::new(0.0, 1.0);
Vec2::new(1.0, -1.0).reflect(n)   // (1, 1) — bounce off horizontal surface
Vec2::new(3.0, 0.0).project_onto(Vec2::new(1.0, 1.0).normalize())  // (1.5, 1.5)
```

### Interpolation

```rust
let a = Vec2::zero();
let b = Vec2::one();

a.lerp(b, 0.0)   // (0, 0)
a.lerp(b, 0.5)   // (0.5, 0.5)
a.lerp(b, 1.0)   // (1, 1)
```

### Segment–segment intersection

Returns the intersection point of segment `self → p1` with segment `q0 → q1`, or `None` if they are parallel or do not overlap.

```rust
let p0 = Vec2::new(0.0, 0.0);
let p1 = Vec2::new(1.0, 1.0);
let q0 = Vec2::new(0.0, 1.0);
let q1 = Vec2::new(1.0, 0.0);

p0.intersect_segs(p1, q0, q1)  // Some(Vec2 { x: 0.5, y: 0.5 })
```

### Winding / signed-area test

```rust
// Returns > 0 if `point` is to the right of directed edge A→B,
// < 0 if to the left, 0 if on the line.
let point = Vec2::new(1.0, 0.5);
point.point_side(Vec2::new(0.0, 0.0), Vec2::new(0.0, 1.0))  // > 0 (right side)
```

### Component-wise operations

```rust
let v = Vec2::new(-2.5, 3.7);
v.abs()            // (2.5, 3.7)
v.floor()          // (-3.0, 3.0)
v.ceil()           // (-2.0, 4.0)
v.round()          // (-3.0, 4.0) — nearest integer as f32

let lo = Vec2::new(0.0, 0.0);
let hi = Vec2::new(1.0, 1.0);
Vec2::new(-0.5, 1.5).clamp(lo, hi)   // (0.0, 1.0)

Vec2::new(1.0, 3.0).min(Vec2::new(2.0, 2.0))  // (1, 2)
Vec2::new(1.0, 3.0).max(Vec2::new(2.0, 2.0))  // (2, 3)

v.min_component()  // minimum of x and y
v.max_component()  // maximum of x and y
```

### Approximate equality

```rust
Vec2::new(1.0, 2.0).approx_eq(Vec2::new(1.0, 2.000001), 1e-4)  // true
```

---

## Vec3

`Vec3` mirrors the `Vec2` API and adds the operations that only make sense in 3-D.

```rust
let x = Vec3::new(1.0, 0.0, 0.0);
let y = Vec3::new(0.0, 1.0, 0.0);

x.cross(y)   // (0, 0, 1)

// Construct from Vec2 + z, or extract the XY plane
let flat = Vec2::new(3.0, 4.0);
let v3   = Vec3::from_xy(flat, 5.0);  // (3, 4, 5)
v3.xy()                               // Vec2 { x: 3.0, y: 4.0 }
```

Everything else (`dot`, `length`, `normalize`, `lerp`, `reflect`, `project_onto`, `clamp_length`, component-wise ops, all operators, `Display`, `approx_eq`) works identically to `Vec2`.

---

## Display

All four types implement `Display` for quick debugging:

```rust
// They can also be diffrent types than just Floating values and Integers
println!("{}", Vec2::new(1.0, 2.0));         // (1.0000, 2.0000)
println!("{}", Vec3::new(0.5, 1.0, -2.5));   // (0.5000, 1.0000, -2.5000)

println!("{}", Vec2::new(1, 4));             // (1, 4)
println!("{}", Vec3::new(1, 4, 7));          // (1, 4, 7)
```

---

## Running the tests

```sh
cargo test -p numix
```

The test suite covers construction, arithmetic operators, compound assignment, `normalize` on zero vectors, `intersect_segs` (hit, parallel, and non-overlapping cases), `reflect`, `project_onto`, `clamp_length`, all component-wise ops, and every `From` conversion.

---

## Design notes

**No generics.** `Vec2` is always `f32`, `Vec2i` is always `i32`. Generic vector types are useful in numeric libraries, but in practice game and graphics code almost exclusively uses these two precisions and the lack of type parameters makes call sites cleaner.

**No SIMD.** The library targets readability and correctness first. If you are in a hot loop doing thousands of vectors per frame, profile before reaching for intrinsics — the compiler already auto-vectorises many of these patterns at `opt-level = 3`.

**`Copy` everywhere.** Vectors are small enough that passing by value is always cheaper than passing a reference, and it eliminates a whole class of borrow-checker friction in rendering code.

**`intersect_segs` uses the cross-product formulation.** The parametric formula `t = (diff × dq) / (dp × dq)` is both shorter and more numerically stable than the determinant expansion that appears in many textbooks. The parallel check (`d.abs() < 1e-8`) guards against near-zero denominators.
