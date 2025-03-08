use wasmtime_mods::run;

fn main() {
    match run() {
        Ok(_) => (),
        Err(e) => eprintln!("Error: {:?}", e),
    }
}
