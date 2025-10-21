use std::fmt::Display;




#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum FsNodeKind
{
    File,
    Directoy,
}

impl Display for FsNodeKind
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self
        {
            FsNodeKind::File => f.write_str("File"),
            FsNodeKind::Directoy => f.write_str("Directoy"),
        }
    }
}

impl FsNodeKind
{
    pub const fn is_file(self) -> bool { matches!(self, Self::File) }
    pub const fn is_directory(self) -> bool { matches!(self, Self::Directoy) }
}

/*
#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub enum FileNode
{
    File,
    Directory
}

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub enum FileNodeRead
{
    File(Vec<u8>),
    Directory(Vec<Path>)
}
*/