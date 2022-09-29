use crate::poseidonos::Poseidonos;
mod poseidonos;

fn main() {
    println!("Hello, PoseidonOS R-type!");

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
        0 => {},
        error_code => {
            std::process::exit(error_code);
        },
    };

    pos.Run();
    pos.Terminate();

}

fn PreventDualExecution(nrProc: u32) {
    // TODO
}

fn CheckPrivileges() -> i32 {
    // TODO
    0
}