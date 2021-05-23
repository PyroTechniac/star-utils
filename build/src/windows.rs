use std::{
    collections::hash_map::DefaultHasher,
    env,
    fs::{metadata, write},
    hash::{Hash, Hasher},
    io::Read,
    path::PathBuf,
    process::Command,
};

fn get_node_version() -> std::io::Result<String> {
    let output = Command::new("node").arg("-v").output()?;
    let stdout_str = String::from_utf8_lossy(&output.stdout);

    Ok(stdout_str.trim().trim_start_matches('v').to_string())
}

fn download_node_lib(dist_url: &str, version: &str, arch: &str) -> Vec<u8> {
    let url = format!(
        "{dist_url}/v{version}/win-{arch}/node.lib",
        dist_url = dist_url,
        version = version,
        arch = arch
    );

    match ureq::get(&url).call() {
        Ok(response) => {
            let mut reader = response.into_reader();
            let mut bytes = vec![];
            reader.read_to_end(&mut bytes).unwrap();
            bytes
        }
        Err(err) => panic!("Failed to download node.lib: {:#?}", err),
    }
}

pub fn setup() {
    let out_dir = env::var("OUT_DIR").expect("OUT_DIR is not set");

    let dist_url =
        env::var("NPM_CONFIG_DISTURL").unwrap_or_else(|_| "https://nodejs.org/dist".to_string());

    let node_version = env::var("NPM_CONFIG_TARGET")
        .or_else(|_| get_node_version())
        .expect("Failed to determine nodejs version");

    let arch = env::var("CARGO_CFG_TARGET_ARCH")
        .map(|arch| match arch.as_str() {
            "x86" => "x86",
            "x86_64" => "x64",
            "aarch64" => "arm64",
            arch => panic!("Unsupported CPU Architecture: {}", arch),
        })
        .expect("Failed to determine target arch");

    println!("cargo:rerun-if-env-changed=NPM_CONFIG_DISTURL");
    println!("cargo:rerun-if-env-changed=NPM_CONFIG_TARGET");

    let mut node_lib_file_path = PathBuf::from(out_dir);
    let link_search_dir = node_lib_file_path.clone();
    let dist_url_hash = {
        let mut hasher = DefaultHasher::new();
        dist_url.hash(&mut hasher);
        hasher.finish()
    };

    let node_lib_file_name = format!(
        "node-{version}-{arch}-{dist_url_hash}.lib",
        version = node_version,
        arch = arch,
        dist_url_hash = dist_url_hash
    );
    node_lib_file_path.push(&node_lib_file_name);

    if metadata(&node_lib_file_path).is_err() {
        let node_lib = download_node_lib(&dist_url, &node_version, &arch);

        write(&node_lib_file_path, &node_lib).expect(&format!(
            "Could not save file to {}",
            node_lib_file_path.to_str().unwrap()
        ));
    }

    println!(
        "cargo:rustc-link-lib={}",
        node_lib_file_path.file_stem().unwrap().to_str().unwrap()
    );
    println!("cargo:rustc-link-search=native={}", link_search_dir.display());
    println!("cargo:rustc-cdylib-link-arg=delayimp.lib");
    println!("cargo:rustc-cdylib-link-arg=/DELAYLOAD:node.exe");
}
