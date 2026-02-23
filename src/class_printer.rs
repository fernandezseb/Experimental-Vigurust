use std::{fs, path::Path};

use time::format_description;

use crate::class_loader::{AttributeInfo, ClassInfo, ConstantPool, ConstantPoolItem, MethodInfo};

pub struct ClassPrinter {
}

impl ClassPrinter {
    fn print_constant_pool(constant_pool: &ConstantPool) {
        println!("Constant pool:");
        for (pos, cp_item) in constant_pool.constants.iter().skip(1).enumerate() {
            let number_formatted = format!("#{}", pos+1);
            let output = match cp_item {
                ConstantPoolItem::CPUTF8Info{utf8_string} => {
                    format!("{:15} {}", "Utf8", utf8_string.clone())
                },
                ConstantPoolItem::CPMethodRef { class_index, name_and_type_index } => {
                    let class_info = constant_pool.get_class_info(*class_index);
                    let class_name = constant_pool.get_string(class_info);
                    let (name_index, descriptor_index) = constant_pool.get_name_and_type(*name_and_type_index);
                    let method_name = constant_pool.get_string(name_index);
                    let method_descriptor = constant_pool.get_string(descriptor_index);
                    let indices = format!("#{}.#{}", class_index, name_and_type_index);
                    format!("{:15} {:15} // {}.\"{}\"{}", "Methodref", indices, class_name, method_name, method_descriptor)
                },
                ConstantPoolItem::CPClassInfo { name_index } => {
                    let index = format!("#{}", name_index);
                    let name = constant_pool.get_string(*name_index);
                    format!("{:15} {:15} // {}", "Class", index, name)
                },
                ConstantPoolItem::CPNameAndTypeInfo { name_index, descriptor_index } => {
                    let indices = format!("#{}:#{}", name_index, descriptor_index);
                    let name = constant_pool.get_string(*name_index);
                    let descriptor = constant_pool.get_string(*descriptor_index);
                    format!("{:15} {:15} // \"{}\":{}", "NameAndType", indices, name, descriptor)
                }
                other => {
                    "".to_string()
                }
            };
            println!("{:>4} = {}", number_formatted, output);
        }
    }

    fn char_to_type(c: char) -> &'static str {
        match c {
            'V' => "void",
            'B' => "byte",
            'C' => "char",
            'D' => "double",
            'F' => "float",
            'I' => "int",
            'J' => "long",
            'S' => "short",
            'Z' => "boolean",
            other => "unknown"
        }
    }
    fn get_as_external_type(type_str: &str) -> String {
        let mut buffer = String::new();
        let mut arr_count = 0;
        let mut in_class = false;

        for c in type_str.chars() {
            match c {
                'L' => {
                    in_class = true;
                },
                ';' => {
                    in_class = false;
                },
                '[' => {
                    arr_count += 1;
                },
                '/' if in_class => {
                    buffer.push('.');
                }
                other if in_class => {
                    buffer.push(other);
                }
                other if !in_class => {
                    buffer += Self::char_to_type(other);
                },
                other => buffer.push(other)
            }
        }

        for i in 0..arr_count {
            buffer += "[]";
        }

        buffer
    }

    fn print_method(method: &MethodInfo, constant_pool: &ConstantPool, class_name: &str) {
        let is_constructor = method.is_constructor(constant_pool);
        let name = if is_constructor {class_name} else {method.get_name(constant_pool)};
        let mut args = String::from("");
        for (number, arg) in method.args.iter().enumerate() {
            if number != 0 {args += ", "};
            args+= &Self::get_as_external_type(&arg);
        }
        let mut return_type = if is_constructor {String::from("")} else {Self::get_as_external_type(&method.return_type)};
        if return_type.len() > 0 { return_type += " "}
        let mut flags = String::from("");
        let mut keywords = String::from("");
        for (name, bitflags) in method.access_flags.iter_names() {
            flags += name;
            flags += ", ";
            keywords += bitflags.as_keyword();
            keywords += " ";
        }
        let flags_cut = &flags[0..flags.len()-2];
        println!("  {}{}{}({}):", keywords, return_type, name, args);
        println!("    descriptor: {}", constant_pool.get_string(method.descriptor_index));
        println!("    flags: {}", flags_cut);
    }

    fn print_methods(class_info: &ClassInfo, class_name: &str) {
        for method in &class_info.methods {
            Self::print_method(method, &class_info.constant_pool, class_name);
        }
    }

    pub fn print_class(class_info: &ClassInfo) {
        let absolute_path = fs::canonicalize(Path::new(&class_info.file_path)).unwrap();
        println!("Classfile {}", absolute_path.display());
        let format_description = format_description::parse("[day] [month repr:short] [year]").unwrap();
        let modified_string = class_info.last_modified.format(&format_description);
        println!("  Last modified {}; size {} bytes", modified_string.unwrap(), class_info.size);
        println!("  SHA-256 checksum {}", class_info.hash);
        for att in &class_info.attributes {
            match att {
                AttributeInfo::ATSourceFile { source_file_index } => {
                    let source_file = class_info.constant_pool.get_string(*source_file_index);
                    println!("  Compiled from \"{}\"", source_file);
                },
                other => {
                }
            }

        }
        let class_name_index = class_info.constant_pool.get_class_info(class_info.this_class);
        let class_name = class_info.constant_pool.get_string(class_name_index);
        println!("class {}", class_name);
        println!("  minor version: {}", class_info.minor_version);
        println!("  major version: {}", class_info.major_version);
        println!("  this_class: #{}", class_info.this_class);
        println!("  super_class: #{}", class_info.super_class);
        println!("  interfaces: {}, fields: {}, methods: {}, attributes: {}",
            class_info.interfaces.len(), class_info.fields.len(), class_info.methods.len(), class_info.attributes.len());
        Self::print_constant_pool(&class_info.constant_pool);
        println!("{{");
        Self::print_methods(class_info, class_name);
        println!("}}");
    }
}