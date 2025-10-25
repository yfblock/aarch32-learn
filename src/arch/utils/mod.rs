use crate::arch::utils::structs::PrivilegeMode;

pub mod structs;
/// Get the current privilege
///
/// <https://developer.arm.com/documentation/ddi0601/2025-09/AArch32-Registers/SPSR--Saved-Program-Status-Register>
#[inline]
pub fn get_pl() -> PrivilegeMode {
    let cpsr = read_csr!("CPSR");
    PrivilegeMode::from(cpsr as u8 & 0x1f)
}
