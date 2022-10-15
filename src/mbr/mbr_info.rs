pub struct ArrayBootRecord
{
    pub arrayName: String,

    /* TODO
    char arrayName[ARRAY_NAME_SIZE];
    unsigned int abrVersion;
    uint32_t pad0[ABR_PADDING_0_SIZE];
    char metaRaidType[META_RAID_TYPE_SIZE];
    char dataRaidType[DATA_RAID_TYPE_SIZE];
    unsigned int totalDevNum;
    unsigned int nvmDevNum;
    unsigned int dataDevNum;
    unsigned int spareDevNum;
    unsigned int mfsInit;
    char createDatetime[DATE_SIZE];
    char updateDatetime[DATE_SIZE];
    unsigned int uniqueId;
    uint32_t pad1[ABR_PADDING_1_NUM];
    struct deviceInfo devInfo[MAX_ARRAY_DEVICE_CNT];
    uint32_t reserved[ABR_RESERVED_NUM];
    */
}