#[cfg(feature = "rrc")]
pub mod spec_rrc {
    #![allow(warnings)]
    include!("./spec_rrc.rs");
}
#[cfg(feature = "s1ap")]
pub mod spec_s1ap {
    #![allow(warnings)]
    include!("./spec_s1ap.rs");
}
#[cfg(feature = "x2ap")]
pub mod spec_x2ap {
    #![allow(warnings)]
    include!("./spec_x2ap.rs");
}
#[cfg(feature = "m2ap")]
pub mod spec_m2ap {
    #![allow(warnings)]
    include!("./spec_m2ap.rs");
}
#[cfg(feature = "lppa")]
pub mod spec_lppa {
    #![allow(warnings)]
    include!("./spec_lppa.rs");
}
