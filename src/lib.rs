//! A [Casdoor](https://github.com/casdoor/casdoor) SDK (contain APIs) with more complete interfaces and better usability.

// -------- rust coding guidelines: https://rust-coding-guidelines.github.io/rust-coding-guidelines-zh/ --------
// -------- rustc lint doc: https://doc.rust-lang.org/rustc/lints/listing/index.html --------
// -------- rust-clippy doc: https://rust-lang.github.io/rust-clippy/master/index.html --------

// [REQUIRED] G.VAR.02 Do not use non-ASCII characters in identifiers
#![deny(non_ascii_idents)]
// [REQUIRED]
#![allow(clippy::disallowed_names)]
// [REQUIRED]
#![allow(clippy::blanket_clippy_restriction_lints)]
// [REQUIRED] G.CMT.02 Add Panic documentation in the docs of public APIs that may panic under certain circumstances
#![warn(clippy::missing_panics_doc)]
// [RECOMMENDED] G.CNS.05 Use const fn for functions or methods wherever applicable
#![warn(clippy::missing_const_for_fn)]
// [REQUIRED] G.TYP.01 Prefer safe conversion functions over `as` for type casting
#![warn(
    clippy::as_conversions,
    clippy::cast_lossless,
    clippy::cast_possible_truncation,
    clippy::cast_possible_wrap,
    clippy::ptr_as_ptr
)]
// [RECOMMENDED] G.VAR.01 Avoid using too many meaningless variable names when destructuring tuples with more than four
// variables
#![warn(clippy::many_single_char_names)]
// [RECOMMENDED] G.TYP.02 Explicitly specify the type for numeric literals
#![warn(clippy::default_numeric_fallback)]
// [RECOMMENDED] G.TYP.03 Use `try_from` methods instead of relying on numeric boundaries for safe conversion
#![warn(clippy::checked_conversions)]
// [RECOMMENDED] G.TYP.BOL.02 Use `if` expressions instead of `match` for boolean conditions
#![warn(clippy::match_bool)]
// [RECOMMENDED] G.TYP.BOL.05 Use logical operators (&&/||) instead of bitwise operators (&/|) for boolean operations
// when not necessary
#![warn(clippy::needless_bitwise_bool)]
// [REQUIRED] G.TYP.INT.02 Avoid `as` casting between signed and unsigned integers; use safe conversion functions
#![deny(clippy::cast_sign_loss)]
// [REQUIRED] G.TYP.INT.03 Avoid using `%` for modulo operations on negative numbers
#![warn(clippy::modulo_arithmetic)]
// [REQUIRED] G.TYP.FLT.02 Avoid precision loss when casting from any numeric type to floating-point; use safe
// conversion functions
#![warn(clippy::cast_precision_loss)]
// [REQUIRED] G.TYP.FLT.03 Be cautious of precision loss in floating-point arithmetic and comparisons
#![warn(clippy::float_arithmetic, clippy::float_cmp, clippy::float_cmp_const)]
// [REQUIRED] G.TYP.FLT.04 Use Rust's built-in methods for floating-point calculations
#![warn(clippy::imprecise_flops, clippy::suboptimal_flops)]
// [OPTIONAL] G.TYP.ARR.01 Use static variables instead of constants for large global arrays
#![warn(clippy::large_stack_arrays)]
// [RECOMMENDED] G.FUD.03 Consider using a custom struct or enum instead of many boolean parameters in function
// signatures
#![warn(clippy::fn_params_excessive_bools)]
// [RECOMMENDED] G.TYP.ENM.04 Avoid using glob imports for enum variants in `use` statements
#![warn(clippy::enum_glob_use)]
// [RECOMMENDED] G.CTF.02 Ensure `else` branches are present whenever `else if` is used
#![warn(clippy::else_if_without_else)]
// [RECOMMENDED] G.STR.03 Convert string literals containing only ASCII characters to byte sequences using `b"str"`
// syntax instead of `as_bytes()`
#![warn(clippy::string_lit_as_bytes)]
// [RECOMMENDED] G.STR.05 Take care to avoid disrupting UTF-8 encoding when slicing strings at specific positions
#![warn(clippy::string_slice)]
// [RECOMMENDED] G.FUD.02 Prefer passing large values by reference if function parameters implement `Copy`
#![warn(clippy::large_types_passed_by_value)]
// [RECOMMENDED] G.FUD.04 Pass small `Copy` type values by value instead of by reference
#![warn(clippy::trivially_copy_pass_by_ref)]
// [REQUIRED] G.GEN.02 Be cautious to avoid using generic default implementations of some methods from Rust's standard
// library; prefer specific type implementations
#![warn(clippy::inefficient_to_string)]
// [REQUIRED] G.TRA.BLN.02 Do not implement the `Copy` trait for iterators
#![warn(clippy::copy_iterator)]
// [RECOMMENDED] G.TRA.BLN.07 Use `copied` method instead of `cloned` for iterable `Copy` types
#![warn(clippy::cloned_instead_of_copied)]
// [RECOMMENDED] G.ERR.01 Avoid using `unwrap` indiscriminately when handling `Option<T>` and `Result<T, E>`
#![warn(clippy::unwrap_used)]
// [RECOMMENDED] G.MOD.03 Avoid using wildcard imports in module declarations
#![warn(clippy::wildcard_imports)]
// [REQUIRED] G.MOD.04 Avoid using different module layout styles within the same project
#![warn(clippy::self_named_module_files)]
// [RECOMMENDED] G.CAR.02 Ensure that necessary metadata is included in the `Cargo.toml` of the crate
#![warn(clippy::cargo_common_metadata)]
// [RECOMMENDED] G.CAR.03 Avoid negative or redundant prefixes and suffixes in feature names
#![warn(clippy::negative_feature_names, clippy::redundant_feature_names)]
// [REQUIRED] G.CAR.04 Avoid using wildcard dependencies in `Cargo.toml`
#![warn(clippy::wildcard_dependencies)]
// [RECOMMENDED] G.MAC.01 Only use the `dbg!()` macro for debugging code
#![warn(clippy::dbg_macro)]
// [REQUIRED] Ensure that locks are released before `await` is called in asynchronous code
#![warn(clippy::await_holding_lock)]
// [REQUIRED] Handle `RefCell` references across `await` points
#![warn(clippy::await_holding_refcell_ref)]
// [RECOMMENDED] G.ASY.04 Avoid defining unnecessary async functions
#![warn(clippy::unused_async)]
// [REQUIRED] G.UNS.SAS.02 Use `assert!` instead of `debug_assert!` to verify boundary conditions in unsafe functions
#![warn(clippy::debug_assert_with_mut_call)]

