use std::process::Command;
use std::{env, fs, path::Path};

pub fn publish_crate(name: &'static str) 
{
    println!("Publishing {}...", name);
    
    env::set_current_dir(format!("hexga/{}", name)).unwrap();
    
    let status = Command::new("cargo")
        .arg("publish")
        .status()
        .expect("Failed to execute cargo publish");
    
    if !status.success() {
        eprintln!("Failed to publish {}", name);
    }
    
    env::set_current_dir("../..").unwrap();
    println!("Done publishing {}", name);
}


pub fn create_crate(name: &'static str) 
{
    fn copy_dir(src: &Path, dst: &Path) -> std::io::Result<()> {
        for entry in fs::read_dir(src)? {
            let entry = entry?;
            let file_type = entry.file_type()?;
            let src_path = entry.path();
            let dst_path = dst.join(entry.file_name());
            
            if file_type.is_dir() {
                fs::create_dir_all(&dst_path)?;
                copy_dir(&src_path, &dst_path)?;
            } else {
                fs::copy(&src_path, &dst_path)?;
            }
        }
        Ok(())
    }

    
    let src = Path::new("./reserved/template");
    let d = format!("./reserved/{}", name);
    let dst = Path::new(&d);
    
    if let Err(e) = fs::create_dir_all(dst) {
        eprintln!("Failed to create directory {}: {}", dst.display(), e);
        return;
    }
    
    if let Err(e) = copy_dir(src, dst) {
        eprintln!("Failed to copy template: {}", e);
        return;
    }
    
    let toml_path = dst.join("Cargo.toml");
    if let Ok(content) = fs::read_to_string(&toml_path) {
        let updated_content = content.replace("name = \"template\"", &format!("name = \"{}\"", name));
        if let Err(e) = fs::write(&toml_path, updated_content) {
            eprintln!("Failed to update Cargo.toml: {}", e);
        }
    }
    
    println!("Created crate {}", name);
}

fn main() 
{
    let crates = 
    [
        "hexga_array",
        "hexga_number",
        "hexga_typedef",
        "hexga_math",
        "hexga_generational",
        "hexga_tools",
        "hexga",
    ];
    
    //create_crate("test_to_reserce");

    for &crate_name in &crates {
        publish_crate(crate_name);
    }
}