use std::ptr;

mod cpp_envelopes_analysis;

fn main() {
    
    let command_line_arguments: Vec<String> = std::env::args().collect();
    let mut is_arguments_valid: bool = true;

    if command_line_arguments.len() != 5
    {
        println!("Please enter 4 arguments - numbers with floating point:\n \
         - first envelope width\n \
         - first envelope length\n \
         - second envelope width\n \
         - second envelope length\n");
        return;
    }

    let envelope1_size1 = command_line_arguments[1].parse::<f64>().unwrap_or_else(|_| {
        println!("Not valid envelope size: {}.", command_line_arguments[1]);
        is_arguments_valid = false;
        0.0
    });

    let envelope1_size2 = command_line_arguments[2].parse::<f64>().unwrap_or_else(|_| {
        println!("Not valid envelope size: {}.", command_line_arguments[2]);
        is_arguments_valid = false;
        0.0
    });

    let envelope2_size1 = command_line_arguments[3].parse::<f64>().unwrap_or_else(|_| {
        println!("Not valid envelope size: {}.", command_line_arguments[3]);
        is_arguments_valid = false;
        0.0
    });

    let envelope2_size2 = command_line_arguments[4].parse::<f64>().unwrap_or_else(|_| {
        println!("Not valid envelope size: {}.", command_line_arguments[4]);
        is_arguments_valid = false;
        0.0
    });

    if !is_arguments_valid{
        println!("Program stopped due to previous error.");
        return;
    }


    let mut container: *mut std::os::raw::c_void;
    container = ptr::null_mut();

    
    unsafe{
        if cpp_envelopes_analysis::InitializeEnvelopesContainer(&mut container, envelope1_size1, envelope1_size2, envelope2_size1, envelope2_size2) == cpp_envelopes_analysis::INIT_SUCCEDED {
            let mut can_contain: bool;
            can_contain = false;

            if cpp_envelopes_analysis::CanOneEnvelopeContainAnother(container, &mut can_contain) == cpp_envelopes_analysis::COMPARATION_SUCCEDED {
                
                if can_contain {
                    println!("One of the envelopes can contain another!");
                }
                else {
                    println!("No one of the envelopes can contain another!");

                }
            }
            
        }
    }
    
}
