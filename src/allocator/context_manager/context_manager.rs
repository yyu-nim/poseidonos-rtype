use crate::include::address_type::SegmentId;

/* This is a Fake at the moment */
pub struct ContextManager {
    next_segment_to_allocate: SegmentId,
}

impl ContextManager {
    // Fake
    pub fn AllocateFreeSegment(&mut self) -> Option<SegmentId> {
        let allocated = self.next_segment_to_allocate;
        self.next_segment_to_allocate += 1;
        Some(allocated)
    }

    pub fn Init(&self) {
        // TODO
    }
}

impl Default for ContextManager {
    fn default() -> Self {
        ContextManager {
            next_segment_to_allocate: 0
        }
    }
}