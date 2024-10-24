use std::sync::{Arc, OnceLock};

use wasmtime::component::Component;
use wasmtime_wasi::{ResourceTable, WasiCtx, WasiCtxBuilder, WasiView};
use wit::{DataVariant, Extension};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let wasm_file = load_wasm()?;
    let host = Arc::new(WasmHost {
        engine: wasm_engine(),
    });

    println!("loading wasm extension...");
    let (ext, store) = host.load_extension(wasm_file).await?;

    let args = vec![DataVariant::Number(2.7), DataVariant::Number(56.3)];
    println!("invoking `add` function on external wasm extension...");
    let result = ext.call_invoke_func(store, "add", &args).await??;

    println!("Result: {result:?}");

    Ok(())
}

fn load_wasm() -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    Ok(std::fs::read("./extensions/guest.wasm")?)
}

fn wasm_engine() -> wasmtime::Engine {
    static WASM_ENGINE: OnceLock<wasmtime::Engine> = OnceLock::new();

    WASM_ENGINE
        .get_or_init(|| {
            let mut config = wasmtime::Config::new();
            config.wasm_component_model(true);
            config.async_support(true);
            wasmtime::Engine::new(&config).unwrap()
        })
        .clone()
}

struct WasmHost {
    engine: wasmtime::Engine,
}
impl WasmHost {
    pub async fn load_extension(
        self: &Arc<Self>,
        wasm_bytes: Vec<u8>,
    ) -> Result<(Extension, wasmtime::Store<WasmState>), Box<dyn std::error::Error>> {
        let component = Component::from_binary(&self.engine, &wasm_bytes)?;

        let mut builder = WasiCtxBuilder::new();

        let mut store = wasmtime::Store::new(
            &self.engine,
            WasmState {
                ctx: builder.build(),
                table: ResourceTable::new(),
            },
        );

        let mut linker = wasmtime::component::Linker::<WasmState>::new(&self.engine);
        wasmtime_wasi::add_to_linker_async(&mut linker).unwrap();

        let extension = Extension::instantiate_async(&mut store, &component, &linker).await?;

        extension.call_init_extension(&mut store).await?;

        Ok((extension, store))
    }
}

struct WasmState {
    ctx: WasiCtx,
    table: ResourceTable,
}
impl WasiView for WasmState {
    fn ctx(&mut self) -> &mut WasiCtx {
        &mut self.ctx
    }
    fn table(&mut self) -> &mut ResourceTable {
        &mut self.table
    }
}

mod wit {
    wasmtime::component::bindgen!({
        async: true,
        world: "extension",
        path: "../api/wit",
    });
}
