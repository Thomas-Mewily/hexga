use super::*;

// A file system that can store at most a single file
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct FsFile<'a>
{
    pub path: Option<Path>,
    pub data: Cow<'a, [u8]>,
}
impl<'a> std::fmt::Debug for FsFile<'a>
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("FsFile").field("bytes", &"...").field("path", &self.path).finish()
    }
}

impl<'a> FsFile<'a>
{
    pub const fn new(path: Option<Path>) -> Self { Self::with_data(path, Cow::Owned(Vec::new())) }
    pub const fn with_data(path: Option<Path>, data: Cow<'a, [u8]>) -> Self { Self { data, path }}
    pub const fn lambda_with_data(data: Cow<'a, [u8]>) -> Self { Self::with_data(Some(Path::empty()), data) }

    fn path_match(&self, path: &path) -> IoResult
    {
        if self.path.is_none() || self.path.as_ref().unwrap() != path { Err(IoError::NotFound) } else { Ok(()) }
    }
}

impl<'a> FsWrite for FsFile<'a>
{
    fn write_bytes(&mut self, path: &path, bytes: &[u8]) ->  IoResult
    {
        self.path_match(path)?;

        match &mut self.data
        {
            Cow::Owned(v) => {
                v.clear();
                v.extend_from_slice(bytes);
            }
            Cow::Borrowed(_) => {
                self.data = Cow::Owned(bytes.to_vec());
            }
        }
        Ok(())
    }

    fn delete(&mut self, path: &path) -> IoResult
    {
        self.path_match(path)?;

        self.path = None;
        match &mut self.data
        {
            Cow::Borrowed(_) => { self.data = Cow::Owned(Vec::new()) },
            Cow::Owned(o) => { o.clear(); }
        }
        Ok(())
    }

    fn move_to(&mut self, path: &path, new_path: &path) -> IoResult {
        self.path_match(path)?;
        self.path = Some(new_path.to_owned());
        Ok(())
    }
}


impl<'a> FsRead for FsFile<'a>
{
    fn read_bytes<'b>(&'b mut self, path: &path) -> IoResult<Cow<'b, [u8]>> {
        self.path_match(path)?;

        Ok(Cow::Borrowed(&self.data))
    }

    fn entries_names(&mut self, path: &path) -> Vec<String>
    {
        if self.path_match(path).is_ok()
        {
            vec![self.path.as_ref().unwrap().to_owned().into()]
        }else
        {
            Vec::new()
        }
    }

    fn node_kind(&mut self, path: &path) -> IoResult<FsNodeKind> {
        self.path_match(path)?;
        Ok(FsNodeKind::File)
    }
}
