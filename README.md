# Numix

A small, ergonomic, zero-dependency math library for 2-D, 3-D and 4-D game and graphics work, written in Rust.

The entire public surface is two files: `types.rs` declares the plain structs, and `vector.rs` implements all the methods and operator overloads on them. There are no allocations — just lightweight, `Copy` vector and matrix types that do exactly what the names say.

---

## Types

| Type     | Fields              | Use case                              |
|----------|---------------------|---------------------------------------|
| `Vec2<T>`| `x, y: T`           | 2-D positions, directions, UVs        |
| `Vec3<T>`| `x, y, z: T`        | 3-D positions, normals, RGB colours   |
| `Vec4<T>`| `x, y, z, w: T`     | Homogeneous coordinates, matrix rows  |
| `Mat3x4<T>` | `[Vec4<T>; 3]`  | Compact affine transform (3 rows × 4 columns) |
| `Mat4x4<T>` | `[Vec4<T>; 4]`  | Full 4×4 transform matrix             |

All types are generic over a scalar `T`. In practice you will use `f32` for most geometry and `i32` for integer grid work.

---

## Installation

Add the crate to your `Cargo.toml`:

```toml
[dependencies]
numix = { path = "../numix" }
```

Then import what you need:

```rust
use numix::types::{Vec2, Vec3, Vec4, Mat4x4, Mat3x4};
```

---

## Vec2\<T\>

### Construction

```rust
Vec2::new(3.0_f32, 4.0)   // from components
```

### Arithmetic

All standard operators are implemented for any scalar `T` that satisfies the `Scalar` trait bound (`Copy + Default + PartialEq + Add + Sub + Mul + Neg`):

```rust
let a = Vec2::new(1, 2);
let b = Vec2::new(3, 4);

a + b        // Vec2 { x: 4, y: 6 }
b - a        // Vec2 { x: 2, y: 2 }
a * 2        // Vec2 { x: 2, y: 4 }
-a           // Vec2 { x: -1, y: -2 }
```

Compound assignment requires `T: AddAssign` or `T: SubAssign` respectively:

```rust
let mut v = Vec2::new(1, 0);
v += Vec2::new(3, 4);   // Vec2 { x: 4, y: 4 }
v -= Vec2::new(1, 1);   // Vec2 { x: 3, y: 3 }
v *= 2;                 // Vec2 { x: 6, y: 6 }  — requires T: MulAssign
```

### Geometry

```rust
let a = Vec2::new(1, 2);
let b = Vec2::new(3, 4);

a.dot(b)         // 11  (1×3 + 2×4)
a.length_sq()    // 5   (1² + 2²), avoids sqrt
```

### Swizzle

```rust
// Vec4::xyz() extracts the first three components as a Vec3
let v = Vec4::new(1, 2, 3, 99);
v.xyz()   // Vec3 { x: 1, y: 2, z: 3 }
```

---

## Vec3\<T\>

`Vec3<T>` mirrors the `Vec2<T>` API and adds the 3-D cross product:

```rust
let x = Vec3::new(1, 0, 0);
let y = Vec3::new(0, 1, 0);

x.cross(y)   // Vec3 { x: 0, y: 0, z: 1 }   (right-hand rule)
x.dot(y)     // 0
x.length_sq()   // 1
```

Cross product is anticommutative:

```rust
assert_eq!(a.cross(b), -b.cross(a));
```

All arithmetic operators (`+`, `-`, `*`, unary `-`, `+=`, `-=`, `*=`) and `Display` work identically to `Vec2<T>`.

---

## Vec4\<T\>

`Vec4<T>` adds a `w` component for homogeneous coordinates and matrix-row use:

```rust
let v = Vec4::new(1, 2, 3, 4);

v.dot(Vec4::new(1, 0, 0, 0))   // 1
v.length_sq()                  // 30  (1+4+9+16)
v.xyz()                        // Vec3 { x: 1, y: 2, z: 3 }
```

All arithmetic operators (`+`, `-`, `*`, unary `-`, `+=`, `-=`, `*=`) and `Display` are implemented.

---

## Mat4x4\<T\>

A row-major 4×4 matrix. `mat[r]` is a `Vec4<T>` holding the four elements of row `r`.

```rust
let identity = Mat4x4::from([
    [1, 0, 0, 0],
    [0, 1, 0, 0],
    [0, 0, 1, 0],
    [0, 0, 0, 1],
]);

let v = Vec4::new(1, 2, 3, 4);
identity * v   // Vec4 { x: 1, y: 2, z: 3, w: 4 }
```

You can also call `.mul_vec4(v)` directly if you prefer the method form.

---

## Mat3x4\<T\>

A row-major 3×4 matrix (3 rows, 4 columns). The implicit last row is `[0, 0, 0, 1]`, making it a compact representation for affine transforms that avoids storing the constant row.

`mat[r]` is a `Vec4<T>` holding the four elements of row `r`.

```rust
// Affine translation: rows are [1 0 0 tx], [0 1 0 ty], [0 0 1 tz]
let m = Mat3x4::from([
    [1, 0, 0, 5],
    [0, 1, 0, 6],
    [0, 0, 1, 7],
]);

// A point (w=1) is translated
m * Vec4::new(0, 0, 0, 1)   // Vec3 { x: 5, y: 6, z: 7 }

// A direction (w=0) is not
m * Vec4::new(1, 0, 0, 0)   // Vec3 { x: 1, y: 0, z: 0 }
```

You can also call `.mul_vec4(v)` directly.

---

## Display

All four types implement `Display`. Formatting delegates to the inner `T`, so output style depends on the scalar type:

```rust
println!("{}", Vec2::new(1.0_f32, 2.0));        // (1, 2)
println!("{}", Vec3::new(0.5_f32, 1.0, -2.5));  // (0.5, 1, -2.5)
println!("{}", Vec2::new(1_i32, 4));            // (1, 4)
println!("{}", Vec3::new(1_i32, 4, 7));         // (1, 4, 7)
```

---

## Running the tests

```sh
cargo test -p numix
```

The test suite covers construction, arithmetic operators, compound assignment, dot products, cross products, `xyz()` extraction, matrix–vector multiplication, identity and translation matrices, and `From` array conversions.

---

## Design notes

**Generic over `T`, not specialised per precision.** The `Scalar` trait captures the minimal operations a component type must support (`Copy + Default + PartialEq + Add + Sub + Mul + Neg`). In practice you will use `f32` for geometry and `i32` for integer grid work, but the same struct serves both without duplication.

**No SIMD.** The library targets readability and correctness first. If you are in a hot loop doing thousands of vectors per frame, profile before reaching for intrinsics — the compiler already auto-vectorises many of these patterns at `opt-level = 3`.

**`Copy` everywhere.** Vectors are small enough that passing by value is always cheaper than passing a reference, and it eliminates a whole class of borrow-checker friction in rendering code.

**`Mat3x4` uses a row-per-`Vec4` layout.** Storing three `Vec4` rows makes `mul_vec4` a clean sequence of three dot products and keeps the translation components in the `w` slot of each row — no special-casing needed.

**`Mat3x4::mul_vec4` respects `w`.** Multiplying by a point (`w = 1`) applies the full affine transform including translation. Multiplying by a direction (`w = 0`) applies only the linear part. This falls out of the dot-product formulation automatically.
