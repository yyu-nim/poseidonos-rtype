#[derive(PartialEq)]
pub enum BackendEvent {
    BackendEvent_FrontendIO,
    BackendEvent_Flush,
    BackendEvent_GC,
    BackendEvent_UserdataRebuild,
    BackendEvent_MetadataRebuild,
    BackendEvent_JournalIO,
    BackendEvent_MetaIO,
    BackendEvent_FlushMap,
    BackendEvent_Unknown,
}