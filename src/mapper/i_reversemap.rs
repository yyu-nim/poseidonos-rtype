pub trait IReverseMap : Send + Sync {

}


// class IReverseMap
// {
// public:
// virtual int Load(ReverseMapPack* rev, StripeId wblsid, StripeId vsid, EventSmartPtr cb) = 0;
// virtual int UpdateReverseMapEntry(ReverseMapPack* rev, StripeId wblsid, uint64_t offset, BlkAddr rba, uint32_t volumeId) = 0;
// virtual std::tuple<BlkAddr, uint32_t> GetReverseMapEntry(ReverseMapPack* rev, StripeId wblsid, uint64_t offset) = 0;
// virtual ReverseMapPack* Assign(StripeId wblsid, StripeId vsid) = 0;
// virtual ReverseMapPack* AllocReverseMapPack(uint32_t vsid) = 0;
// virtual int ReconstructReverseMap(uint32_t volumeId, uint64_t totalRba, uint32_t wblsid, uint32_t vsid, uint64_t blockCount, std::map<uint64_t, BlkAddr> revMapInfos) = 0;
// virtual int Flush(ReverseMapPack* rev, StripeId wblsid, Stripe* stripe, StripeId vsid, EventSmartPtr cb) = 0;
// };
