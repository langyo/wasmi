use std::{fs::File, io::Read as _};

/// Returns the Wasm binary at the given `file_name` as `Vec<u8>`.
///
/// # Note
///
/// This includes validation and compilation to `wasmi` bytecode.
///
/// # Panics
///
/// If the benchmark Wasm file could not be opened, read or parsed.
pub fn load_wasm_from_file(file_name: &str) -> Vec<u8> {
    let mut file = File::open(file_name)
        .unwrap_or_else(|error| panic!("could not open benchmark file {}: {}", file_name, error));
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).unwrap_or_else(|error| {
        panic!("could not read file at {} to buffer: {}", file_name, error)
    });
    buffer
}

/// Parses the Wasm binary at the given `file_name` into a `wasmi` module.
///
/// # Note
///
/// This includes validation and compilation to `wasmi` bytecode.
///
/// # Panics
///
/// If the benchmark Wasm file could not be opened, read or parsed.
pub fn load_module_from_file_v1(file_name: &str) -> wasmi::Module {
    let wasm = load_wasm_from_file(file_name);
    let engine = wasmi::Engine::default();
    wasmi::Module::new(&engine, &wasm[..]).unwrap_or_else(|error| {
        panic!(
            "could not parse Wasm module from file {}: {}",
            file_name, error
        )
    })
}

/// Parses the Wasm binary from the given `file_name` into a `wasmi` `v1` module.
///
/// # Note
///
/// This includes validation and compilation to `wasmi` bytecode.
///
/// # Panics
///
/// If the benchmark Wasm file could not be opened, read or parsed.
pub fn load_instance_from_file_v1(file_name: &str) -> (wasmi::Store<()>, wasmi::Instance) {
    let module = load_module_from_file_v1(file_name);
    let mut linker = <wasmi::Linker<()>>::default();
    let mut store = wasmi::Store::new(module.engine(), ());
    let instance = linker
        .instantiate(&mut store, &module)
        .unwrap()
        .start(&mut store)
        .unwrap();
    (store, instance)
}

/// Converts the `.wat` encoded `bytes` into `.wasm` encoded bytes.
pub fn wat2wasm(bytes: &[u8]) -> Vec<u8> {
    wat::parse_bytes(bytes).unwrap().into_owned()
}

/// Parses the Wasm source from the given `.wat` bytes into a `wasmi` `v0` module.
///
/// # Note
///
/// This includes validation and compilation to `wasmi` bytecode.
///
/// # Panics
///
/// If the benchmark Wasm file could not be opened, read or parsed.
pub fn load_instance_from_wat_v1(wat_bytes: &[u8]) -> (wasmi::Store<()>, wasmi::Instance) {
    let wasm = wat2wasm(wat_bytes);
    let engine = wasmi::Engine::default();
    let module = wasmi::Module::new(&engine, &wasm[..]).unwrap();
    let mut linker = <wasmi::Linker<()>>::default();
    let mut store = wasmi::Store::new(&engine, ());
    let instance = linker
        .instantiate(&mut store, &module)
        .unwrap()
        .start(&mut store)
        .unwrap();
    (store, instance)
}