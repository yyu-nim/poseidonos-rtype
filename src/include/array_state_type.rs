pub enum ArrayStateEnum {
    NOT_EXIST = 0,
    EXIST_NORMAL,
    EXIST_DEGRADED,
    BROKEN,
    TRY_MOUNT,
    TRY_UNMOUNT,
    NORMAL,
    DEGRADED,
    REBUILD,
    TYPE_COUNT
}