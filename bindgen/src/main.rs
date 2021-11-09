use interoptopus::util::NamespaceMappings;
use interoptopus::{Error, Interop};

fn main() {
    match bindings_csharp() {
        Ok(_) => println!("Success"),
        Err(e) => println!("Failed: {}", e)
    }
}

fn bindings_csharp() -> Result<(), Error> {
    use interoptopus_backend_csharp::{Config, Generator};
    use interoptopus_backend_csharp::overloads::{DotNet};

    let config = Config {
        dll_name: "lib_test".to_string(),
        namespace_mappings: NamespaceMappings::new("My.Company"),
        ..Config::default()
    };

    Generator::new(config, lib_test::my_inventory())
        .add_overload_writer(DotNet::new())
        .write_file("bindgen/generated/Interop.cs")?;

    std::fs::copy("target/debug/lib_test.dll", r"C:\Users\nico\source\tmp\ConsoleApplication42\ConsoleApplication42\bin\Debug\lib_test.dll").unwrap();

    Ok(())
}