fn main() {
    if let Err(e) = cistern_two::main_wrapper() {
        panic!("cistern has encountered a fatal error: {:?}", e);
    }
}
