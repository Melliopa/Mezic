// src/smart_contracts/wasm_vm.rs

use wasmtime::*;

pub struct WasmVM {
    engine: Engine,
    store: Store<()>,
}

impl WasmVM {
    pub fn new() -> Self {
        let engine = Engine::default();
        let store = Store::new(&engine, ());
        WasmVM { engine, store }
    }

    pub fn execute_contract(&mut self, wasm_code: &[u8], params: &[u8]) -> Result<(), String> {
        let module = Module::new(&self.engine, wasm_code).map_err(|e| e.to_string())?;
        let instance = Instance::new(&mut self.store, &module, &[]).map_err(|e| e.to_string())?;
        let func = instance.get_typed_func::<(), (), _>(&self.store, "execute").map_err(|e| e.to_string())?;
        func.call(&mut self.store, ()).map_err(|e| e.to_string())
    }
}
