wit_bindgen::generate!({
    world: "host",
    path: "../../wit/host.wit",
});

struct Host;

impl Guest for Host {
    //fn get_info() -> Meta {
    //    Meta {
    //        none: 3,
    //        id: String::from("template"),
    //        //name: "Template".to_string(),
    //        //version: "0.1.0".to_string(),
    //        //author: "LeviLovie".to_string(),
    //        //description: "A template mod".to_string(),
    //    }
    //}

    fn info() {
        print("Info");
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
