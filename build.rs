fn main() {
    // Check if npm in installed
    let npm_installed = std::process::Command::new("npm")
        .arg("--version")
        .output()
        .is_ok();

    if !npm_installed {
        panic!("npm is not installed");
    }

    // Run npm install in the frontend directory
    let status = std::process::Command::new("npm")
        .arg("install")
        .current_dir("front")
        .status()
        .expect("npm install failed");

    if !status.success() {
        panic!("npm install failed");
    }

    // Run npm run build in the frontend directory
    let status = std::process::Command::new("npm")
        .arg("run")
        .arg("build")
        .current_dir("front")
        .status()
        .expect("npm run build failed");

    if !status.success() {
        panic!("npm run build failed");            
    }

    // Copy contents from front/dist to /static directory
    let status = std::process::Command::new("cp")
        .arg("-r")
        .arg("front/dist/.")
        .arg("static/")
        .status()
        .expect("copy failed");

    if !status.success() {
        panic!("copy failed");
    }

    println!("cargo:rerun-if-changed=front");


}