mod application;
mod authn;
mod authz;
mod cert;
mod config;
mod organization;
mod provider;
mod sdk;
mod user;
pub mod utils;

pub use application::*;
pub use authn::*;
pub use authz::*;
#[cfg(feature = "api")]
pub use casdoor_api::{apis, models as api_models};
pub use cert::*;
pub use config::*;
pub use organization::*;
pub use provider::*;
pub use reqwest::{Method, StatusCode, Url};
pub use sdk::*;
pub use user::*;

pub type SdkResult<T> = std::result::Result<T, SdkError>;

#[cfg(test)]
mod tests {
    use crate::*;
    #[test]
    fn example() {
        let endpoint = "http://localhost:8000";
        let client_id = "0e6ad201d317fb74fe9d";
        let client_secret = "1fc847b0fdb3cb3f067c15ee383dee6213bd3fde";
        let certificate = r###"
-----BEGIN CERTIFICATE-----
MIIE+TCCAuGgAwIBAgIDAeJAMA0GCSqGSIb3DQEBCwUAMDYxHTAbBgNVBAoTFENh
c2Rvb3IgT3JnYW5pemF0aW9uMRUwEwYDVQQDEwxDYXNkb29yIENlcnQwHhcNMjEx
MDE1MDgxMTUyWhcNNDExMDE1MDgxMTUyWjA2MR0wGwYDVQQKExRDYXNkb29yIE9y
Z2FuaXphdGlvbjEVMBMGA1UEAxMMQ2FzZG9vciBDZXJ0MIICIjANBgkqhkiG9w0B
AQEFAAOCAg8AMIICCgKCAgEAsInpb5E1/ym0f1RfSDSSE8IR7y+lw+RJjI74e5ej
rq4b8zMYk7HeHCyZr/hmNEwEVXnhXu1P0mBeQ5ypp/QGo8vgEmjAETNmzkI1NjOQ
CjCYwUrasO/f/MnI1C0j13vx6mV1kHZjSrKsMhYY1vaxTEP3+VB8Hjg3MHFWrb07
uvFMCJe5W8+0rKErZCKTR8+9VB3janeBz//zQePFVh79bFZate/hLirPK0Go9P1g
OvwIoC1A3sarHTP4Qm/LQRt0rHqZFybdySpyWAQvhNaDFE7mTstRSBb/wUjNCUBD
PTSLVjC04WllSf6Nkfx0Z7KvmbPstSj+btvcqsvRAGtvdsB9h62Kptjs1Yn7GAuo
I3qt/4zoKbiURYxkQJXIvwCQsEftUuk5ew5zuPSlDRLoLByQTLbx0JqLAFNfW3g/
pzSDjgd/60d6HTmvbZni4SmjdyFhXCDb1Kn7N+xTojnfaNkwep2REV+RMc0fx4Gu
hRsnLsmkmUDeyIZ9aBL9oj11YEQfM2JZEq+RVtUx+wB4y8K/tD1bcY+IfnG5rBpw
IDpS262boq4SRSvb3Z7bB0w4ZxvOfJ/1VLoRftjPbLIf0bhfr/AeZMHpIKOXvfz4
yE+hqzi68wdF0VR9xYc/RbSAf7323OsjYnjjEgInUtRohnRgCpjIk/Mt2Kt84Kb0
wn8CAwEAAaMQMA4wDAYDVR0TAQH/BAIwADANBgkqhkiG9w0BAQsFAAOCAgEAn2lf
DKkLX+F1vKRO/5gJ+Plr8P5NKuQkmwH97b8CS2gS1phDyNgIc4/LSdzuf4Awe6ve
C06lVdWSIis8UPUPdjmT2uMPSNjwLxG3QsrimMURNwFlLTfRem/heJe0Zgur9J1M
8haawdSdJjH2RgmFoDeE2r8NVRfhbR8KnCO1ddTJKuS1N0/irHz21W4jt4rxzCvl
2nR42Fybap3O/g2JXMhNNROwZmNjgpsF7XVENCSuFO1jTywLaqjuXCg54IL7XVLG
omKNNNcc8h1FCeKj/nnbGMhodnFWKDTsJcbNmcOPNHo6ixzqMy/Hqc+mWYv7maAG
Jtevs3qgMZ8F9Qzr3HpUc6R3ZYYWDY/xxPisuKftOPZgtH979XC4mdf0WPnOBLqL
2DJ1zaBmjiGJolvb7XNVKcUfDXYw85ZTZQ5b9clI4e+6bmyWqQItlwt+Ati/uFEV
XzCj70B4lALX6xau1kLEpV9O1GERizYRz5P9NJNA7KoO5AVMp9w0DQTkt+LbXnZE
HHnWKy8xHQKZF9sR7YBPGLs/Ac6tviv5Ua15OgJ/8dLRZ/veyFfGo2yZsI+hKVU5
nCCJHBcAyFnm1hdvdwEdH33jDBjNB6ciotJZrf/3VYaIWSalADosHAgMWfXuWP+h
8XKXmzlxuHbTMQYtZPDgspS5aK+S4Q9wb8RRAYo=
-----END CERTIFICATE-----
"###;
        let org_name = "built-in";
        let app_name = "myapp";

        let sdk = Config::new(
            endpoint,
            client_id,
            client_secret,
            certificate,
            org_name,
            Some(app_name.to_owned()),
        )
        .into_sdk();
        println!("{:?}", sdk);
        println!("{:?}", sdk.authn());
    }
}
