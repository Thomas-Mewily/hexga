use std::env;
use std::process::Command;
use std::time::Duration;

pub fn publish_all_crate()
{
    let crates: Vec<&str> = include_str!("../name_2_share.md").lines().collect();
    for name in crates
    {
        let name = name.trim();
        if name.starts_with("//") | name.starts_with("#") || name.is_empty()
        {
            continue;
        }
        publish_crate(name);
    }
}

pub fn publish_crate(name: &'static str)
{
    const CRATES_FOLDER: [&'static str; 3] = ["crates", "experimental", "old"];

    println!("Publishing {}...", name);

    // Find which folder contains this crate
    let crate_path = CRATES_FOLDER
        .iter()
        .map(|folder| format!("{}/{}", folder, name))
        .find(|path| std::path::Path::new(path).exists())
        .unwrap_or_else(|| panic!("Crate '{}' not found in any crates subfolder", name));

    env::set_current_dir(&crate_path).unwrap();

    let status = Command::new("cargo").arg("publish").status().expect("Failed to execute cargo publish");

    if !status.success()
    {
        eprintln!("Failed to publish {}", name);
    }

    env::set_current_dir("../..").unwrap();
    println!("Done publishing {}", name);
    println!();
    println!();

    std::thread::sleep(Duration::from_millis(500));
}
