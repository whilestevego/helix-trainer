use include_dir::{Dir, include_dir};

pub static EXERCISES: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/exercises");
