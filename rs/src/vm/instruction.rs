use crate::vm::opcode;

/// Lua虚拟机指令
/// 通过本地trait的方式，能够给u32实现虚拟机指令需要的方法
pub trait Instruction {
    // 从指令中解码op_code
    fn op_code(self) -> u8;
    // I_ABC模式，提取参数
    fn abc(self) -> (isize, isize, isize);
    // I_ABx模式，提取参数
    fn abx(self) -> (isize, isize);
    // I_AsBx模式，提取参数
    fn asbx(self) -> (isize, isize);
    // I_Ax模式，提取参数
    fn ax(self) -> isize;
    // 获取op name
    fn op_name(self) -> &'static str;
    // 获取op mode
    fn op_mode(self) -> u8;
    // 获取b arg mode
    fn b_arg_mode(self) -> u8;
    // 获取c arg mode
    fn c_arg_mode(self) -> u8;
}

const MAX_ARG_BX: isize = (1 << 18) - 1;
const MAX_ARG_SBX: isize = MAX_ARG_BX >> 1;

impl Instruction for u32 {
    // 从指令中解码op_code
    fn op_code(self) -> u8 {
        self as u8 & 0x3F
    }

    // I_ABC模式，提取参数
    fn abc(self) -> (isize, isize, isize) {
        let a = (self >> 6 & 0xFF) as isize;
        let b = (self >> 14 & 0x1FF) as isize;
        let c = (self >> 23 & 0x1FF) as isize;
        println!("{a} {b} {c}");
        (a, b, c)
    }

    // I_ABx模式，提取参数
    fn abx(self) -> (isize, isize) {
        let a = (self >> 6 & 0xFF) as isize;
        let bx = (self >> 14) as isize;
        (a, bx)
    }

    // I_AsBx模式，提取参数
    fn asbx(self) -> (isize, isize) {
        let (a, sbx) = self.abx();
        (a, sbx - MAX_ARG_SBX)
    }

    // I_Ax模式，提取参数
    fn ax(self) -> isize {
        (self >> 6) as isize
    }

    // 获取op name
    fn op_name(self) -> &'static str {
        opcode::OP_CODES[self.op_code() as usize].name
    }

    // 获取op mode
    fn op_mode(self) -> u8 {
        opcode::OP_CODES[self.op_code() as usize].op_mode
    }

    // 获取b arg mode
    fn b_arg_mode(self) -> u8 {
        opcode::OP_CODES[self.op_code() as usize].b_arg_mode
    }

    // 获取c arg mode
    fn c_arg_mode(self) -> u8 {
        opcode::OP_CODES[self.op_code() as usize].c_arg_mode
    }
}
