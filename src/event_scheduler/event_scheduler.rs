use lazy_static::lazy_static;
lazy_static!{
    pub static ref EventSchedulerSingleton : EventScheduler = {
        EventScheduler
    };
}

pub struct EventScheduler;

impl EventScheduler {

    /***
     @schedulerCpuInput: e.g., [1, 3, 5]
     @eventCpuSetInput: e.g., [6, 7]
     */
    pub fn Initialize(&self, workerCountInput: u32, schedulerCpuInput: Vec<u8>, eventCpuSetInput: Vec<u8>)
    {
        // TODO
    }

}