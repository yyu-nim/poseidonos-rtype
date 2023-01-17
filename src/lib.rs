#![allow(nonstandard_style, unused_variables, unused_imports, non_snake_case, non_camel_case_types)]
// TODO: 위와 같은 허용을 하는 것이 code quality 떨어뜨릴 우려가 있으므로, 추후 삭제를 한다.
// 현재는 stub 들이 많아, warning이 너무 많아 error 찾기가 힘드므로 일시적으로 허용한다.

pub mod include;
pub mod main;
pub mod spdk_wrapper;
pub mod network;
pub mod master_context;
pub mod helper;
pub mod io;
pub mod qos;
pub mod event_scheduler;
pub mod device;
pub mod io_scheduler;
pub mod metafs;
pub mod io_submit_interface;
pub mod bio;
pub mod array_mgmt;
pub mod array_models;
pub mod array_components;
pub mod array;
pub mod state;
pub mod volume;
pub mod sys_event;
pub mod mbr;
pub mod metadata;
pub mod allocator;
pub mod journal_manager;
pub mod gc;
pub mod allocator_service;
pub mod mapper;

// This is to avoid ambiguity due to src/lib.rs
#[path="./lib/mod.rs"]
pub mod lib;

// FFI bindings for SPDK
pub mod generated;

#[macro_use]
extern crate static_assertions;
