pub use crate::
{
    Io,

    IoError,IoErrorKind,IoResult,
    IoSave,IoSaveFrom,IoSaveResult,
    IoLoad,IoLoadFrom,IoLoadResult,

    fs::
    {
        IoFsCore,IoFsWrite,IoFsRead,
        IoFsDisk,LoadToDisk,SaveToDisk
    }
};

pub use crate::markup_extension::*;

#[cfg(feature = "derive")]
pub use hexga_io_derive::*;