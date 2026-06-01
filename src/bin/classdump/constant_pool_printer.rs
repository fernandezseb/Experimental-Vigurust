use vigur::class_loader::{ConstantPool, ConstantPoolItem};

pub fn resolve_print_cp_item_named(item: &ConstantPoolItem, cp: &ConstantPool) -> String {
    match item {
        ConstantPoolItem::CPMethodRef { class_index: _class_index, name_and_type_index: _name_and_type_index } =>
            {
                format!(" Method {}", resolve_print_cp_item(item, cp))
            }
        _ => {
            String::from("CP item type not found")
        }
    }
}

pub fn resolve_print_cp_item(item: &ConstantPoolItem, cp: &ConstantPool) -> String {
    match item {
        ConstantPoolItem::CPMethodRef {class_index, name_and_type_index} =>
            {
                format!("{}.{}",
                        resolve_print_cp_item(cp.constants.get(*class_index as usize).unwrap(), &cp),
                        resolve_print_cp_item(cp.constants.get(*name_and_type_index as usize).unwrap(), &cp)
                )
            },
        ConstantPoolItem::CPClassInfo {name_index} => {
            format!("{}", cp.get_string(*name_index))
        },
        ConstantPoolItem::CPNameAndTypeInfo {name_index, descriptor_index} => {
            let name = cp.get_string(*name_index);
            if name == "<init>" {
                format!("\"{}\":{}", name, cp.get_string(*descriptor_index))
            } else {
                format!("{}:{}", name, cp.get_string(*descriptor_index))
            }
        }
        _ => {
            String::from("CP item type not found")
        }
    }
}