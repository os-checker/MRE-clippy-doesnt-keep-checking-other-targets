`cargo check` will check on lib and bin targets by default, but `cargo clippy` 
stops once an error is met.

```rust
$ cargo check
warning: value assigned to `a` is never read
 --> src/lib.rs:2:13
  |
2 |     let mut a = 123;
  |             ^
  |
  = help: maybe it is overwritten before being read?
  = note: `#[warn(unused_assignments)]` on by default

warning: value assigned to `b` is never read
 --> src/lib.rs:6:5
  |
6 |     b = a;
  |     ^
  |
  = help: maybe it is overwritten before being read?

warning: function `lib` is never used
 --> src/lib.rs:1:4
  |
1 | fn lib() {
  |    ^^^
  |
  = note: `#[warn(dead_code)]` on by default

warning: crate `MRE_clippy_doesnt_keep_going` should have a snake case name
  |
  = help: convert the identifier to snake case: `mre_clippy_doesnt_keep_going`
  = note: `#[warn(non_snake_case)]` on by default

warning: `MRE-clippy-doesnt-keep-going` (lib) generated 4 warnings
warning: unused variable: `a`
 --> src/main.rs:3:9
  |
3 |     let a = 3.14;
  |         ^ help: if this is intentional, prefix it with an underscore: `_a`
  |
  = note: `#[warn(unused_variables)]` on by default

warning: `MRE-clippy-doesnt-keep-going` (bin "MRE-clippy-doesnt-keep-going") generated
 1 warning
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.07s
```

Only lib diagnostics are emitted by clippy:

```rust
$ cargo clippy
    Checking MRE-clippy-doesnt-keep-going v0.1.0 (/rust/tmp/os-checker/MRE-clippy-does
nt-keep-going)
warning: value assigned to `a` is never read
 --> src/lib.rs:2:13
  |
2 |     let mut a = 123;
  |             ^
  |
  = help: maybe it is overwritten before being read?
  = note: `#[warn(unused_assignments)]` on by default

warning: value assigned to `b` is never read
 --> src/lib.rs:6:5
  |
6 |     b = a;
  |     ^
  |
  = help: maybe it is overwritten before being read?

warning: function `lib` is never used
 --> src/lib.rs:1:4
  |
1 | fn lib() {
  |    ^^^
  |
  = note: `#[warn(dead_code)]` on by default

error: this looks like you are trying to swap `a` and `b`
 --> src/lib.rs:5:5
  |
5 | /     a = b;
6 | |     b = a;
  | |_________^ help: try: `std::mem::swap(&mut a, &mut b)`
  |
  = note: or maybe you should use `std::mem::replace`?
  = help: for further information visit https://rust-lang.github.io/rust-clippy/master
/index.html#almost_swapped
  = note: `#[deny(clippy::almost_swapped)]` on by default

warning: crate `MRE_clippy_doesnt_keep_going` should have a snake case name
  |
  = help: convert the identifier to snake case: `mre_clippy_doesnt_keep_going`
  = note: `#[warn(non_snake_case)]` on by default

warning: `MRE-clippy-doesnt-keep-going` (lib) generated 4 warnings
error: could not compile `MRE-clippy-doesnt-keep-going` (lib) due to 1 previous error;
 4 warnings emitted
```

Even though `--all-targets` is specified:

```rust
$ cargo clippy --all-targets
    Checking MRE-clippy-doesnt-keep-going v0.1.0 (/rust/tmp/os-checker/MRE-clippy-does
nt-keep-going)
warning: value assigned to `a` is never read
 --> src/lib.rs:2:13
  |
2 |     let mut a = 123;
  |             ^
  |
  = help: maybe it is overwritten before being read?
  = note: `#[warn(unused_assignments)]` on by default

warning: value assigned to `b` is never read
 --> src/lib.rs:6:5
  |
6 |     b = a;
  |     ^
  |
  = help: maybe it is overwritten before being read?

warning: function `lib` is never used
 --> src/lib.rs:1:4
  |
1 | fn lib() {
  |    ^^^
  |
  = note: `#[warn(dead_code)]` on by default

error: this looks like you are trying to swap `a` and `b`
 --> src/lib.rs:5:5
  |
5 | /     a = b;
6 | |     b = a;
  | |_________^ help: try: `std::mem::swap(&mut a, &mut b)`
  |
  = note: or maybe you should use `std::mem::replace`?
  = help: for further information visit https://rust-lang.github.io/rust-clippy/master
/index.html#almost_swapped
  = note: `#[deny(clippy::almost_swapped)]` on by default

warning: crate `MRE_clippy_doesnt_keep_going` should have a snake case name
  |
  = help: convert the identifier to snake case: `mre_clippy_doesnt_keep_going`
  = note: `#[warn(non_snake_case)]` on by default

warning: `MRE-clippy-doesnt-keep-going` (lib) generated 4 warnings
error: could not compile `MRE-clippy-doesnt-keep-going` (lib) due to 1 previous error;
 4 warnings emitted
warning: build failed, waiting for other jobs to finish...
warning: `MRE-clippy-doesnt-keep-going` (lib test) generated 3 warnings (3 duplicates)
error: could not compile `MRE-clippy-doesnt-keep-going` (lib test) due to 1 previous e
rror; 3 warnings emitted
```
