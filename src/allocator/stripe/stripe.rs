use crate::include::address_type::StripeId;

#[derive(Clone)]
pub struct Stripe;

impl Stripe {
    pub fn Assign(&self, vsid: StripeId, wb_lsid: StripeId, tail_array_idx: u32) -> bool {
        // TODO
        true
    }
}