use crate::io_submit_interface::i_io_submit_handler::IIOSubmitHandler;

pub struct IOSubmitHandler;

impl IOSubmitHandler {
    pub fn new() -> IOSubmitHandler {
        IOSubmitHandler
    }
}

impl IIOSubmitHandler for IOSubmitHandler {

}