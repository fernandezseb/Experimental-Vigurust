mod class_loader;
mod byte_array;

use class_loader::ClassLoader;

fn main() {
    let class_info = ClassLoader::load_class("./Main.class");
}