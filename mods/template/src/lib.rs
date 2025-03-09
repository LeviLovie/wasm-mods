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

    fn info() {}

    //fn on_init() {}
    //
    //fn on_update() {}
    //
    //fn on_shutdown() {}
}

export!(Host);
