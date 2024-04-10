use bytes::{Buf, BytesMut};

use super::chunk::{self, Constant, LocVar, Prototype, UpValue};

#[derive(Debug)]
pub struct Reader {
    data: BytesMut,
}

#[allow(dead_code)]
impl Reader {
    pub fn new(data: &[u8]) -> Self {
        Self { data: BytesMut::from(data) }
    }

    pub fn read_u8(&mut self) -> u8 {
        self.data.get_u8()
    }

    fn read_u32(&mut self) -> u32 {
        self.data.get_u32_le()
    }

    fn read_u64(&mut self) -> u64 {
        self.data.get_u64_le()
    }

    fn read_lua_int(&mut self) -> i64 {
        self.data.get_i64_le()
    }

    fn read_lua_num(&mut self) -> f64 {
        self.data.get_f64_le()
    }

    // 字符串分为短字符串和长字符串
    // 对于NULL字符串，长度为0x00
    // 对于短字符串, 长度 <= 253(0xFD), 先用一个字节记录长度+1, 然后是字节数组
    // 对于长字符串, 长度 >= 254(0xFE), 第一个字节是0xFF, 然后加一个size_t记录长度+1, 最后是字节数组
    fn read_string(&mut self) -> String {
        let size = self.data.get_u8();
        if size == 0x00 {
            return String::new();
        }

        let buf: BytesMut;
        if size == 0xFF {
            let size = self.data.get_u64();
            buf = self.data.split_to((size - 1) as usize);
        } else {
            buf = self.data.split_to((size - 1) as usize);
        }

        let str = std::str::from_utf8(&buf).unwrap();

        str.to_string()
    }

    fn read_bytes(&mut self, n_bytes: u32) -> BytesMut {
        self.data.split_to(n_bytes as usize)
    }
}

impl Reader {
    pub fn check_header(&mut self) {
        if self.read_bytes(4).as_ref() != &chunk::LUA_SIGNATURE {
            panic!("not a precompiled chunk");
        } else if self.read_u8() != chunk::LUAC_VERSION {
            panic!("version mismatched");
        } else if self.read_u8() != chunk::LUAC_FORMAT {
            panic!("format mismatched");
        } else if self.read_bytes(6).as_ref() != &chunk::LUAC_DATA {
            panic!("corrupted");
        } else if self.read_u8() != chunk::CINT_SIZE {
            panic!("int size mismatched");
        } else if self.read_u8() != chunk::C_SIZE_T_SIZE {
            panic!("size_t size mismatched");
        } else if self.read_u8() != chunk::INSTRUCTION_SIZE {
            panic!("instruction size mismatched");
        } else if self.read_u8() != chunk::LUA_INTEGER_SIZE {
            panic!("lua integer size mismatched");
        } else if self.read_u8() != chunk::LUA_NUMBER_SIZE {
            panic!("lua number size mismatched");
        } else if self.read_lua_int() != chunk::LUAC_INT {
            panic!("luac_int mismatched");
        } else if self.read_lua_num() != chunk::LUAC_NUM {
            panic!("lua_num mismatched");
        }
    }

    fn read_code(&mut self) -> Vec<u32> {
        let size = self.read_u32();
        let mut codes = Vec::with_capacity(size as usize);
        for _ in 0..size {
            codes.push(self.read_u32());
        }

        codes
    }

    fn read_constants(&mut self) -> Vec<Constant> {
        let size = self.read_u32();
        let mut constants = Vec::with_capacity(size as usize);
        for _ in 0..size {
            constants.push(self.read_constant());
        }
        constants
    }

    fn read_constant(&mut self) -> Constant {
        match self.read_u8() {
            chunk::TAG_NIL => Constant::Nil,
            chunk::TAG_BOOLEAN => Constant::Boolean(self.read_u8() != 0),
            chunk::TAG_INTEGER => Constant::Integer(self.read_lua_int()),
            chunk::TAG_NUMBER => Constant::Number(self.read_lua_num()),
            chunk::TAG_SHORT_STR => Constant::Str(self.read_string()),
            chunk::TAG_LONG_STR => Constant::Str(self.read_string()),
            _ => Constant::Nil
        }
    }

    fn read_upvalues(&mut self) -> Vec<UpValue> {
        let size = self.read_u32();
        let mut upvalues = Vec::with_capacity(size as usize);
        for _ in 0..size {
            upvalues.push(
                UpValue {
                    instack: self.read_u8(),
                    idx: self.read_u8(),
                }
            );
        }

        upvalues
    }

