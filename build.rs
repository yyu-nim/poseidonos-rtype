extern crate bindgen;

use std::env;
use std::path::PathBuf;

fn main() {
    // Tell cargo to look for shared libraries in the specified directory
    // println!("cargo:rustc-link-search=/path/to/lib");

    // Tell cargo to tell rustc to link the system bzip2
    // shared library.
    // println!("cargo:rustc-link-lib=bz2"); // later link with libspdk*.a

    // Tell cargo to invalidate the built crate whenever the wrapper changes
    println!("cargo:rerun-if-changed=wrapper.h");

    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header("wrapper.h")
        .clang_arg("-Ilib/spdk-headers/include")
        // 아래 blocklist는
        // error[E0588]: packed type cannot transitively contain a `#[repr(align)]` type
        // 를 회피하기 위한 것임. pos rtype에서 직접 참조하지 않는 struct들은 굳이
        // bindings.rs에 넣지 않아도 되므로 일단 아래와 같이 처리.
        .blocklist_item("spdk_nvme_tcp_rsp")
        .blocklist_item("spdk_nvme_tcp_cmd")
        .blocklist_item("spdk_nvmf_fabric_prop_get_rsp")
        .blocklist_item("spdk_nvmf_fabric_connect_rsp")
        .blocklist_item("spdk_nvmf_fabric_connect_cmd")
        .blocklist_item("spdk_nvmf_fabric_auth_send_cmd")
        .blocklist_item("spdk_nvmf_fabric_auth_recv_cmd")
        .blocklist_item("spdk_nvme_ctrlr_data")
        .blocklist_item("spdk_nvme_health_information_page")
        .blocklist_item("spdk_nvme_sgl_descriptor")
        .blocklist_item("spdk_nvme_cmd")
        .blocklist_item("spdk_nvme_cmd__bindgen_ty_1")
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from("src/generated/");
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}