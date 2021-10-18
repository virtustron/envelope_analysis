use std::ptr;

mod cpp_envelopes_analysis;

fn main() {
    //let container: *mut ::std::os::raw::c_void;
    let container: *mut *mut std::os::raw::c_void;
    container = ptr::null_mut();

    unsafe{
        if cpp_envelopes_analysis::InitializeEnvelopesContainer(container) == cpp_envelopes_analysis::INIT_SUCCEDED {
            let can_contain: *mut bool = ptr::null_mut();
            *can_contain = false;

            if cpp_envelopes_analysis::CanOneEnvelopeContainAnother(*container, can_contain) == cpp_envelopes_analysis::COMPARATION_SUCCEDED {
                
                if *can_contain {
                    println!("Can contain!");
                }
                else {
                    println!("Can not contain!");

                }
            }
            
        }
    }
}
