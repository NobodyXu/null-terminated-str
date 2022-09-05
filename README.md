# null-terminated-str

[![Rust](https://github.com/NobodyXu/null-terminated-str/actions/workflows/build.yml/badge.svg)](https://github.com/NobodyXu/null-terminated-str/actions/workflows/build.yml)

[![crate.io downloads](https://img.shields.io/crates/d/null-terminated-str)](https://crates.io/crates/null-terminated-str)

[![crate.io version](https://img.shields.io/crates/v/null-terminated-str)](https://crates.io/crates/null-terminated-str)

[![docs](https://docs.rs/null-terminated-str/badge.svg)](https://docs.rs/null-terminated-str)

Provides null terminated utf-8 str `NullTerminatedStr` (borrowed) and
`NullTerminatedString` (owned) that is compatible with
 - `std::ffi::CStr`
 - `std::ffi::CString`
 - `str`
 - `String`

Also provides `const_null_terminated_str!` to create `NullTerminatedStr`
at compile time and `IntoNullTerminatedString` that accepts `&str`,
`String`, `&NullTerminatedStr` and `NullTerminatedString` to avoid
frequent allocation in FFI call.
