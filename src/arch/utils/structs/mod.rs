use num_enum::FromPrimitive;

/// PrivilegeMode Information
///
/// <https://developer.arm.com/documentation/ddi0601/2025-09/AArch32-Registers/SPSR--Saved-Program-Status-Register>
#[derive(PartialEq, Eq, FromPrimitive, Debug, Clone, Copy)]
#[repr(u8)]
pub enum PrivilegeMode {
    User = 0b10000,
    Fiq,
    Irq,
    Supervisor,
    Monitor,
    Abort,
    Hyp,
    Undefined,
    System,
    #[default]
    Reserved,
}
