use lazy_static::lazy_static;
use crate::io::general_io::io_submit_handler::IOSubmitHandler;
lazy_static!{
    pub static ref instance : Box<dyn IIOSubmitHandler> = {
        Box::new(IOSubmitHandler::new())
    };
}

pub trait IIOSubmitHandler : Sync {

}
