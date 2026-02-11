mod class_loader;
mod byte_array;
mod descriptor_parser;
mod class_printer;

use class_loader::ClassLoader;

use class_printer::ClassPrinter;

fn main() {
    let class_info = ClassLoader::load_class("./Main.class");
    ClassPrinter::print_class(&class_info);
}