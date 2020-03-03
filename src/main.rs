fn main() {
    match cistern_two::main_wrapper() {
        Ok(_) => {}
        Err(e) => {
            panic!("cistern has encountered an error: {:?}", e);
        }
    }
}
