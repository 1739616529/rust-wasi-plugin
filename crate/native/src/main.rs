
use std::sync::Arc;
use wasmer::{imports, Function, Instance, Module, Store, TypedFunction};
use wasmer_wasix::http::default_http_client;
use wasmer_wasix::{default_fs_backing, WasiEnv};
use wasmer_wasix::{
    runtime::{
        module_cache::{ModuleCache, SharedCache},
        package_loader::UnsupportedPackageLoader,
        resolver::MultiSource,
        task_manager::tokio::TokioTaskManager,
    },
    virtual_net, PluggableRuntime,
};
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 基础 demo
    let _ = demo1()?;


    Ok(())
}

fn demo1() -> Result<(), Box<dyn std::error::Error>> {
    // Let's declare the Wasm module with the text representation.
    let wasm_bytes = std::fs::read("target/wasm32-wasip1/debug/plugin_test_kit.wasm")?;

    // Create a Store.
    let mut store = Store::default();

    let module = Module::new(&store, wasm_bytes)?;
    let builder = WasiEnv::builder("add");
    let rt = build_rt();
    let builder = builder.runtime(rt).fs(default_fs_backing());
    let wasi_env = builder.finalize(&mut store)?;
    let wasi_env_import_object = wasi_env.import_object(&mut store, &module)?;

    
    // Define the __add function
    let __add = Function::new_typed(&mut store, |a: i32, b: i32| -> i32 {
        a + b
    });

    let mut import_object = imports! {
        "env" => {
            "add" => __add.clone(),
            "_add" => __add.clone(),
            "__add" => __add.clone(),
            "___add" => __add.clone(),
        }
    };
    import_object.extend(&wasi_env_import_object);



    let instance = Instance::new(&mut store, &module, &import_object).unwrap();
    let dyn_f: &Function = instance.exports.get("add").unwrap();
    let f: TypedFunction<(i32, i32), i32> = dyn_f.typed(&mut store).unwrap();

    let count = f.call(&mut store, 4, 6).unwrap();
    println!("count: {}", count);

    Ok(())
}
fn build_rt() -> Arc<PluggableRuntime> {
    let dummy_loader = UnsupportedPackageLoader;
    let cache =
        SharedCache::default().with_fallback(wasmer_wasix::runtime::module_cache::in_memory());
    return Arc::new(PluggableRuntime {
        rt: Arc::new(TokioTaskManager::default()),
        networking: Arc::new(virtual_net::UnsupportedVirtualNetworking::default()),
        http_client: Some(Arc::new(default_http_client().unwrap())),
        package_loader: Arc::new(dummy_loader),
        source: Arc::new(MultiSource::new()),
        engine: None,
        module_cache: Arc::new(cache),
        tty: None,
        journals: vec![],
    });
}
