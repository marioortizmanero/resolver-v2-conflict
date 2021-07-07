#[cfg(all(feature = "async", feature = "sync"))]
compile_error! {
    "Conflict found."
}
