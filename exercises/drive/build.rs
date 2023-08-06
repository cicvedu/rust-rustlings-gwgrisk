
fn main() {


    std::env::set_var("TEST_FOO", "111");

    println!("cargo:rustc-cfg=pass");
}

