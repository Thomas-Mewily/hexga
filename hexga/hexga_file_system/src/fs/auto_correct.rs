use std::path::PathBuf;
use super::*;

#[derive(Default)]
pub struct FsAutoCorrectExtension<Fs>
{
    inner: Fs,
}

impl<Fs> FsRead for FsAutoCorrectExtension<Fs>
    where Fs: FsRead
{
    fn read_bytes<'a>(&'a mut self, path: &path) -> FileResult<Cow<'a, [u8]>> {
        let path = path.auto_correct_extension(self);
        self.inner.read_bytes(&path)
    }

    fn node_kind(&mut self, path: &path) -> FileResult<FsNodeKind> {
        let path = path.auto_correct_extension(self);
        self.inner.node_kind(&path)
    }

    fn entries_fullname(&mut self, path: &path) -> Vec<String> {
        let path = path.auto_correct_extension(self);
        self.inner.entries_fullname(&path)
    }
}

impl<Fs> FsWrite for FsAutoCorrectExtension<Fs>
    where Fs: FsWrite
{
    fn create_directory(&mut self, path: &path) ->  FileResult {
        self.inner.create_directory(path)
    }

    fn write_bytes(&mut self, path: &path, bytes: &[u8]) ->  FileResult {
        self.inner.write_bytes(path, bytes)
    }

    fn delete(&mut self, path: &path) -> FileResult {
        let path = path.auto_correct_extension(self);
        self.inner.delete(&path)
    }

    fn move_to(&mut self, path: &path, new_path: &path) -> FileResult {
        let path = path.auto_correct_extension(self);
        let new_path = new_path.auto_correct_extension(self);
        self.inner.move_to(&path,&new_path)
    }
}

