wit_bindgen::generate!({
    world: "host",
    path: "../../wit/host.wit",
});

struct Host;

impl Guest for Host {
    fn run() -> u32 {
        print("Hello, world!");
        42
    }
}

export!(Host);
