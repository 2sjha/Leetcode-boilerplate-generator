use crate::generator;
use crate::parser;
use crate::utils::{self, Example};

pub fn cpp_driver_code(examples: Vec<Example>, starter_code: &String) -> String {
    let mut driver_code: String = format!("#include \"./../cpp-utils/printutils.h\"\n#include \"bits/stdc++.h\"\n#include <iostream>\nusing namespace std;\n\n{}\n\nint main() {{\n\tSolution sol;\n\n", starter_code);
    let function_name: String = parser::parse_func_name_from_starter_code(starter_code);

    let code_example_1 = cpp_code_for_example(&examples[0], 1, &function_name);
    if code_example_1.is_none() {
        eprintln!("Couldn't understand example. Writing examples as comment. You're on your own for this one.");
        driver_code += generator::examples_as_comment(examples).as_str();
    } else {
        driver_code += code_example_1.unwrap().as_str();
        for i in 1..examples.len() {
            driver_code += cpp_code_for_example(&examples[i], i + 1, &function_name)
                .unwrap()
                .as_str();
        }
    }

    driver_code += "\treturn 0;\n}";
    driver_code
}

fn cpp_code_for_example(
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
        let code_for_input_var = cpp_code_for_input_var(lc_input_var_type, &input_var_value);
        if code_for_input_var.is_none() {
            return None;
        }

        let (cpp_input_var_type, cpp_input_var_value) = code_for_input_var.unwrap();
        example_string += format!(
            "\t{} {}_{} = {};\n",
            cpp_input_var_type, input_var_name, example_number, cpp_input_var_value
        )
        .as_str();
    }

    // return_type output_i = output_val_i;
    let code_for_output_var =
        cpp_code_for_output_var(&example.get_output_type(), &example.get_output_value());
    if code_for_output_var.is_none() {
        return None;
    }

    let (cpp_output_var_type, cpp_output_var_value) = code_for_output_var.unwrap();
    example_string += format!(
        "\t{} expected_{} = {};\n",
        cpp_output_var_type, example_number, cpp_output_var_value
    )
    .as_str();

    // return_type output_i = func_name(input_var_i_1, input_var_i_2, .. input_var_i_n);
    example_string += format!(
        "\t{} output_{} = sol.{}(",
        cpp_output_var_type, example_number, function_name
    )
    .as_str();
    let mut i: usize = 0;
    while i < example.get_input_var_types().len() - 1 {
        let input_var_name_type = &example.get_input_var_types()[i];
        example_string += format!("{}_{}, ", input_var_name_type.0, example_number).as_str();
        i += 1;
    }
    example_string += format!(
        "{}_{});\n",
        &example.get_input_var_types()[i].0,
        example_number
    )
    .as_str();

    // print output_i
    example_string += print_output(&example.get_output_type(), example_number).as_str();

    // assert(output_i == expected_i);
    example_string += format!(
        "\tassert(output_{} == expected_{});\n\n",
        example_number, example_number
    )
    .as_str();

    Some(example_string)
}

