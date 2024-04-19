/// Lua指令编码模式常量
/// Lua指令模式可以分为4类
/// I_ABC:  可以携带3个操作数A、B、C，分别占用8、9、9个bit
/// I_ABx:  可以携带A、Bx两个操作数，分别占用8、18个bit
/// I_AsBx: 可以携带A、sBx两个操作数，分别占用8、18个bit
/// I_Ax:   可以携带Ax一个操作数，占用26个bit
/// Lua虚拟机指令以iABC模式居多，在总计47条指令中，有39条使用iABC模式。其余8条指令中，有3条使用iABx指令，4条使用iAsBx模式，1条使用iAx格式
pub const OP_MODE_ABC: u8 = 0;
pub const OP_MODE_ABX: u8 = 1;
pub const OP_MODE_ASBX: u8 = 2;
pub const OP_MODE_AX: u8 = 3;

/// 操作数类型
/// 每条指令可以携带1~3个操作数，其中操作数A主要用于表示目标寄存器索引
/// 其他操作数可以分为四种类型
/// OpArgN: 参数不会被使用
/// OpArgU: 参数会被使用
/// OpArgR: 参数表示寄存器或jump偏移
/// OpArgK: 参数表示常量索引或寄存器索引
pub const OP_ARG_N: u8 = 0;
pub const OP_ARG_U: u8 = 1;
pub const OP_ARG_R: u8 = 2;
pub const OP_ARG_K: u8 = 3;

/// Lua5.3中每条Lua虚拟机指令占4字节，共32个bit
/// 低6位用于表示操作码(opcode)
/// 高26位用于表示操作数
// const OP_MOVE: u8 = 0;
// const OP_LOADK: u8 = 1;
// const OP_LOADKX: u8 = 2;
// const OP_LOADBOOL: u8 = 3;
// const OP_LOADNIL: u8 = 4;
// const OP_GETUPVAL: u8 = 5;
// const OP_GETTABUP: u8 = 6;
// const OP_GETTABLE: u8 = 7;
// const OP_SETTABUP: u8 = 8;
// const OP_SETUPVAL: u8 = 9;
// const OP_SETTABLE: u8 = 10;
// const OP_NEWTABLE: u8 = 11;
// const OP_SELF: u8 = 12;
// const OP_ADD: u8 = 13;
// const OP_SUB: u8 = 14;
// const OP_MUL: u8 = 15;
// const OP_MOD: u8 = 16;
// const OP_POW: u8 = 17;
// const OP_DIV: u8 = 18;
// const OP_IDIV: u8 = 19;
// const OP_BAND: u8 = 20;
// const OP_BOR: u8 = 21;
// const OP_BXOR: u8 = 22;
// const OP_SHL: u8 = 23;
// const OP_SHR: u8 = 24;
// const OP_UNM: u8 = 25;
// const OP_BNOT: u8 = 26;
// const OP_NOT: u8 = 27;
// const OP_LEN: u8 = 28;
// const OP_CONCAT: u8 = 29;
// const OP_JMP: u8 = 30;
// const OP_EQ: u8 = 31;
// const OP_LT: u8 = 32;
// const OP_LE: u8 = 33;
// const OP_TEST: u8 = 34;
// const OP_TESTSET: u8 = 35;
// const OP_CALL: u8 = 36;
// const OP_TAILCALL: u8 = 37;
// const OP_RETURN: u8 = 38;
// const OP_FORLOOP: u8 = 39;
// const OP_FORPREP: u8 = 40;
// const OP_TFORCALL: u8 = 41;
// const OP_TFORLOOP: u8 = 42;
// const OP_SETLIST: u8 = 43;
// const OP_CLOSURE: u8 = 44;
// const OP_VARARG: u8 = 45;
// const OP_EXTRAARG: u8 = 46;

pub struct OpCode {
    // OpCode是否是test，如果是test，那么下一个操作码必须是jump
    pub test_flag: u8,
    // 寄存器A赋值
    pub set_a_flag: u8,
    // B arg模式
    pub b_arg_mode: u8,
    // C arg模式
    pub c_arg_mode: u8,
    // 指令编码模式
    pub op_mode: u8,
    pub name: &'static str,
}

