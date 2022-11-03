use std::path::PathBuf;

fn clean_migrations() {
  let out_dir = PathBuf::from(std::env::var("OUT_DIR").unwrap());
  // TODO: far from ideal, but there's no other way to get the target dir, see <https://github.com/rust-lang/cargo/issues/5457>
  let migrations = out_dir
    .parent()
    .unwrap()
    .parent()
    .unwrap()
    .parent()
    .unwrap()
    .join("migrations");
    
    std::fs::remove_dir_all(&migrations).ok();
}
fn main() {
    clean_migrations();
    println!("cargo:rerun-if-changed=migrations");
    tauri_build::build()
}
