//wit_bindgen::generate!({
//    world: "host",
//    path: "../../wit/host.wit",
//});
//
//struct Host;
//
//impl Guest for Host {
//    fn info() -> Meta {
//        print("Info");
//        let id_str = "test";
//
//        // Convert to bytes
//        let bytes = id_str.as_bytes().to_vec();
//        let length = bytes.len() as u32;
//
//        Meta {
//            none: 2,
//            id: StringData { bytes, length },
//        }
//    }
//
//    fn init() {
//        print("Init");
//    }
//
//    fn update() {
//        print("Update");
//    }
//
//    fn shutdown() {
//        print("Shutdown");
//    }
//}
//
//export!(Host);
