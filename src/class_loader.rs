use crate::{byte_array::{self, ByteArray}, descriptor_parser::parse_descriptor};

use std::fs;
use cesu8::from_cesu8;
use sha256::digest;
use time::OffsetDateTime;
use crate::class_loader::AttributeInfo::{ATCode, ATLineNumberTable, ATLocalVariableTable, ATSourceFile};
use bitflags::bitflags;

const MAGIC_NUMBER : u32 = 0xCAFEBABE;

const CT_UTF8: u8            = 1;
const CT_INTEGER: u8         = 3;
const CT_FLOAT: u8           = 4;
const CT_LONG: u8            = 5;
const CT_DOUBLE: u8          = 6;
const CT_CLASS: u8           = 7;
const CT_STRING: u8          = 8;
const CT_FIELDREF: u8        = 9;
const CT_METHODREF: u8       = 10;
const CT_INTERFACEMETHOD: u8 = 11;
const CT_NAMEANDTYPE: u8     = 12;
const CT_METHODHANDLE: u8    = 15;
const CT_METHODTYPE: u8      = 16;
const CT_INVOKEDYNAMIC: u8   = 18;

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct MethodFlags: u16 {
        const ACC_PUBLIC = 0x0001;
        const ACC_PRIVATE = 0x0002;
        const ACC_PROTECTED = 0x0004;
        const ACC_STATIC = 0x0008;
        const ACC_FINAL = 0x0010;
        const ACC_SYNCHRONIZED = 0x0020;
        const ACC_BRIDGE = 0x0040;
        const ACC_VARARGS = 0x0080;
        const ACC_NATIVE = 0x0100;
        const ACC_ABSTRACT = 0x0400;
        const ACC_STRICT = 0x0800;
        const ACC_SYNTHETIC = 0x1000;
    }
}

impl MethodFlags {
    pub fn as_keyword(&self) -> &'static str {
        match *self {
            Self::ACC_PUBLIC => "public",
            Self::ACC_PRIVATE => "private",
            Self::ACC_PROTECTED => "protected",
            Self::ACC_STATIC => "static",
            Self::ACC_FINAL => "final",
            Self::ACC_SYNCHRONIZED => "synchronized",
            Self::ACC_BRIDGE => "bridge",
            Self::ACC_VARARGS => "varargs",
            Self::ACC_NATIVE => "native",
            Self::ACC_ABSTRACT => "abstract",
            Self::ACC_STRICT => "strict",
            Self::ACC_SYNTHETIC => "SYNTHETIC",
            other => "unknown"
        }
    }
}

pub struct ClassInfo {
    pub constant_pool: ConstantPool,
    pub file_path: String,
    pub size: usize,
    pub last_modified: OffsetDateTime,
    pub minor_version: u16,
    pub major_version: u16,
    pub access_flags: u16,
    pub this_class: u16,
    pub super_class: u16,
    pub interfaces: Vec<u16>,
    pub fields: Vec<FieldInfo>,
    pub methods: Vec<MethodInfo>,
    pub attributes: Vec<AttributeInfo>,
    pub source_file: String,
    pub hash: String
    // static_fields_count: u16,
}

pub struct MethodInfo {
    pub access_flags: MethodFlags,
    pub descriptor_index: u16,
    pub attributes: Vec<AttributeInfo>,
    pub return_type: String,
    pub args: Vec<String>,
    name_index: u16,
}

impl MethodInfo {
    pub fn get_name<'a>(self: &Self, constant_pool: &'a ConstantPool) -> &'a str {
        return &constant_pool.get_string(self.name_index);
    }

    pub fn is_constructor(&self, constant_pool: &ConstantPool) -> bool {
        self.get_name(constant_pool) == "<init>"
    }
}

pub struct ConstantPool {
    pub constants: Vec<ConstantPoolItem>
}

impl ConstantPool {
    pub fn get_string(self: &ConstantPool, index: u16) -> &str {
        // TODO: Check index?
        let item = &self.constants[(index) as usize];
        match item {
            ConstantPoolItem::CPUTF8Info { utf8_string } => {
                utf8_string
            },
            other => panic!("No string found in constantpool at index: {}", index)
        }
    }
    
    pub fn get_class_info(self: &ConstantPool, index: u16) -> u16 {
        let item = &self.constants[(index) as usize];
        match item {
            ConstantPoolItem::CPClassInfo { name_index} => {
                *name_index
            },
            other => panic!("No ClassInfo found in constantpool at index: {}", index)
        }
    }

    pub fn get_name_and_type(self: &ConstantPool, index: u16) -> (u16,u16) {
        let item = &self.constants[(index) as usize];
        match item {
            ConstantPoolItem::CPNameAndTypeInfo { name_index, descriptor_index} => {
                (*name_index, *descriptor_index)
            },
            other => panic!("No ClassInfo found in constantpool at index: {}", index)
        }
    }
}

