pub trait VolumeEvent {

    // Default implementation is provided
    fn Tag(&self) -> String;

    fn SetVolumeBase(&self,
                     volEventBase: &mut VolumeEventBase,
                     volId: i32, volSizeBytes: u64, volName: String,
                     uuid: String, subnqn: String, _isPrimaryRole: bool) {
        volEventBase.volId = volId;
        volEventBase.volName = volName;
        volEventBase.volSizeByte = volSizeBytes;
        volEventBase.subnqn = subnqn;
        volEventBase.uuid = uuid;
        volEventBase.isPrimaryRole = _isPrimaryRole;
    }

    fn SetVolumePerf(&self,
                     volEventPerf: &mut VolumeEventPerf,
                     maxiops: u64, maxbw: u64) {
        volEventPerf.maxbw = maxbw;
        volEventPerf.maxiops = maxiops;
    }

    fn SetVolumeArrayInfo(&self,
                          volArrayInfo: &mut VolumeArrayInfo,
                          arrayId: i32, arrayName: String) {
        volArrayInfo.arrayId = arrayId;
        volArrayInfo.arrayName = arrayName;
    }

    // Need to provide implementation
    fn VolumeCreated(&self,
                     volEventBase: VolumeEventBase,
                     volEventPerf: VolumeEventPerf,
                     volArrayInfo: VolumeArrayInfo) -> i32;
    fn VolumeUpdated(&self,
                     volEventBase: VolumeEventBase,
                     volEventPerf: VolumeEventPerf,
                     volArrayInfo: VolumeArrayInfo) -> i32;
    fn VolumeDeleted(&self,
                     volEventBase: VolumeEventBase,
                     volArrayInfo: VolumeArrayInfo) -> i32;
    fn VolumeMounted(&self,
                     volEventBase: VolumeEventBase,
                     volEventPerf: VolumeEventPerf,
                     volArrayInfo: VolumeArrayInfo) -> i32;
    fn VolumeUnmounted(&self,
                     volEventBase: VolumeEventBase,
                     volArrayInfo: VolumeArrayInfo) -> i32;
    fn VolumeLoaded(&self,
                     volEventBase: VolumeEventBase,
                     volEventPerf: VolumeEventPerf,
                     volArrayInfo: VolumeArrayInfo) -> i32;
    fn VolumeDetached(&self,
                      volList: Vec<i32>,
                      volArrayInfo: VolumeArrayInfo) -> i32;
}

pub struct VolumeEventBase
{
    volId: i32,
    volSizeByte: u64,
    volName: String,
    uuid: String,
    subnqn: String,
    isPrimaryRole: bool,
}

pub struct VolumeEventPerf
{
    maxiops: u64,
    maxbw: u64,
}

pub struct VolumeArrayInfo
{
    arrayId: i32,
    arrayName: String,
}
