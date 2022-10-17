use crate::allocator::allocator::Allocator;
use crate::journal_manager::journal_manager::JournalManager;
use crate::state::interface::i_state_control::IStateControl;

pub struct Metadata {
    arrayName: String,
    arrayIdx: u32,
    allocator: Allocator,
    journalManager: JournalManager,
}

impl Metadata {
    pub fn new(arrayName: String, arrayIdx: u32, state: &Box<dyn IStateControl>) -> Metadata {
        let allocator = Allocator::new(arrayName.clone(), arrayIdx); // IStateControl은 사용 안되는 것 같아 생략
        let journalManager = JournalManager::new(arrayName.clone(), arrayIdx); // IArrayInfo* 는 추후에 넘겨줄 것.
        // TODO: MetaFsServiceSingleton/MetaServiceSingleton injection

        Metadata {
            arrayName,
            arrayIdx,
            allocator,
            journalManager,
        }
    }
}