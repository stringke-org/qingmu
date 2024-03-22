use std::rc::Rc;
use std::sync::{Arc, Mutex};

use deno_runtime::{BootstrapOptions, WorkerLogLevel};
use deno_runtime::deno_broadcast_channel::InMemoryBroadcastChannel;
use deno_runtime::deno_core::{located_script_name, ModuleLoader, ModuleSpecifier};
use deno_runtime::permissions::PermissionsContainer;
use deno_runtime::worker::{MainWorker, WorkerOptions};
use tokio::task;

pub struct Runtime {
    // 通过 event-bus ?
}

impl Runtime {
    pub async fn create(entrypoint: ModuleSpecifier, module_loader: impl ModuleLoader + 'static) -> Arc<Mutex<MainWorker>> {
        let module_loader = Rc::new(module_loader);

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
            module_loader,
            broadcast_channel: InMemoryBroadcastChannel::default(),
            fs: Arc::new(deno_runtime::deno_fs::RealFs),
            should_wait_for_inspector_session: false,
            ..Default::default()
        };

        let mut worker = MainWorker::bootstrap_from_options(
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

        local.run_until(async move {
            let mut worker = worker_clone.lock().unwrap();
            worker.execute_main_module(&entrypoint).await.unwrap_or_else(error_handler);
            worker.dispatch_load_event(&located_script_name!()).unwrap_or_else(error_handler);
            worker.run_event_loop(true).await.unwrap_or_else(error_handler);
            worker.dispatch_load_event(&located_script_name!()).unwrap_or_else(error_handler);
        }).await;

        worker_shared
    }
}