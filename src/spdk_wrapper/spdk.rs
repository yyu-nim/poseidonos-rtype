#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use std::borrow::Borrow;
use std::mem::size_of;
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread::JoinHandle;
use lazy_static::lazy_static;
use log::{error, info, warn};
use std::thread;
use std::time::Duration;

lazy_static!{
    static ref spdkInitialized : AtomicBool = AtomicBool::new(false);
}

pub struct Spdk {
    spdkThread: Option<JoinHandle<()>>,
}

impl Spdk {
    pub fn new() -> Spdk {
        Spdk {
            spdkThread: None
        }
    }

    pub fn Init(&mut self, args: Vec<&'static str>) -> bool {
        if spdkInitialized.load(Ordering::Relaxed) {
            warn!("SPDK is already initialized");
            return true;
        }
        self.spdkThread = Some(thread::spawn(move || {
            // _InitWorker
            let mut opts = spdk_clib::spdk_app_opts::default();
            spdk_clib::spdk_app_opts_init(&mut opts, size_of::<spdk_clib::spdk_app_opts>());
            {
                opts.name = "ibof_nvmf";
                opts.mem_channel = -1;
                opts.print_level = spdk_clib::spdk_log_level::SPDK_LOG_INFO;
                opts.reactor_mask = "TODO";
                opts.main_core = 0; // TODO
            }
            let empty_option = spdk_clib::option::default();
            let parse_callback = 0 as *mut i32; // TODO
            let usage_callback = 0 as *mut i32; // TODO
            let rc = spdk_clib::spdk_app_parse_args(args.len() as i32,
                                                    args.clone(), &opts, "", empty_option,
                parse_callback, usage_callback);
            if rc != spdk_clib::spdk_app_parse_args_rvals::SPDK_APP_PARSE_ARGS_SUCCESS
            {
                error!("failed to parse spdk args: {:?}, error: {:?}", args, rc);
                std::process::exit(rc as i32);
            }
            /* Blocks until the application is exiting */
            let started_callback = 0 as *mut i32; // TODO
            let rc = spdk_clib::spdk_app_start(&opts, started_callback /*Spdk::_AppStartedCallback*/, 0 as *mut i32);
            info!("spdk_app_start result = {}", rc);
        }));
        while !spdkInitialized.load(Ordering::Relaxed) {
            thread::sleep(Duration::from_secs(1)); // 원래는 1 us 단위였는데, 1 s 도 괜찮다고 봄.
            info!("waiting for spdk initialization...");
        }
        true
    }

    pub fn Finalize() {
        // TODO
    }
}

// TODO: cfg로 linux profile vs. macos (windows) profile 만들어서 전자의 경우는 lib link, 후자의 경우는 stub
pub mod spdk_clib {
    type spdk_msg_fn = *mut i32;
    type void_ptr = *mut i32;
    type int_ptr = *mut i32;

    pub(crate) struct spdk_app_opts {
        pub name: &'static str,
        pub reactor_mask: &'static str,
        pub mem_channel: i32,
        pub main_core: i32,
        pub print_level: spdk_log_level,
    }
    impl Default for spdk_app_opts {
        fn default() -> Self {
            spdk_app_opts {
                name: "",
                reactor_mask: "",
                mem_channel: 0,
                main_core: 0,
                print_level: spdk_log_level::SPDK_LOG_DISABLED
            }
        }
    }
    pub(crate) struct option {
        pub name: &'static str,
        pub has_arg: i32,
        pub flag: int_ptr,
        pub val: i32,
    }
    impl Default for option {
        fn default() -> Self {
            option {
                name: "",
                has_arg: 0,
                flag: 0 as *mut i32,
                val: 0
            }
        }
    }
    #[derive(Debug, PartialEq)]
    pub(crate) enum spdk_app_parse_args_rvals {
        SPDK_APP_PARSE_ARGS_HELP = 0,
        SPDK_APP_PARSE_ARGS_SUCCESS = 1,
        SPDK_APP_PARSE_ARGS_FAIL = 2
    }
    pub(crate) enum spdk_log_level {
        SPDK_LOG_DISABLED = -1,
        SPDK_LOG_ERROR,
        SPDK_LOG_WARN,
        SPDK_LOG_NOTICE,
        SPDK_LOG_INFO,
        SPDK_LOG_DEBUG,
    }

    /* void spdk_app_opts_init(struct spdk_app_opts *opts, size_t opts_size); */
    pub(crate) fn spdk_app_opts_init(opts: &mut spdk_app_opts, opts_size: usize) {
        // TODO: opts는 c-compatible 하게 pointer type으로 변경해야 할 것.
    }

    /* spdk_app_parse_args_rvals_t spdk_app_parse_args(int argc, char **argv,
        struct spdk_app_opts *opts, const char *getopt_str,
        struct option *app_long_opts,
        int (*parse)(int ch, char *arg),
        void (*usage)(void)); */
    pub(crate) fn spdk_app_parse_args(argc: i32, argv: Vec<&str>, opts: &spdk_app_opts, getopt_str: &str,
                                      _app_long_opts: option, parse: *mut i32, usage: *mut i32) -> spdk_app_parse_args_rvals {
        // STUB
        spdk_app_parse_args_rvals::SPDK_APP_PARSE_ARGS_SUCCESS
    }

    /* int spdk_app_start(struct spdk_app_opts *opts_user, spdk_msg_fn start_fn, void *ctx); */
    pub(crate) fn spdk_app_start(opts_user: &spdk_app_opts, start_fn: spdk_msg_fn, ctx: void_ptr) -> i32 {
        // STUB
        0
    }

    /* void spdk_app_stop(int rc); */
    fn spdk_app_stop(rc: i32) {
        // STUB
    }

    /* void spdk_app_fini(void); */
    fn spdk_app_fini() {
        // STUB
    }

    // TODO: 나중에 linux profile추가할 때 bindgen 으로 header 생성해서, 손 좀 더 볼 것. 지금은 일단 컴파일만 되도록.
}