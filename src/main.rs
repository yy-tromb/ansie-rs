fn main() {
    std::env::args().skip(1).for_each(|e| print!("\x1b[{}m",e));
}
