use serde::ser::Error;

use super::*;

#[derive(Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct AssetError
{
    pub path: Path,
    pub kind: IoError,
    pub childs: Vec<AssetError>
}


// TODO: impl it
impl Display for AssetError
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
    {
        writeln!(f, "Asset error at '{}': {}", self.path, self.kind)?;

        fn fmt_children(children: &[AssetError], f: &mut std::fmt::Formatter<'_>, indent: usize) -> std::fmt::Result {
            let indent_str = "  ".repeat(indent);
            for child in children {
                writeln!(f, "{}- {}: {}", indent_str, child.path, child.kind)?;
                fmt_children(&child.childs, f, indent + 1)?;
            }
            Ok(())
        }

        fmt_children(&self.childs, f, 1)
    }
}

// TODO: impl it
impl std::error::Error for AssetError
{
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)>
    {
        None
    }
    fn provide<'a>(&'a self, _: &mut std::error::Request<'a>) {}
}



impl serde::ser::Error for AssetError
{
    fn custom<T>(msg:T) -> Self where T:std::fmt::Display
    {
        Self { kind: IoError::custom(msg), ..Default::default() }
    }
}