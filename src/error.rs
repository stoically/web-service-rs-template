//! Error types.

/// Error.
//
// TODO: Better integration of `thiserror` into `error-stack` might be nice, or
// similar features in `error-stack`. See
// <https://github.com/hashintel/hash/discussions/804>.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Axum")]
    Axum,

    #[error("Hyper")]
    Hyper,

    #[error("AddrParse")]
    AddrParse,
}
