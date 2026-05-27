use crate::class_loader::{ConstantPool, ConstantPoolItem};

pub fn resolve_print_cp_item_named(item: &ConstantPoolItem, cp: &ConstantPool) {
    match item {
        ConstantPoolItem::CPMethodRef {class_index, name_and_type_index} =>
            {
                print!(" Method ");
                resolve_print_cp_item(item, cp);
            }
        _ => {
            print!("CP item type not found");
        }
    }
}

pub fn resolve_print_cp_item(item: &ConstantPoolItem, cp: &ConstantPool) {
    match item {
        ConstantPoolItem::CPMethodRef {class_index, name_and_type_index} =>
            {
                resolve_print_cp_item(cp.constants.get(*class_index as usize).unwrap(), &cp);
                print!(".");
                resolve_print_cp_item(cp.constants.get(*name_and_type_index as usize).unwrap(), &cp);
            },
        ConstantPoolItem::CPClassInfo {name_index} => {
            print!("{}", cp.get_string(*name_index));
        },
        ConstantPoolItem::CPNameAndTypeInfo {name_index, descriptor_index} => {
            let name = cp.get_string(*name_index);
            if name == "<init>" {
                print!("\"{}\":{}", name, cp.get_string(*descriptor_index));
            } else {
                print!("{}:{}", name, cp.get_string(*descriptor_index));
            }
        }
        _ => {
            print!("CP item type not found");
        }
    }
}