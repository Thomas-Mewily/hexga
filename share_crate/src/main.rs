#![allow(dead_code)]
#![allow(unused_variables)]

mod func;
pub use func::{publish_crate, create_crate};


fn publish_all_crate()
{
    let crates : Vec<&str> = include_str!("../name_2_share.md").lines().collect();
    for name in crates
    {
        let name = name.trim();
        if name.starts_with("//") | name.starts_with("#") || name.is_empty() { continue; }
        publish_crate(name);
    }
}

fn main() 
{
    publish_all_crate();

    /* 
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
    */
    //create_crate("test_to_reserce");

    /* 
    for &crate_name in &crates {
        publish_crate(crate_name);
    }
    */
}