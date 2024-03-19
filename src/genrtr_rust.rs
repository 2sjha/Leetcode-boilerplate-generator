use crate::generator;
use crate::parser;
use crate::utils::{self, Example};

pub fn rust_driver_code(examples: Vec<Example>, starter_code: &String) -> String {
    let mut driver_code: String = format!("struct Solution;\n{}\n\nfn main() {{\n", starter_code);
    let function_name: String = parser::parse_func_name_from_starter_code(starter_code);

    // Cant parse 1st example or create code for it
    // Then problem code/type might be too specific
    // Thus write them as comment.
    let code_example_1 = rust_code_for_example(&examples[0], 1, &function_name);
    if code_example_1.is_none() {
        eprintln!("Couldn't understand example. Writing examples as comment. You're on your own for this one.");
        driver_code += generator::examples_as_comment(examples).as_str();
    } else {
        driver_code += code_example_1.unwrap().as_str();
        for i in 1..examples.len() {
            driver_code += rust_code_for_example(&examples[i], i + 1, &function_name)
                .unwrap()
                .as_str();
        }
    }

    driver_code += "}\n";
    driver_code
}

fn rust_code_for_example(
    example: &Example,
    example_number: usize,
    function_name: &String,
) -> Option<String> {
    let mut example_string: String = String::new();

    // input_type_1 input_var_i_1 = input_val_i_1;
    // ..
    // input_type_n input_var_i_n = input_val_i_n;
    for i in 0..example.get_input_var_types().len() {
        let (input_var_name, lc_input_var_type) = &example.get_input_var_types()[i];
        let input_var_value = &example.get_input_var_values()[input_var_name];
        let code_for_input_var = rust_code_for_input_var(lc_input_var_type, &input_var_value);
        if code_for_input_var.is_none() {
            return None;
        }

        let (rust_input_var_type, rust_input_var_value) = code_for_input_var.unwrap();
        example_string += format!(
            "\tlet {}_{}: {} = {};\n",
            snake_case(&input_var_name),
            example_number,
            rust_input_var_type,
            rust_input_var_value
        )
        .as_str();
    }

    // return_type output_i = output_val_i;
    let code_for_output_var =
        rust_code_for_output_var(&example.get_output_type(), &example.get_output_value());
    if code_for_output_var.is_none() {
        return None;
    }

    let (rust_output_var_type, rust_output_var_value) = code_for_output_var.unwrap();
    example_string += format!(
        "\tlet expected_{}: {} = {};\n",
        example_number, rust_output_var_type, rust_output_var_value
    )
    .as_str();

    // return_type output_i = func_name(input_var_i_1, input_var_i_2, .. input_var_i_n);
    example_string += format!(
        "\tlet output_{}: {} = Solution::{}(",
        example_number, rust_output_var_type, function_name
    )
    .as_str();
    let mut i: usize = 0;
    while i < example.get_input_var_types().len() - 1 {
        let input_var_name_type = &example.get_input_var_types()[i];
        example_string += format!(
            "{}_{}, ",
            snake_case(&input_var_name_type.0),
            example_number
        )
        .as_str();
        i += 1;
    }
    example_string += format!(
        "{}_{});\n",
        snake_case(&example.get_input_var_types()[i].0),
        example_number
    )
    .as_str();

    // print output_i
    example_string += format!("\tprintln!(\"{{:?}}\", output_{});\n", example_number).as_str();

    // assert(output_i == expected_i);
    example_string += format!(
        "\tassert!(output_{} == expected_{});\n\n",
        example_number, example_number
    )
    .as_str();

    Some(example_string)
}