    fn read_upvalue_names(&mut self) -> Vec<String> {
        let size = self.read_u32();
        let mut upvalue_names = Vec::with_capacity(size as usize);
        for _ in 0..size {
            upvalue_names.push(self.read_string());
        }

        upvalue_names
    }

    fn read_loc_vars(&mut self) -> Vec<LocVar> {
        let size = self.read_u32();
        let mut loc_vars = Vec::with_capacity(size as usize);
        for _ in 0..size {
            loc_vars.push(
                LocVar {
                    var_name: self.read_string(),
                    start_pc: self.read_u32(),
                    end_pc: self.read_u32(),
                }
            );
        }

        loc_vars
    }

    fn read_line_info(&mut self) -> Vec<u32> {
        let size = self.read_u32();
        let mut line_infos = Vec::with_capacity(size as usize);
        for _ in 0..size {
            line_infos.push(self.read_u32());
        }

        line_infos
    }

    fn read_protos(&mut self, parent_source: String) -> Vec<Prototype> {
        let size = self.read_u32();
        let mut protos = Vec::with_capacity(size as usize);
        for _ in 0..size {
            protos.push(self.read_proto(parent_source.clone()))
        }

        protos
    }

    pub fn read_proto(&mut self, parent_source: String) -> Prototype {
        let mut source = self.read_string();
        if source.is_empty() {
            source = parent_source;
        }

        Prototype {
            line_defined: self.read_u32(),
            last_line_defined: self.read_u32(),
            num_params: self.read_u8(),
            is_vararg: self.read_u8(),
            max_stack_size: self.read_u8(),
            code: self.read_code(),
            constants: self.read_constants(),
            upvalues: self.read_upvalues(),
            protos: self.read_protos(source.clone()),
            line_info: self.read_line_info(),
            loc_vars: self.read_loc_vars(),
            upvalue_names: self.read_upvalue_names(),
            source,
        }
    }
}

#[cfg(test)]
mod test {
    use crate::binary::chunk;
    use crate::binary::reader::Reader;
    
    #[test]
    fn test_read_u8() {
        let mut reader = Reader::new(&chunk::LUA_SIGNATURE);
        let b = reader.read_u8();
        assert_eq!(b, 0x1B);
        let b = reader.read_u8();
        assert_eq!(b, b'L');
        let b = reader.read_u8();
        assert_eq!(b, b'u');
        let b = reader.read_u8();
        assert_eq!(b, b'a');
    }

    #[test]
    fn test_read_string_null() {
        let string = [0x00];
        let mut reader = Reader::new(&string);
        let result = reader.read_string();
        assert_eq!(result, "".to_string());
    }

    #[test]
    fn test_read_string_lte_0xfd() {
        let string = [0x0B, b'h', b'e', b'l', b'l', b'o', b'w', b'o', b'r', b'l', b'd'];
        let mut reader = Reader::new(&string);
        let result = reader.read_string();
        assert_eq!(result, "helloworld".to_string());
    }

    #[test]
    fn test_read_string_gte_0xff() {
        let string = [0xFF, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x0B, b'h', b'e', b'l', b'l', b'o', b'w', b'o', b'r', b'l', b'd'];
        let mut reader = Reader::new(&string);
        let result = reader.read_string();
        assert_eq!(result, "helloworld".to_string());
    }

    #[test]
    fn test_check_header() {
        let mut string: Vec<u8> = Vec::new();
        string.push(0x1B);
        string.push(b'L');
        string.push(b'u');
        string.push(b'a');
        string.push(0x53);
        string.push(0x00);
        string.push(0x19);
        string.push(0x93);
        string.push(b'\r');
        string.push(b'\n');
        string.push(0x1A);
        string.push(b'\n');
        string.push(0x04);
        string.push(0x08);
        string.push(0x04);
        string.push(0x08);
        string.push(0x08);
        // mac use little endian
        string.extend((0x5678 as i64).to_le_bytes());
        string.extend((370.5 as f64).to_le_bytes());
        let mut reader = Reader::new(&string);
        reader.check_header();
    }

    #[test]
    #[should_panic]
    fn test_check_header_panic() {
        let string = [0x1B, b'L', b'u', b'1'];
        let mut reader = Reader::new(&string);
        reader.check_header();
    }
}