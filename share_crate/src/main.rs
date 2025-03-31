use std::process::Command;
use std::env;

fn publish_crate(name: &'static str) 
{
    println!("Publishing {}...", name);
    
    if let Err(e) = env::set_current_dir(format!("hexga/{}", name)) {
        eprintln!("Failed to change directory: {}", e);
        return;
    }
    
    let status = Command::new("cargo")
        .arg("publish")
        .status()
        .expect("Failed to execute cargo publish");
    
    if !status.success() {
        eprintln!("Failed to publish {}", name);
    }
    
    if let Err(e) = env::set_current_dir("../..") {
        eprintln!("Failed to revert directory: {}", e);
    }
    
    println!("Done publishing {}", name);
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
    ];
    
    for &crate_name in &crates {
        publish_crate(crate_name);
    }
}