use crate::include::address_type::SegmentId;

pub struct ContextManager;

impl ContextManager {
    pub fn AllocateFreeSegment(&self) -> Option<SegmentId> {
        todo!()
    }
}

impl Default for ContextManager {
    fn default() -> Self {
        ContextManager
    }
}