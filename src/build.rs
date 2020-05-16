use wayland_scanner::{generate_code, Side};

fn main() {
    let protocol_file = "./src/idle.xml";
    let output_file = "./src/generated.rs";
    generate_code(protocol_file, output_file, Side::Client);
}
