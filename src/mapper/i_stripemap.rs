use std::collections::HashSet;
use crate::include::address_type::{StripeAddr, StripeId, StripeLoc};

use mockall::*;
use mockall::predicate::*;
use crate::mapper::i_stripemap::default::DefaultStripeMap;

#[automock]
pub trait IStripeMap : Send + Sync {
    fn GetLSA(&self, vsid: StripeId) -> StripeAddr;
    fn GetLSAandReferLsid(&self, vsid: StripeId) -> (StripeAddr, bool);
    fn GetRandomLsid(&self, vsid: StripeId) -> StripeId;
    fn SetLSA(&self, vsid: StripeId, lsid: StripeId, loc: StripeLoc) -> i32;
    fn IsInUserDataArea(&self, entry: StripeAddr) -> bool;
    fn IsInWriteBufferArea(&self, entry: StripeAddr) -> bool;
    fn GetDirtyStripeMapPages(&self, vsid: i32) -> HashSet<u64>;
}

pub fn boxed_default() -> Box<dyn IStripeMap> {
    Box::new(DefaultStripeMap)
}

pub mod default {
    use std::collections::HashSet;
    use crate::include::address_type::{StripeAddr, StripeId, StripeLoc};
    use crate::mapper::i_stripemap::IStripeMap;

    // This is to be on par with pos-cpp's life cycle management of a few objects
    // like stripe map, wb stripe manager, .. and etc, which is initialized
    // long after its creation
    pub struct DefaultStripeMap;
    impl IStripeMap for DefaultStripeMap {
        fn GetLSA(&self, vsid: StripeId) -> StripeAddr {
            StripeAddr {
                stripe_loc: StripeLoc::IN_USER_AREA,
                stripe_id: 0
            }
        }

        fn GetLSAandReferLsid(&self, vsid: StripeId) -> (StripeAddr, bool) {
            (StripeAddr {
                stripe_loc: StripeLoc::IN_USER_AREA,
                stripe_id: 0
            }, false)
        }

        fn GetRandomLsid(&self, vsid: StripeId) -> StripeId {
            0
        }

        fn SetLSA(&self, vsid: StripeId, lsid: StripeId, loc: StripeLoc) -> i32 {
            0
        }

        fn IsInUserDataArea(&self, entry: StripeAddr) -> bool {
            false
        }

        fn IsInWriteBufferArea(&self, entry: StripeAddr) -> bool {
            false
        }

        fn GetDirtyStripeMapPages(&self, vsid: i32) -> HashSet<u64> {
            Default::default()
        }
    }

}