pub const OP_CODES: &'static [OpCode] = &[
    /*       B       C     mode    name    */
    opcode(0, 1, OP_ARG_R, OP_ARG_N, OP_MODE_ABC, "MOVE    "), // R(A) := R(B)
    opcode(0, 1, OP_ARG_K, OP_ARG_N, OP_MODE_ABX, "LOADK   "), // R(A) := Kst(Bx)
    opcode(0, 1, OP_ARG_N, OP_ARG_N, OP_MODE_ABX, "LOADKX  "), // R(A) := Kst(extra arg)
    opcode(0, 1, OP_ARG_U, OP_ARG_U, OP_MODE_ABC, "LOADBOOL"), // R(A) := (bool)B; if (C) pc++
    opcode(0, 1, OP_ARG_U, OP_ARG_N, OP_MODE_ABC, "LOADNIL "), // R(A), R(A+1), ..., R(A+B) := nil
    opcode(0, 1, OP_ARG_U, OP_ARG_N, OP_MODE_ABC, "GETUPVAL"), // R(A) := UpValue[B]
    opcode(0, 1, OP_ARG_U, OP_ARG_K, OP_MODE_ABC, "GETTABUP"), // R(A) := UpValue[B][RK(C)]
    opcode(0, 1, OP_ARG_R, OP_ARG_K, OP_MODE_ABC, "GETTABLE"), // R(A) := R(B)[RK(C)]
    opcode(0, 0, OP_ARG_K, OP_ARG_K, OP_MODE_ABC, "SETTABUP"), // UpValue[A][RK(B)] := RK(C)
    opcode(0, 0, OP_ARG_U, OP_ARG_N, OP_MODE_ABC, "SETUPVAL"), // UpValue[B] := R(A)
    opcode(0, 0, OP_ARG_K, OP_ARG_K, OP_MODE_ABC, "SETTABLE"), // R(A)[RK(B)] := RK(C)
    opcode(0, 1, OP_ARG_U, OP_ARG_U, OP_MODE_ABC, "NEWTABLE"), // R(A) := {} (size = B,C)
    opcode(0, 1, OP_ARG_R, OP_ARG_K, OP_MODE_ABC, "SELF    "), // R(A+1) := R(B); R(A) := R(B)[RK(C)]
    opcode(0, 1, OP_ARG_K, OP_ARG_K, OP_MODE_ABC, "ADD     "), // R(A) := RK(B) + RK(C)
    opcode(0, 1, OP_ARG_K, OP_ARG_K, OP_MODE_ABC, "SUB     "), // R(A) := RK(B) - RK(C)
    opcode(0, 1, OP_ARG_K, OP_ARG_K, OP_MODE_ABC, "MUL     "), // R(A) := RK(B) * RK(C)
    opcode(0, 1, OP_ARG_K, OP_ARG_K, OP_MODE_ABC, "MOD     "), // R(A) := RK(B) % RK(C)
    opcode(0, 1, OP_ARG_K, OP_ARG_K, OP_MODE_ABC, "POW     "), // R(A) := RK(B) ^ RK(C)
    opcode(0, 1, OP_ARG_K, OP_ARG_K, OP_MODE_ABC, "DIV     "), // R(A) := RK(B) / RK(C)
    opcode(0, 1, OP_ARG_K, OP_ARG_K, OP_MODE_ABC, "IDIV    "), // R(A) := RK(B) // RK(C)
    opcode(0, 1, OP_ARG_K, OP_ARG_K, OP_MODE_ABC, "BAND    "), // R(A) := RK(B) & RK(C)
    opcode(0, 1, OP_ARG_K, OP_ARG_K, OP_MODE_ABC, "BOR     "), // R(A) := RK(B) | RK(C)
    opcode(0, 1, OP_ARG_K, OP_ARG_K, OP_MODE_ABC, "BXOR    "), // R(A) := RK(B) ~ RK(C)
    opcode(0, 1, OP_ARG_K, OP_ARG_K, OP_MODE_ABC, "SHL     "), // R(A) := RK(B) << RK(C)
    opcode(0, 1, OP_ARG_K, OP_ARG_K, OP_MODE_ABC, "SHR     "), // R(A) := RK(B) >> RK(C)
    opcode(0, 1, OP_ARG_R, OP_ARG_N, OP_MODE_ABC, "UNM     "), // R(A) := -R(B)
    opcode(0, 1, OP_ARG_R, OP_ARG_N, OP_MODE_ABC, "BNOT    "), // R(A) := ~R(B)
    opcode(0, 1, OP_ARG_R, OP_ARG_N, OP_MODE_ABC, "NOT     "), // R(A) := not R(B)
    opcode(0, 1, OP_ARG_R, OP_ARG_N, OP_MODE_ABC, "LEN     "), // R(A) := length of R(B)
    opcode(0, 1, OP_ARG_R, OP_ARG_R, OP_MODE_ABC, "CONCAT  "), // R(A) := R(B).. ... ..R(C)
    opcode(0, 0, OP_ARG_R, OP_ARG_N, OP_MODE_ASBX, "JMP     "), // pc+=sBx; if (A) close all upvalues >= R(A - 1)
    opcode(1, 0, OP_ARG_K, OP_ARG_K, OP_MODE_ABC, "EQ      "),  // if ((RK(B) == RK(C)) ~= A) then pc++
    opcode(1, 0, OP_ARG_K, OP_ARG_K, OP_MODE_ABC, "LT      "),  // if ((RK(B) <  RK(C)) ~= A) then pc++
    opcode(1, 0, OP_ARG_K, OP_ARG_K, OP_MODE_ABC, "LE      "),  // if ((RK(B) <= RK(C)) ~= A) then pc++
    opcode(1, 0, OP_ARG_N, OP_ARG_U, OP_MODE_ABC, "TEST    "),  // if not (R(A) <=> C) then pc++
    opcode(1, 1, OP_ARG_R, OP_ARG_U, OP_MODE_ABC, "TESTSET "), // if (R(B) <=> C) then R(A) := R(B) else pc++
    opcode(0, 1, OP_ARG_U, OP_ARG_U, OP_MODE_ABC, "CALL    "), // R(A), ... ,R(A+C-2) := R(A)(R(A+1), ... ,R(A+B-1))
    opcode(0, 1, OP_ARG_U, OP_ARG_U, OP_MODE_ABC, "TAILCALL"), // return R(A)(R(A+1), ... ,R(A+B-1))
    opcode(0, 0, OP_ARG_U, OP_ARG_N, OP_MODE_ABC, "RETURN  "), // return R(A), ... ,R(A+B-2)
    opcode(0, 1, OP_ARG_R, OP_ARG_N, OP_MODE_ASBX, "FORLOOP "), // R(A)+=R(A+2); if R(A) <?= R(A+1) then { pc+=sBx; R(A+3)=R(A) }
    opcode(0, 1, OP_ARG_R, OP_ARG_N, OP_MODE_ASBX, "FORPREP "), // R(A)-=R(A+2); pc+=sBx
    opcode(0, 0, OP_ARG_N, OP_ARG_U, OP_MODE_ABC, "TFORCALL"), // R(A+3), ... ,R(A+2+C) := R(A)(R(A+1), R(A+2));
    opcode(0, 1, OP_ARG_R, OP_ARG_N, OP_MODE_ASBX, "TFORLOOP"), // if R(A+1) ~= nil then { R(A)=R(A+1); pc += sBx }
    opcode(0, 0, OP_ARG_U, OP_ARG_U, OP_MODE_ABC, "SETLIST "), // R(A)[(C-1)*FPF+i] := R(A+i), 1 <= i <= B
    opcode(0, 1, OP_ARG_U, OP_ARG_N, OP_MODE_ABX, "CLOSURE "), // R(A) := closure(KPROTO[Bx])
    opcode(0, 1, OP_ARG_U, OP_ARG_N, OP_MODE_ABC, "VARARG  "), // R(A), R(A+1), ..., R(A+B-2) = vararg
    opcode(0, 0, OP_ARG_U, OP_ARG_U, OP_MODE_AX, "EXTRAARG"), // extra (larger) argument for previous opcode
];

const fn opcode(
    test_flag: u8,
    set_a_flag: u8,
    b_arg_mode: u8,
    c_arg_mode: u8,
    op_mode: u8,
    name: &'static str,
) -> OpCode {
    OpCode {
        b_arg_mode,
        c_arg_mode,
        op_mode,
        name,
        test_flag,
        set_a_flag,
    }
}
