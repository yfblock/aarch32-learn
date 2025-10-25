#[macro_export]
macro_rules! read_csr {
    ($csr:literal) => {
        read_csr!($csr, usize)
    };
    ($csr:literal, $ty:ty) => {
        unsafe {
            let value: $ty;
            core::arch::asm!(concat!("MRS {}, ", $csr), out(reg) value);
            value
        }
    };
}
