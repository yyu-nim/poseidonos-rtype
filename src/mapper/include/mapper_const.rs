use atomic_enum::atomic_enum;

#[atomic_enum]
#[derive(PartialEq)]
pub enum MapFlushState {
    FLUSHING,
    FLUSH_DONE,
}

#[atomic_enum]
#[derive(PartialEq)]
pub enum MapLoadState {
    NOT_LOADED,
    LOADING,
    LOAD_DONE,
}