fn rust_code_for_input_var(lc_var_type: &String, var_value: &String) -> Option<(String, String)> {
    let rust_var_type: String;
    let mut rust_var_value: String;

    if lc_var_type == utils::IN_INT {
        rust_var_type = String::from("i32");
        rust_var_value = var_value.clone()
    } else if lc_var_type == utils::IN_STRING {
        rust_var_type = String::from("String");
        rust_var_value = format!("{}.to_string()", var_value);
    } else if lc_var_type == utils::IN_LIST_INT {
        rust_var_type = String::from("Vec<i32>");
        rust_var_value = String::new();
        for c in var_value.chars() {
            if c == '[' {
                rust_var_value += "vec!["
            } else {
                rust_var_value.push(c);
            }
        }
    } else if lc_var_type == utils::IN_LIST_STRING || lc_var_type == utils::IN_LIST_STRING2 {
        rust_var_type = String::from("Vec<String>");
        rust_var_value = String::new();
        for c in var_value.chars() {
            if c == '[' {
                rust_var_value += "vec!["
            } else if c == ',' {
                rust_var_value += ".to_string(),"
            } else if c == ']' {
                rust_var_value += ".to_string()]"
            } else {
                rust_var_value.push(c);
            }
        }
    } else if lc_var_type == utils::IN_MATRIX_INT {
        rust_var_type = String::from("Vec<Vec<i32>>");
        rust_var_value = String::new();
        for c in var_value.chars() {
            if c == '[' {
                rust_var_value += "vec!["
            } else {
                rust_var_value.push(c);
            }
        }
    } else if lc_var_type == utils::IN_MATRIX_CHAR {
        rust_var_type = String::from("Vec<Vec<char>>");
        rust_var_value = String::new();
        for c in var_value.chars() {
            if c == '[' {
                rust_var_value += "vec!["
            } else if c == '\"' {
                rust_var_value += "\'"
            } else {
                rust_var_value.push(c);
            }
        }
    } else if lc_var_type == utils::IN_LIST_CHAR {
        rust_var_type = String::from("Vec<char>");
        rust_var_value = String::new();
        for c in var_value.chars() {
            if c == '[' {
                rust_var_value += "vec!["
            } else if c == '\"' {
                rust_var_value += "\'"
            } else {
                rust_var_value.push(c);
            }
        }
    } else {
        return None;
    }

    Some((rust_var_type, rust_var_value))
}

fn rust_code_for_output_var(lc_var_type: &String, var_value: &String) -> Option<(String, String)> {
    let rust_var_type: String;
    let mut rust_var_value: String;

    if lc_var_type == utils::OUT_INT {
        rust_var_type = String::from("i32");
        rust_var_value = var_value.clone()
    } else if lc_var_type == utils::OUT_BOOL {
        rust_var_type = String::from("bool");
        rust_var_value = var_value.clone();
    } else if lc_var_type == utils::OUT_VOID {
        rust_var_type = String::from("()");
        rust_var_value = var_value.clone();
    } else if lc_var_type == utils::OUT_STRING {
        rust_var_type = String::from("String");
        rust_var_value = format!("String::from({})", var_value);
    } else if lc_var_type == utils::OUT_LIST_INT || lc_var_type == utils::OUT_LIST_INT2 {
        rust_var_type = String::from("Vec<i32>");
        rust_var_value = String::new();
        for c in var_value.chars() {
            if c == '[' {
                rust_var_value += "vec!["
            } else {
                rust_var_value.push(c);
            }
        }
    } else if lc_var_type == utils::OUT_LIST_STRING {
        rust_var_type = String::from("Vec<String>");
        rust_var_value = String::new();
        for c in var_value.chars() {
            if c == '[' {
                rust_var_value += "vec!["
            } else if c == ',' {
                rust_var_value += ".to_string(),"
            } else if c == ']' {
                rust_var_value += ".to_string()]"
            } else {
                rust_var_value.push(c);
            }
        }
    } else if lc_var_type == utils::OUT_MATRIX_INT || lc_var_type == utils::OUT_MATRIX_INT2 {
        rust_var_type = String::from("Vec<Vec<i32>>");
        rust_var_value = String::new();
        for c in var_value.chars() {
            if c == '[' {
                rust_var_value += "vec!["
            } else {
                rust_var_value.push(c);
            }
        }
    } else {
        return None;
    }

    Some((rust_var_type, rust_var_value))
}

fn snake_case(var_name: &String) -> String {
    let mut sc_var_name: String = String::new();
    for c in var_name.chars() {
        if c.is_ascii_uppercase() {
            sc_var_name += "_";
            sc_var_name.push(c.to_lowercase().next().unwrap());
        } else {
            sc_var_name.push(c);
        }
    }

    sc_var_name
}
