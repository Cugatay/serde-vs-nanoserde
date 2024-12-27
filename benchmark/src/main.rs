use rand::{distributions::Alphanumeric, Rng};
use std::{
    fs::{remove_file, File},
    io::{ErrorKind, Write},
}; // 0.8

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

    let file_name;
    let write_str;

    match write_type {
        WriteType::Serde => {
            file_name = "../with-serde/src/main.rs";
            write_str = serde_function_creator(function_count);
        }
        WriteType::NanoSerde => {
            file_name = "../with-nanoserde/src/main.rs";
            write_str = nanoserde_function_creator(function_count);
        }
    }

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

fn serde_function_creator(function_count: usize) -> String {
    let mut return_str = String::from(
        "use serde::{{Serialize, Deserialize}};
        ",
    );

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
#[derive(Serialize, Deserialize)]
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

fn nanoserde_function_creator(function_count: usize) -> String {
    let mut return_str = String::from(
        "use nanoserde::{DeJson, SerJson};
        ",
    );

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
#[derive(DeJson, SerJson)]
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
