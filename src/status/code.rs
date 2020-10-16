#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum Code {
    GenericFailure = 1,
    ManifestLoadingFailure = 2,
    SrcInfoOutOfSync = 3,
}
