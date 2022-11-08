fn main() {
    println!("cargo:rustc-link-arg=-Wl,-rpath=$ORIGIN:$ORIGIN/../lib:/usr/lib64/openmpi/lib");
    println!("cargo:rustc-link-search=.:/usr/lib64/openmpi/lib");
}
