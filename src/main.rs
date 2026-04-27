mod filesystem;

fn main() {
    let name: &str = "nazwa.txt";
    let inside = b"test dir/name";
    filesystem::add_file("upload", name, inside);
}
