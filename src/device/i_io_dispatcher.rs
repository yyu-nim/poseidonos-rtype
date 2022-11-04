use crate::bio::ubio::Ubio;
use crate::include::pos_event_id::PosEventId;

pub trait IIODispatcher : Send + Sync {
    // TODO
    fn Submit(&self, ubio: Ubio, sync: bool, ioRecoveryNeeded: bool) -> PosEventId;
    fn AddIOWorker(&self);
}