fn cpp_code_for_input_var(lc_var_type: &String, var_value: &String) -> Option<(String, String)> {
    let cpp_var_type: String;
    let mut cpp_var_value: String;

    if lc_var_type == utils::IN_INT {
        cpp_var_type = String::from("int");
        cpp_var_value = var_value.clone()
    } else if lc_var_type == utils::IN_STRING {
        cpp_var_type = String::from("string");
        cpp_var_value = var_value.clone();
    } else if lc_var_type == utils::IN_LIST_INT {
        cpp_var_type = String::from("vector<int>");
        cpp_var_value = String::new();
        for c in var_value.chars() {
            if c == '[' {
                cpp_var_value.push('{')
            } else if c == ']' {
                cpp_var_value.push('}')
            } else {
                cpp_var_value.push(c);
            }
        }
    } else if lc_var_type == utils::IN_LIST_STRING {
        cpp_var_type = String::from("vector<string>");
        cpp_var_value = String::new();
        for c in var_value.chars() {
            if c == '[' {
                cpp_var_value.push('{');
            } else if c == ']' {
                cpp_var_value.push('}')
            } else {
                cpp_var_value.push(c);
            }
        }
    } else if lc_var_type == utils::IN_MATRIX_INT {
        cpp_var_type = String::from("vector<vector<int>>");
        cpp_var_value = String::new();
        for c in var_value.chars() {
            if c == '[' {
                cpp_var_value.push('{');
            } else if c == ']' {
                cpp_var_value.push('}')
            } else {
                cpp_var_value.push(c);
            }
        }
    } else if lc_var_type == utils::IN_MATRIX_CHAR {
        cpp_var_type = String::from("vector<vector<char>>");
        cpp_var_value = String::new();
        for c in var_value.chars() {
            if c == '[' {
                cpp_var_value.push('{');
            } else if c == ']' {
                cpp_var_value.push('}')
            } else if c == '\"' {
                cpp_var_value += "\'"
            } else {
                cpp_var_value.push(c);
            }
        }
    } else {
        return None;
    }

    Some((cpp_var_type, cpp_var_value))
}

fn cpp_code_for_output_var(lc_var_type: &String, var_value: &String) -> Option<(String, String)> {
    let cpp_var_type: String;
    let mut cpp_var_value: String;

    if lc_var_type == utils::OUT_INT {
        cpp_var_type = String::from("int");
        cpp_var_value = var_value.clone()
    } else if lc_var_type == utils::OUT_BOOL {
        cpp_var_type = String::from("bool");
        cpp_var_value = var_value.clone();
    } else if lc_var_type == utils::OUT_VOID {
        cpp_var_type = String::new();
        cpp_var_value = var_value.clone();
    } else if lc_var_type == utils::OUT_STRING {
        cpp_var_type = String::from("string");
        cpp_var_value = var_value.clone();
    } else if lc_var_type == utils::OUT_LIST_INT
        || lc_var_type == utils::OUT_LIST_INT2
        || lc_var_type == utils::OUT_LIST_STRING
        || lc_var_type == utils::OUT_MATRIX_INT
        || lc_var_type == utils::OUT_MATRIX_INT2
    {
        if lc_var_type == utils::OUT_LIST_INT || lc_var_type == utils::OUT_LIST_INT2 {
            cpp_var_type = String::from("vector<int>");
        } else if lc_var_type == utils::OUT_LIST_STRING {
            cpp_var_type = String::from("vector<string>");
        } else if lc_var_type == utils::OUT_MATRIX_INT || lc_var_type == utils::OUT_MATRIX_INT2 {
            cpp_var_type = String::from("vector<vector<int>>");
        } else {
            return None; // Won't happen
        }

        cpp_var_value = String::new();
        for c in var_value.chars() {
            if c == '[' {
                cpp_var_value.push('{');
            } else if c == ']' {
                cpp_var_value.push('}')
            } else {
                cpp_var_value.push(c);
            }
        }
    } else {
        return None;
    }

    Some((cpp_var_type, cpp_var_value))
}

fn print_output(lc_var_type: &String, example_number: usize) -> String {
    let print_output: String;

    if lc_var_type == utils::OUT_VOID {
        print_output = "".to_string();
    } else if lc_var_type == utils::OUT_INT
        || lc_var_type == utils::OUT_BOOL
        || lc_var_type == utils::OUT_STRING
    {
        print_output = format!("\tcout << output_{} << endl;\n", example_number);
    } else if lc_var_type == utils::OUT_LIST_INT
        || lc_var_type == utils::OUT_LIST_INT2
        || lc_var_type == utils::OUT_LIST_STRING
    {
        print_output = format!("\tprintArray(output_{});\n", example_number);
    } else if lc_var_type == utils::OUT_MATRIX_INT || lc_var_type == utils::OUT_MATRIX_INT2{
        print_output = format!("\tprint2Dmatrix(output_{});\n", example_number);
    } else {
        print_output = "".to_string();
    }

    print_output
}
