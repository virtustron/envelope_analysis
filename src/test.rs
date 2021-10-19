

#[cfg(test)]
mod test {
    //use super::*;
    use crate::cpp_envelopes_analysis;
    use std::ptr;

    #[test]
    fn larger_can_contain_smaller() {
        let mut can_contain: bool;
        can_contain = false;
        
        let mut container: *mut std::os::raw::c_void;
        container = ptr::null_mut();

        unsafe{
            let init_result: std::os::raw::c_int;
            init_result = cpp_envelopes_analysis::InitializeEnvelopesContainer(&mut container, 20.0, 20.0, 10.0, 10.0);

            let comparation_result: std::os::raw::c_int;
            comparation_result = cpp_envelopes_analysis::CanOneEnvelopeContainAnother(container, &mut can_contain);

            assert!(
                can_contain,
                "fn InitializeEnvelopesContainer() returned `{}\n \
                 and fn CanOneEnvelopeContainAnother() returned {}\n",
                init_result, 
                comparation_result
            );
        }
    }

    #[test]
    fn smaller_cannot_contain_larger() {
        let mut can_contain: bool;
        can_contain = false;

        let mut container: *mut std::os::raw::c_void;
        container = ptr::null_mut();

        unsafe{
            let init_result: std::os::raw::c_int;
            init_result = cpp_envelopes_analysis::InitializeEnvelopesContainer(&mut container, 20.0, 20.0, 10.0, 50.0);

            let comparation_result: std::os::raw::c_int;
            comparation_result = cpp_envelopes_analysis::CanOneEnvelopeContainAnother(container, &mut can_contain);

            assert!(
                !can_contain,
                "fn InitializeEnvelopesContainer() returned `{}\n \
                 and fn CanOneEnvelopeContainAnother() returned {}\n",
                init_result, 
                comparation_result
            );
        }
    }

    #[test]
    fn return_comparation_succeded() {
        let mut can_contain: bool;
        can_contain = false;

        let mut container: *mut std::os::raw::c_void;
        container = ptr::null_mut();

        unsafe{
            let init_result: std::os::raw::c_int;
            init_result = cpp_envelopes_analysis::InitializeEnvelopesContainer(&mut container, 20.0, 20.0, 10.0, 50.0);

            let comparation_result: std::os::raw::c_int;
            comparation_result = cpp_envelopes_analysis::CanOneEnvelopeContainAnother(container, &mut can_contain);

            assert_eq!(
                comparation_result, cpp_envelopes_analysis::COMPARATION_SUCCEDED,
                "fn InitializeEnvelopesContainer() returned `{}\n \
                 and fn CanOneEnvelopeContainAnother() returned {}\n",
                init_result, 
                comparation_result
            );
        }
    }
}