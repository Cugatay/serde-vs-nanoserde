use rand::{distributions::Alphanumeric, Rng};
use std::{
    fs::{remove_file, File},
    io::{ErrorKind, Write},
};

enum WriteType {
    Serde,
    NanoSerde,
}

fn main() {
    write_to(WriteType::Serde);
    write_to(WriteType::NanoSerde);
}

fn write_to(write_type: WriteType) {
    let function_count = 10_000;

    let file_name = match write_type {
        WriteType::Serde => "../with-serde/src/main.rs",
        WriteType::NanoSerde => "../with-nanoserde/src/main.rs",
    };

    let write_str = function_creator(write_type, function_count);

    if let Err(e) = remove_file(file_name) {
        match e.kind() {
            ErrorKind::NotFound => {}
            _ => panic!("Other than NotFound error on remove_file command"),
        }
    }

    let mut file = File::create_new(file_name).expect("Couldn't create new file");

    file.write_all(write_str.as_bytes()).unwrap();
}

fn random_str(len: usize) -> String {
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(len)
        .map(char::from)
        .collect()
}

fn function_creator(write_type: WriteType, function_count: usize) -> String {
    let mut return_str;
    let derive_str;

    match write_type {
        WriteType::Serde => {
            return_str = String::from(
                "use serde::{{Serialize, Deserialize}};
        ",
            );
            derive_str = "#[derive(Serialize, Deserialize)]";
        }
        WriteType::NanoSerde => {
            return_str = String::from(
                "use nanoserde::{DeJson, SerJson};
        ",
            );
            derive_str = "#[derive(DeJson, SerJson)]";
        }
    };

    let mut main_fn_str = String::from(
        "
        fn main() {
        ",
    );

    for _ in 0..function_count {
        let struct_name: String = random_str(15).chars().filter(|c| !c.is_numeric()).collect();
        let struct_field_name: String =
            random_str(15).chars().filter(|c| !c.is_numeric()).collect();
        let struct_field_value = random_str(30);

        let func_str = format!(
            "
{derive_str}
struct {struct_name} {{
    {struct_field_name}: String,
}}

fn use_{struct_name}() {{
    let created_{struct_name} = {struct_name} {{
        {struct_field_name}: \"{struct_field_value}\".to_string(),
    }};

    println!(\"{{}}\", created_{struct_name}.{struct_field_name});
}}
            "
        );

        return_str += func_str.as_str();

        main_fn_str += format!(
            "
use_{struct_name}();
            "
        )
        .as_str();
    }

    main_fn_str += "\n}";

    return_str += main_fn_str.as_str();

    return_str
}
