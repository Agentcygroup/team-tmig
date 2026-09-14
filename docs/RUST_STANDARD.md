Rust Secure Coding Standard
Control: ISO/IEC 27001:2022 A.8.28.
1. No unsafe outside src/ffi.rs.
2. No unwrap/expect in src/, only src/bin/.
3. Fallible public fns return Result or Option.
4. No panic! in library code.
5. No todo!/unimplemented!.
6. No const fn calling non-const methods.
7. No pub use module::* from siblings.
8. State enums: Idle/Active/Degraded/Failed.
9. Every module has mod.rs.
10. Every public fn has a doc comment.
