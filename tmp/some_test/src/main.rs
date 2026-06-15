use std::path::PathBuf;
use hexga::prelude::*;

fn print_dir_rec(path: PathBuf) -> IoResult
{
    println!("{:?}", path);
    for p in Io.read_dir(path)?
    {
        let _ = print_dir_rec(p);
    }
    Ok(())
}

fn main()
{
    let _ = print_dir_rec("./".into());
}
