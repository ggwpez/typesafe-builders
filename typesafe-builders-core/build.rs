fn main() {
	// Go upwards until we find README.md
	let mut path = std::env::current_dir().unwrap();
	while !path.join("README.md").exists() {
		path = path.parent().unwrap().to_path_buf();
	}
	path = path.join("README.md");

	println!("cargo:rustc-env=README_PATH={}", path.display());
}
