#[cfg(miri)]
#[cfg(not(feature = "miri"))]
compile_error!("To use miri, use the `miri` flag");
