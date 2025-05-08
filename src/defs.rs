use num_enum::{IntoPrimitive, TryFromPrimitive};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, TryFromPrimitive, IntoPrimitive)]
#[repr(u16)]
pub enum Opcode {
    Next = 0,
    EndScript,
    EndEvt,
    Lbl,
    Goto,
    Do,
    While,
    DoBreak,
    DoContinue,
    WaitFrm,
    WaitMsec,
    Halt,
    IfStrEqual,
    IfStrNotEqual,
    IfStrSmall,
    IfStrLarge,
    IfStrSmallEqual,
    IfStrLargeEqual,
    IffEqual,
    IffNotEqual,
    IffSmall,
    IffLarge,
    IffSmallEqual,
    IffLargeEqual,
    IfEqual,
    IfNotEqual,
    IfSmall,
    IfLarge,
    IfSmallEqual,
    IfLargeEqual,
    IfFlag,
    IfNotFlag,
    Else,
    EndIf,
    Switch,
    Switchi,
    CaseEqual,
    CaseNotEqual,
    CaseSmall,
    CaseLarge,
    CaseSmallEqual,
    CaseLargeEqual,
    CaseEtc,
    CaseOr,
    CaseAnd,
    CaseFlag,
    CaseEnd,
    CaseBetween,
    SwitchBreak,
    EndSwitch,
    Set,
    Seti,
    Setf,
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Addf,
    Subf,
    Mulf,
    Divf,
    SetRead,
    Read,
    Read2,
    Read3,
    Read4,
    ReadN,
    SetReadf,
    Readf,
    Readf2,
    Readf3,
    Readf4,
    ReadfN,
    ClampInt,
    SetUserWrk,
    SetUserFlg,
    AllocUserWrk,
    And,
    Andi,
    Or,
    Ori,
    SetFrameFromMsec,
    SetMsecFromFrame,
    SetRam,
    SetRamf,
    GetRam,
    GetRamf,
    Setr,
    Setrf,
    Getr,
    Getrf,
    UserFunc,
    RunEvt,
    RunEvtId,
    RunChildEvt,
    DeleteEvt,
    RestartEvt,
    SetPri,
    SetSpd,
    SetType,
    StopAll,
    StartAll,
    StopOther,
    StartOther,
    StopId,
    StartId,
    ChkEvt,
    InlineEvt,
    InlineEvtId,
    EndInline,
    BrotherEvt,
    BrotherEvtId,
    EndBrother,
    DebugPutMsg,
    DebugMsgClear,
    DebugPutReg,
    DebugName,
    DebugRem,
    DebugBp,
}

pub const EVTDAT_ADDR_MAX: i32 = -290000000;
pub const EVTDAT_FLOAT_MAX: i32 = -220000000;
pub const EVTDAT_UF_MAX: i32 = -200000000;
pub const EVTDAT_UW_MAX: i32 = -180000000;
pub const EVTDAT_GSW_MAX: i32 = -160000000;
pub const EVTDAT_LSW_MAX: i32 = -140000000;
pub const EVTDAT_GSWF_MAX: i32 = -120000000;
pub const EVTDAT_LSWF_MAX: i32 = -100000000;
pub const EVTDAT_GF_MAX: i32 = -80000000;
pub const EVTDAT_LF_MAX: i32 = -60000000;
pub const EVTDAT_GW_MAX: i32 = -40000000;
pub const EVTDAT_LW_MAX: i32 = -20000000;

pub const EVTDAT_ADDR_BASE: i32 = -270000000;
pub const EVTDAT_FLOAT_BASE: i32 = -240000000;
pub const EVTDAT_UF_BASE: i32 = -210000000;
pub const EVTDAT_UW_BASE: i32 = -190000000;
pub const EVTDAT_GSW_BASE: i32 = -170000000;
pub const EVTDAT_LSW_BASE: i32 = -150000000;
pub const EVTDAT_GSWF_BASE: i32 = -130000000;
pub const EVTDAT_LSWF_BASE: i32 = -110000000;
pub const EVTDAT_GF_BASE: i32 = -90000000;
pub const EVTDAT_LF_BASE: i32 = -70000000;
pub const EVTDAT_GW_BASE: i32 = -50000000;
pub const EVTDAT_LW_BASE: i32 = -30000000;
