use std::sync::{Arc, Mutex};
use crate::include::address_type::StripeId;
use crate::mapper::i_reversemap::IReverseMap;
use crate::mapper::reversemap::reverse_map::ReverseMapPack;

#[derive(Clone)]
pub struct Stripe;

impl Stripe {
    pub fn from(rev: Option<ReverseMapPack>,
                rev_map_manager: Arc<Mutex<Box<dyn IReverseMap>>>,
                with_data_buffer: bool,
                num_blks_per_stripe: u32) -> Stripe {
        // TODO
        Stripe
    }

    pub fn Assign(&self, vsid: StripeId, wb_lsid: StripeId, tail_array_idx: u32) -> bool {
        // TODO
        true
    }
}