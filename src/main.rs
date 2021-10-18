use std::ptr;

mod cpp_envelopes_analysis;

fn main() {
    
    let mut command_line_arguments = std::env::args();

    if command_line_arguments.len() != 5
    {
        println!("Please enter 4 arguments - numbers with floating point:\n \
         - first envelope width\n \
         - first envelope length\n \
         - second envelope width\n \
         - second envelope length\n");
        return;
    }

    command_line_arguments.next();    
    let arg_envelope1_size1 = command_line_arguments.next().unwrap();
    let arg_envelope1_size2 = command_line_arguments.next().unwrap();
    let arg_envelope2_size1 = command_line_arguments.next().unwrap();
    let arg_envelope2_size2 = command_line_arguments.next().unwrap();

    let envelope1_size1 = arg_envelope1_size1.parse::<f64>().unwrap_or_else(|_| {
        println!("Not valid envelope size: {}.", arg_envelope1_size1);
        0.0
    });

    let envelope1_size2 = arg_envelope1_size2.parse::<f64>().unwrap_or_else(|_| {
        println!("Not valid envelope size: {}.", arg_envelope1_size2);
        0.0
    });

    let envelope2_size1 = arg_envelope2_size1.parse::<f64>().unwrap_or_else(|_| {
        println!("Not valid envelope size: {}.", arg_envelope2_size1);
        0.0
    });

    let envelope2_size2 = arg_envelope2_size2.parse::<f64>().unwrap_or_else(|_| {
        println!("Not valid envelope size: {}.", arg_envelope2_size2);
        0.0
    });

    let mut container: *mut std::os::raw::c_void;
    container = ptr::null_mut();

    
    unsafe{
        if cpp_envelopes_analysis::InitializeEnvelopesContainer(&mut container, envelope1_size1, envelope1_size2, envelope2_size1, envelope2_size2) == cpp_envelopes_analysis::INIT_SUCCEDED {
            let mut can_contain: bool;
            can_contain = false;

            if cpp_envelopes_analysis::CanOneEnvelopeContainAnother(container, &mut can_contain) == cpp_envelopes_analysis::COMPARATION_SUCCEDED {
                
                if can_contain {
                    println!("Can contain!");
                }
                else {
                    println!("Can not contain!");

                }
            }
            
        }
    }
    
}
