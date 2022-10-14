use crate::array::array::Array;

pub struct MetaFs {
    arrayName_: String,
    arrayId_ : u32,
    is_array_loaded: bool,
}

impl MetaFs {

    pub fn new(array: &Array, is_array_loaded: bool) -> MetaFs {
        let arrayName_ = array.GetName();
        let arrayId_ = array.GetIndex();
        MetaFs {
            arrayName_,
            arrayId_,
            is_array_loaded,
        }
    }

}