pub enum AttributeInfo {
    ATLineNumberTable{entries: Vec<LineNumberTableEntry>},
    ATCode{
        max_stack: u16,
        max_locals: u16,
        code: Vec<u8>,
        exceptions: Vec<ExceptionTableEntry>,
        attributes: Vec<AttributeInfo>,
    },
    ATLocalVariableTable{entries: Vec<LocalVariableTableEnty>},
    ATSourceFile{source_file_index: u16},
}

pub struct FieldInfo {
    access_flags: u16,
    name_index: u16,
    desciptor_index: u16,
    is_private: bool,
    attributes: Vec<AttributeInfo>
}



pub struct ClassLoader {
}

pub enum ConstantPoolItem {
    CPMethodRef{class_index: u16, name_and_type_index: u16},
    CPClassInfo{name_index: u16},
    CPUTF8Info{utf8_string: String},
    CPFieldRef{class_index: u16, name_and_type_index: u16},
    CPStringInfo{string_index: u16},
    CPNameAndTypeInfo{name_index: u16, descriptor_index: u16}
}

struct AttributeParser {
}

pub struct ExceptionTableEntry {
    start_pc: u16,
    end_pc: u16,
    handler_pc: u16,
    catch_type: u16,
}

pub struct LineNumberTableEntry {
    start_pc: u16,
    line_number: u16,
}

pub struct LocalVariableTableEnty {
    start_pc: u16,
    length: u16,
    name_index: u16,
    descriptor_index: u16,
    index: u16,
}

impl AttributeParser {
    fn read_exception_table_entry(byte_array: &mut ByteArray) -> ExceptionTableEntry {
        let start_pc = byte_array.read_u16();
        let end_pc = byte_array.read_u16();
        let handler_pc = byte_array.read_u16();
        let catch_type = byte_array.read_u16();

        ExceptionTableEntry{start_pc, end_pc, handler_pc, catch_type}
    }
    fn read_exception_table(byte_array: &mut ByteArray) -> Vec<ExceptionTableEntry> {
        let exception_table_length = byte_array.read_u16() as usize;
        let mut exception_table: Vec<ExceptionTableEntry> = Vec::with_capacity(exception_table_length);
        for _current_exception in 0..exception_table_length
        {
            exception_table.push(Self::read_exception_table_entry(byte_array));
        }
        exception_table
    }

    fn read_attributes(byte_array: &mut ByteArray, constant_pool: &ConstantPool) -> Vec<AttributeInfo> {
        let count = byte_array.read_u16() as usize;
        let mut vec = Vec::with_capacity(count);
        for _current_attribute in 0..count {
            let attribute_name_index = byte_array.read_u16();
            let attribute_length = byte_array.read_u32();
            let name = constant_pool.get_string(attribute_name_index);
            match name {
                "Code" => {
                    let max_stack = byte_array.read_u16();
                    let max_locals = byte_array.read_u16();
                    let code_length = byte_array.read_u32() as usize;
                    let code = byte_array.read_bytes(code_length);
                    let code_vec = code.to_vec().clone();
                    let exceptions = Self::read_exception_table(byte_array);
                    let attributes = Self::read_attributes(byte_array, constant_pool);
                    vec.push(ATCode {
                        max_stack,
                        max_locals,
                        code: code_vec,
                        exceptions,
                        attributes
                    });
                },
                "LineNumberTable" => {
                    let line_number_table_length = byte_array.read_u16() as usize;
                    let mut entries = Vec::with_capacity(line_number_table_length);
                    for _line_number_table_entry in 0..line_number_table_length {
                        let start_pc = byte_array.read_u16();
                        let line_number = byte_array.read_u16();
                        entries.push(LineNumberTableEntry{start_pc, line_number, });
                    }
                    vec.push(ATLineNumberTable {entries});
                }
                "LocalVariableTable" => {
                    let local_variable_table_length = byte_array.read_u16() as usize;
                    let mut entries = Vec::with_capacity(local_variable_table_length);
                    for _local_variable_table_entry_index in 0..local_variable_table_length {
                        let start_pc = byte_array.read_u16();
                        let length = byte_array.read_u16();
                        let name_index = byte_array.read_u16();
                        let descriptor_index = byte_array.read_u16();
                        let index = byte_array.read_u16();
                        entries.push(LocalVariableTableEnty{
                            start_pc,
                            length,
                            name_index,
                            descriptor_index,
                            index
                        })
                    }
                    vec.push(ATLocalVariableTable {entries});
                },
                "SourceFile" => {
                    let source_file_index = byte_array.read_u16();
                    vec.push(ATSourceFile {source_file_index});
                }
                other => panic!("Unknown attribute {} found", name)
            }
        }
        vec
    }
}


