pub struct Aid;

pub enum XnkError {
    Illegal,
}

pub type Result<T> = core::result::Result<T, XnkError>;

pub enum TimerStat {
    Continue,
    Cancel,
}

pub fn tickrate() -> u64 {
    todo!()
}

pub fn settimer(ticks: u64, func: impl FnMut() -> TimerStat) -> Aid {
    todo!()
}

pub fn aidstop(id: Aid) -> Result<()> {
    todo!()
}

pub fn aiddone(id: Aid) -> bool {
    todo!()
}

pub fn syscall_set(opcode: u64, func: impl FnMut()) -> Result<()> {
    todo!()
}

pub fn aidyield(id: Aid) {
}
