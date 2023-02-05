use std::collections::HashSet;
use crate::include::address_type::{StripeAddr, StripeId, StripeLoc};

use mockall::*;
use mockall::predicate::*;

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