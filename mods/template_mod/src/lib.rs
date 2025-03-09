wit_bindgen::generate!({
    world: "host",
    path: "../../wit/host.wit",
});

struct Host;

impl Guest for Host {
    fn info() -> Meta {
        print("Info");
        Meta { none: 2 }
    }

    fn init() {
        print("Init");
    }

    fn update() {
        print("Update");
    }

    fn shutdown() {
        print("Shutdown");
    }
}

export!(Host);
