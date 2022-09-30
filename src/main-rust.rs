#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use poseidonos_rtype::include::pos_event_id::PosEventId;
use poseidonos_rtype::main::poseidonos::Poseidonos;

#[macro_use]
extern crate log;

fn main() {
    if let Ok(_) = std::env::var("RUST_LOG") {
        // do nothing
    } else {
        // 원래는 cmd line 인자로 바로 넘겨주고 싶은데, 윈도에서 환경 변수 세팅이 다른 것 같아 임시 코드 넣음.
        std::env::set_var("RUST_LOG", "INFO");
    }
    env_logger::init();

    info!("Hello, PoseidonOS R-type!");

    PreventDualExecution(1);
    let ret = CheckPrivileges();
    match ret {
        0 => { /* has enough privileges */ },
        error_code => {
            std::process::exit(error_code);
        },
    };
    let pos = Poseidonos;
    let ret = pos.Init(Vec::new());
    match ret {
        PosEventId::SUCCESS => {},
        error_code => {
            eprintln!("failed to initialize pos {:?}", error_code);
            std::process::exit(6 /*ENXIO*/);
        },
    };

    pos.Run();
    pos.Terminate();

}

fn PreventDualExecution(_nrProc: u32) {
    // TODO
}

fn CheckPrivileges() -> i32 {
    // TODO
    0
}