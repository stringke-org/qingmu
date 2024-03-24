use std::rc::Rc;
use std::sync::{Arc, Mutex};

use deno_runtime::deno_broadcast_channel::InMemoryBroadcastChannel;
use deno_runtime::deno_core::{located_script_name, ModuleLoader, ModuleSpecifier};
use deno_runtime::permissions::PermissionsContainer;
use deno_runtime::worker::{MainWorker, WorkerOptions};
use deno_runtime::{BootstrapOptions, WorkerLogLevel};
use tokio::task;

use crate::channel::EventSender;
use crate::loader::TsModuleLoader;
use crate::ops::plugin_runtime;

async fn create_worker(
    package: String,
    sender: EventSender,
    entrypoint: ModuleSpecifier,
) -> Arc<Mutex<MainWorker>> {
    let bootstrap_options = BootstrapOptions {
        args: vec![],
        is_tty: false,
        cpu_count: 1,
        enable_testing_features: false,
        locale: deno_runtime::deno_core::v8::icu::get_language_tag(),
        location: None,
        log_level: WorkerLogLevel::Info,
        no_color: false,
        unstable: true,
        ..Default::default()
    };

    let worker_options = WorkerOptions {
        bootstrap: bootstrap_options,
        module_loader: Rc::new(TsModuleLoader),
        broadcast_channel: InMemoryBroadcastChannel::default(),
        fs: Arc::new(deno_runtime::deno_fs::RealFs),
        should_wait_for_inspector_session: false,
        extensions: vec![plugin_runtime::init_ops_and_esm(
            package.clone(),
            sender.clone(),
        )],
        ..Default::default()
    };

    let worker = MainWorker::bootstrap_from_options(
        entrypoint.clone(),
        PermissionsContainer::allow_all(),
        worker_options,
    );
    let worker_shared = Arc::new(Mutex::new(worker));
    let worker_clone = worker_shared.clone();

    let error_handler = |err| {
        // TO-DO: 显示一个带有错误的小窗口
        println!("Worker error: {:?}", err);
        std::process::exit(1);
    };

    let local = task::LocalSet::new();

    local
        .run_until(async move {
            let mut worker = worker_clone.lock().unwrap();
            worker
                .execute_main_module(&entrypoint)
                .await
                .unwrap_or_else(error_handler);
            worker
                .dispatch_load_event(&located_script_name!())
                .unwrap_or_else(error_handler);
            worker
                .run_event_loop(true)
                .await
                .unwrap_or_else(error_handler);
            worker
                .dispatch_load_event(&located_script_name!())
                .unwrap_or_else(error_handler);
        })
        .await;

    worker_shared
}

pub struct Runtime {
    pub sender: EventSender,
    pub package: String,
    pub worker: Arc<Mutex<MainWorker>>,
}

impl Runtime {
    pub fn new(sender: EventSender, package: String, worker: Arc<Mutex<MainWorker>>) -> Self {
        Self {
            sender,
            package,
            worker,
        }
    }
}
