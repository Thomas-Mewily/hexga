pub use crate::
{
    Io,

    IoError,IoErrorKind,IoResult,
    IoSave,IoSaveFrom,IoSaveResult,
    IoLoad,IoLoadFrom,IoLoadResult,

    fs::
    {
        IoFs,IoFsWrite,IoFsRead,
        IoFsDisk,SaveToDisk
    }
};

#[cfg(feature = "derive")]
pub use hexga_io_derive::*;