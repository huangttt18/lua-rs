/// Header 常量
pub const LUA_SIGNATURE: [u8; 4] = [0x1B, b'L', b'u', b'a'];
pub const LUAC_VERSION: u8 = 0x53;
pub const LUAC_FORMAT: u8 = 0x00;
pub const LUAC_DATA: [u8; 6] = [0x19, 0x93, b'\r', b'\n', 0x1A, b'\n'];
pub const CINT_SIZE: u8 = 0x04;
pub const C_SIZE_T_SIZE: u8 = 0x08;
pub const INSTRUCTION_SIZE: u8 = 0x04;
pub const LUA_INTEGER_SIZE: u8 = 0x08;
pub const LUA_NUMBER_SIZE: u8 = 0x08;
pub const LUAC_INT: i64 = 0x5678;
pub const LUAC_NUM: f64 = 370.5;

// Prototype constants
pub const TAG_NIL: u8 = 0x00;
pub const TAG_BOOLEAN: u8 = 0x01;
pub const TAG_NUMBER: u8 = 0x03;
pub const TAG_INTEGER: u8 = 0x13;
pub const TAG_SHORT_STR: u8 = 0x04;
pub const TAG_LONG_STR: u8 = 0x14;

/// Lua chunk文件结构
#[derive(Debug)]
pub struct BinaryChunk {
    pub header: Header,
    pub size_upvalues: u8,
    pub main_func: Prototype,
}

/// Lua chunk文件头
#[derive(Debug)]
pub struct Header {
    // [4 bytes] MagicNumber: 0x1B4C6561
    pub signature: [u8; 4],
    // [1 byte]  主版本号(MajorVersion) * 16 + 小版本号(MinorVersion), lua5(MajorVer).3(MinorVer).4(ReleaseVer) => 5 * 16 + 3 = 83 => 0x53
    pub version: u8,
    // [1 byte]  格式化Number: 0x00
    pub format: u8,
    // [6 bytes] LUAC_DATA: 0x19 93 0D 0A 1A 0A
    pub luac_data: [u8; 6],
    // [1 byte]  C int 占用字节数
    pub c_int_size: u8,
    // [1 byte]  C size_t 占用字节数
    pub size_t_size: u8,
    // [1 byte]  Instruction 占用字节数
    pub instruction_size: u8,
    // [1 byte]  Lua 整型 占用字节数
    pub lua_integer_size: u8,
    // [1 byte]  Lua 浮点型占用字节数
    pub lua_number_size: u8,
    // [n bytes] Lua整数 0x5678，占用字节大小根据机器决定
    pub luac_int: i64,
    // [n bytes] Lua浮点数 370.5，占用字节大小根据机器决定
    pub luac_num: f64,
}

/// lua 函数原型，包括
/// 1. 函数基本信息
///     1.1 源文件名
///     1.2 起止行号
///     1.3 固定参数个数
///     1.4 是否是vararg函数
///     1.5 函数运行需要的寄存器数量
/// 2. 指令表
/// 3. 常量表
/// 4. upvalue表
/// 5. 子函数原型表
/// 6. 调试信息
///     6.1 行号表
///     6.2 局部变量表
///     6.3 upvalue名列表
#[derive(Debug)]
pub struct Prototype {
    // 源文件名
    pub source: String,
    // 开始行号
    pub line_defined: u32,
    // 结束行号
    pub last_line_defined: u32,
    // 固定参数个数
    pub num_params: u8,
    // 是否包含可变参数
    pub is_vararg: u8,
    // 寄存器数量
    pub max_stack_size: u8,
    // 指令表
    pub code: Vec<u32>,
    // 常量表
    pub constants: Vec<Constant>,
    // upvalue表
    pub upvalues: Vec<UpValue>,
    // 函数原型表
    pub protos: Vec<Prototype>,
    // 行号表
    pub line_info: Vec<u32>,
    // 局部变量表
    pub loc_vars: Vec<LocVar>,
    // upvalue名表
    pub upvalue_names: Vec<String>,
}

/// 常量,
#[derive(Debug)]
pub enum Constant {
    Nil,
    Boolean(bool),
    Number(f64),
    Integer(i64),
    Str(String),
}

/// 类似闭包中的变量
#[derive(Debug)]
pub struct UpValue {
    pub instack: u8,
    pub idx: u8,
}

/// 局部变量
#[derive(Debug)]
pub struct LocVar {
    pub var_name: String,
    pub start_pc: u32,
    pub end_pc: u32,
}