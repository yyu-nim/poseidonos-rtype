use crate::allocator::allocator::Allocator;
use crate::array_models::interface::i_array_info::ArrayInfo;
use crate::journal_manager::journal_manager::JournalManager;
use crate::state::interface::i_state_control::IStateControl;

pub struct Metadata {
    arrayInfo: ArrayInfo,
    allocator: Allocator,
    journalManager: JournalManager,
}

impl Metadata {
    pub fn new(arrayInfo: ArrayInfo, state: &Box<dyn IStateControl>) -> Metadata {
        let allocator = Allocator::new(arrayInfo.clone()); // IStateControl은 사용 안되는 것 같아 생략
        let journalManager = JournalManager::new(arrayInfo.clone()); // IArrayInfo* 는 추후에 넘겨줄 것.
        // TODO: MetaFsServiceSingleton/MetaServiceSingleton injection

        Metadata {
            arrayInfo,
            allocator,
            journalManager,
        }
    }
}