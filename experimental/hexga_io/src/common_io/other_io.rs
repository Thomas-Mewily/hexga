use super::*;

/// Access to the path where the exe is stored.
#[derive(Debug, Default)]
pub struct IoExe;

impl FileSystemProvider for IoExe
{
    type FileSystem = Self;
    fn file_system() -> Self::FileSystem { Self }
}
impl FileSystemIsolated for IoExe
{
    fn isolated_root(&mut self) -> PathBuf { 
        let mut current_exe = std::env::current_exe().expect("Failed to get the current exe path");
        current_exe.pop();
        current_exe
    }
    
    type HostFileSystem = IoGlobal;
    fn non_isolated_file_system() -> Self::HostFileSystem { IoGlobal }
}


/// Access the `data` folder where the exe is stored.
#[derive(Debug, Default)]
pub struct IoData;

impl FileSystemProvider for IoData
{
    type FileSystem = Self;
    fn file_system() -> Self::FileSystem { Self }
}
impl FileSystemIsolated for IoData
{
    fn isolated_root(&mut self) -> PathBuf { IoExe.isolated_root().join("data") }
    
    type HostFileSystem = IoGlobal;
    fn non_isolated_file_system() -> Self::HostFileSystem { IoGlobal }
}