use strum_macros::Display;

#[derive(Display, Clone, Debug, Eq, Hash, PartialEq)]
pub enum PartitionType
{
    META_NVM,
    WRITE_BUFFER,
    META_SSD,
    USER_DATA,
    JOURNAL_SSD,
    TYPE_COUNT
}