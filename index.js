const rust = import(".pkg/rust_3d_wasm");

rust.then(m => m.say_hello_from_rust())
    .catch(console.log);