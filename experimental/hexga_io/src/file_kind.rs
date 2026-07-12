use super::*;


#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[non_exhaustive]
pub enum FileKind
{
    File,
    Dir,
    Symlink,
}

pub type FileType = Option<FileKind>;

pub trait FileTypeExtension
{
    fn is_file(&self) -> bool;
    fn is_dir(&self) -> bool;
    fn is_symlink(&self) -> bool;
}

impl FileTypeExtension for FileKind
{
    fn is_file(&self) -> bool { matches!(self, FileKind::File) }
    fn is_dir(&self) -> bool { matches!(self, FileKind::Dir) }
    fn is_symlink(&self) -> bool { matches!(self, FileKind::Symlink) }
}
impl FileTypeExtension for Option<FileKind>
{
    fn is_file(&self) -> bool { self.as_ref().map(FileTypeExtension::is_file).unwrap_or(false) }
    fn is_dir(&self) -> bool { self.as_ref().map(FileTypeExtension::is_dir).unwrap_or(false) }
    fn is_symlink(&self) -> bool { self.as_ref().map(FileTypeExtension::is_symlink).unwrap_or(false) }
}