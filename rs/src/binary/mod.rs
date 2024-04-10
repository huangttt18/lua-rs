mod reader;
pub mod chunk;

pub fn undump(data: Vec<u8>) -> chunk::Prototype {
    let mut reader = reader::Reader::new(data.as_ref());
    reader.check_header();
    reader.read_u8();
    reader.read_proto("".to_string())
}