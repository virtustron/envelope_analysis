mod cpp_envelopes_analysis;

fn main() {
    let container: *mut ::std::os::raw::c_void;

    unsafe{
        if cpp_envelopes_analysis::InitializeEnvelopesContainer(container) == cpp_envelopes_analysis::INIT_SUCCEDED {
            let mut can_contain: *mut bool;

            if cpp_envelopes_analysis::CanOneEnvelopeContainAnother(container, can_contain) == 0 {
                
            }
        }
    }

    /*
	void* container = NULL;
	
	// 10.0, 10.0, 15.0, 5.0
	if (InitializeEnvelopesContainer(&container) == INIT_SUCCEDED)
	{
		bool can_contain;
		
		if (CanOneEnvelopeContainAnother(container, &can_contain));
	
		if (can_contain)
		{
			std::cout << "Can contain" << "\n";
		}
		else
		{
			std::cout << "Can not contain" << "\n";
		}

	}

	if (container != NULL)
	{
		delete container;
	} 


    */



    println!("Hello, world!");
}
