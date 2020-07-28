const rust = import("./pkg/index");

rust.then(m => m.say_hello_from_rust())
    .catch(console.log);