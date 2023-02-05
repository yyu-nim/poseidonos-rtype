pub enum VolumeIoType
{
    UserRead,
    UserWrite,
    InternalIo,
    MaxVolumeIoTypeCnt
}

pub const MAX_VOLUME_COUNT: usize = 256;