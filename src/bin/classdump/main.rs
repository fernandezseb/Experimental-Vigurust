use vigur::class_loader::ClassLoader;
use crate::class_printer::ClassPrinter;

mod class_printer;
mod code_printer;
mod constant_pool_printer;

fn main() {
    let class_info = ClassLoader::load_class("./Main.class");
    ClassPrinter::print_class(&class_info);
}