impl ClassLoader {
    fn read_constant_pool_item(tag: u8, byte_array: &mut ByteArray) -> ConstantPoolItem {
        match tag {
            CT_METHODREF => {
                let class_index = byte_array.read_u16();
                let name_and_type_index = byte_array.read_u16();
                ConstantPoolItem::CPMethodRef {
                    class_index,
                    name_and_type_index
                }
            },
            CT_CLASS => {
                let name_index = byte_array.read_u16();
                ConstantPoolItem::CPClassInfo {
                    name_index
                }
            },
            CT_FIELDREF => {
                let class_index = byte_array.read_u16();
                let name_and_type_index = byte_array.read_u16();
                ConstantPoolItem::CPFieldRef {
                    class_index,
                    name_and_type_index
                }
            },
            CT_STRING => {
                let string_index = byte_array.read_u16();
                ConstantPoolItem::CPStringInfo {
                    string_index
                }
            },
            CT_UTF8 => {
                let size = byte_array.read_u16() as usize;
                let bytes: &[u8] = byte_array.read_bytes(size);
                let cpitem = ConstantPoolItem::CPUTF8Info{
                    utf8_string:  from_cesu8(bytes).unwrap().to_string()
                };
                cpitem
            },
            CT_NAMEANDTYPE => {
                let name_index = byte_array.read_u16();
                let descriptor_index = byte_array.read_u16();
                ConstantPoolItem::CPNameAndTypeInfo {
                    name_index,
                    descriptor_index
                }
            },
            other => panic!("Unknown tag {} read in Class", tag)
        }
    }

    fn read_constant_pool(byte_array: &mut ByteArray) -> ConstantPool {
        let cp_count: usize = byte_array.read_u16() as usize;

        let mut constant_pool: ConstantPool = ConstantPool { constants: Vec::with_capacity(cp_count) };

        // Add filler
        constant_pool.constants.push(ConstantPoolItem::CPClassInfo { name_index: 0 });

        for _current_index in 0..(cp_count-1) {
            let tag = byte_array.read_u8();
            constant_pool.constants.push(Self::read_constant_pool_item(tag, byte_array));
        }

        constant_pool

    }

    fn read_interfaces(byte_array: &mut ByteArray) -> Vec<u16> {
        let count = byte_array.read_u16() as usize;
        let mut vec: Vec<u16> = Vec::with_capacity(count);
        for _current_interface in 0..count {
            vec.push(byte_array.read_u16());
        }
        vec
    }

    fn read_fields(byte_array: &mut ByteArray) -> Vec<FieldInfo> {
        let count = byte_array.read_u16() as usize;
        let mut vec: Vec<FieldInfo> = Vec::with_capacity(count);
        // TODO: Implement
        vec
    }

    fn read_methods(byte_array: &mut ByteArray, constant_pool: &ConstantPool) -> Vec<MethodInfo> {
        let count = byte_array.read_u16() as usize;
        let mut vec: Vec<MethodInfo> = Vec::with_capacity(count);
        for _current_method in 0..count {
            let access_flags = byte_array.read_u16();
            let name_index = byte_array.read_u16();
            let descriptor_index = byte_array.read_u16();
            let attributes = AttributeParser::read_attributes(byte_array, constant_pool);
            let descriptor = parse_descriptor(constant_pool.get_string(descriptor_index).to_string());

            let method_flags : MethodFlags = MethodFlags::from_bits_truncate(access_flags);

            vec.push(MethodInfo{
                access_flags: method_flags,
                descriptor_index,
                attributes,
                return_type: descriptor.return_type,
                args: descriptor.args,
                name_index
            });
        }
        vec
    } 

    
    pub fn load_class(path: &str) -> ClassInfo {
        let mut byte_array = ByteArray::new (
            fs::read(path).unwrap(),
            0
        );
        let metadata = fs::metadata(path).unwrap();
        let modified: OffsetDateTime = metadata.modified().unwrap().into();
        //println!("{}", modified_string.unwrap());
        let result = byte_array.read_u32();
        assert_eq!(result, MAGIC_NUMBER);
        let minor_version = byte_array.read_u16();
        let major_version = byte_array.read_u16();
        let constant_pool = Self::read_constant_pool(&mut byte_array);
        let access_flags = byte_array.read_u16();
        let this_class = byte_array.read_u16();
        let super_class = byte_array.read_u16();
        let interfaces = Self::read_interfaces(&mut byte_array);
        let fields = Self::read_fields(&mut byte_array);
        let methods = Self::read_methods(&mut byte_array, &constant_pool);
        let attributes = AttributeParser::read_attributes(&mut byte_array, &constant_pool);
        ClassInfo {
            constant_pool: constant_pool,
            file_path: String::from(path),
            size: byte_array.len(),
            last_modified: modified,
            minor_version,
            major_version,
            access_flags,
            this_class,
            super_class,
            interfaces,
            fields,
            methods,
            attributes,
            source_file: String::from("TODO"),
            hash: digest(byte_array.bytes),
        }
    }
}