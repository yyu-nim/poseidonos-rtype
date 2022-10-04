use lazy_static::lazy_static;

lazy_static!{
    pub static ref AccelEngineApi : AccelEngineApiImpl = {
        AccelEngineApiImpl
    };
}

pub struct AccelEngineApiImpl;
impl AccelEngineApiImpl {
    pub fn Initialize(&self) {
        // STUB
    }
}