/* automatically generated by rust-bindgen 0.59.1 */

pub const INIT_SUCCEDED: ::std::os::raw::c_int = 0;
pub const INIT_INVALID_ENVELOPE_SIZE: ::std::os::raw::c_int = 1;
pub const COMPARATION_SUCCEDED: ::std::os::raw::c_int = 0;
pub const COMPARATION_FAILED: ::std::os::raw::c_int = 1;
pub const COMPARATION_CONTAINER_IS_NULL: ::std::os::raw::c_int = 2;
extern "C" {
    #[link_name = "\u{1}_Z28InitializeEnvelopesContainerPPvdddd"]
    pub fn InitializeEnvelopesContainer(
        container_to_initialize: *mut *mut ::std::os::raw::c_void,
        envelope1_size1: f64,
        envelope1_size2: f64,
        envelope2_size1: f64,
        envelope2_size2: f64,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    #[link_name = "\u{1}_Z28CanOneEnvelopeContainAnotherPvPb"]
    pub fn CanOneEnvelopeContainAnother(
        container: *mut ::std::os::raw::c_void,
        can_contain: *mut bool,
    ) -> ::std::os::raw::c_int;
}
