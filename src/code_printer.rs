use bitflags::bitflags;

use crate::byte_array::ByteArray;

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct Opcode: u8 {
    // CONSTANTS
	const nop = 0x00;
	const aconst_null = 0x01;
	const iconst_m1 = 0x02;
	const iconst_0 = 0x03;
	const iconst_1 = 0x04;
	const iconst_2 = 0x05;
	const iconst_3 = 0x06;
	const iconst_4 = 0x07;
	const iconst_5 = 0x08;
	const lconst_0 = 0x09;
	const lconst_1 = 0x0a;
	const fconst_0 = 0x0b;
	const fconst_1 = 0x0c;
	const fconst_2 = 0x0d;
	const dconst_0 = 0x0e;
	const dconst_1 = 0x0f;
	const bipush = 0x10;
	const sipush = 0x11;
	const ldc = 0x12;
	const ldc_w = 0x13;
	const ldc2_w = 0x14;
	// LOADS
	const iload = 0x15;
	const lload = 0x16;
	const fload = 0x17;
	const dload = 0x18;
	const aload = 0x19;
	const iload_0 = 0x1a;
	const iload_1 = 0x1b;
	const iload_2 = 0x1c;
	const iload_3 = 0x1d;
	const lload_0 = 0x1e;
	const lload_1 = 0x1f;
	const lload_2 = 0x20;
	const lload_3 = 0x21;
	const fload_0 = 0x22;
	const fload_1 = 0x23;
	const fload_2 = 0x24;
	const fload_3 = 0x25;
	const dload_0 = 0x26;
	const dload_1 = 0x27;
	const dload_2 = 0x28;
	const dload_3 = 0x29;
	const aload_0 = 0x2a;
	const aload_1 = 0x2b;
	const aload_2 = 0x2c;
	const aload_3 = 0x2d;
	const iaload = 0x2e;
	const laload = 0x2f;
	const faload = 0x30;
	const daload = 0x31;
	const aaload = 0x32;
	const baload = 0x33;
	const caload = 0x34;
	const saload = 0x35;
	// STORES
	const istore = 0x36;
	const lstore = 0x37;
	const fstore = 0x38;
	const dstore = 0x39;
	const astore = 0x3a;
	const istore_0 = 0x3b;
	const istore_1 = 0x3c;
	const istore_2 = 0x3d;
	const istore_3 = 0x3e;
	const lstore_0 = 0x3f;
	const lstore_1 = 0x40;
	const lstore_2 = 0x41;
	const lstore_3 = 0x42;
	const fstore_0 = 0x43;
	const fstore_1 = 0x44;
	const fstore_2 = 0x45;
	const fstore_3 = 0x46;
	const dstore_0 = 0x47;
	const dstore_1 = 0x48;
	const dstore_2 = 0x49;
	const dstore_3 = 0x4a;
	const astore_0 = 0x4b;
	const astore_1 = 0x4c;
	const astore_2 = 0x4d;
	const astore_3 = 0x4e;
	const iastore = 0x4f;
	const lastore = 0x50;
	const fastore = 0x51;
	const dastore = 0x52;
	const aastore = 0x53;
	const bastore = 0x54;
	const castore = 0x55;
	const sastore = 0x56;
	// STACK
	const pop = 0x57;
	const pop2 = 0x58;
	const dup = 0x59;
	const dup_x1 = 0x5a;
	const dup_x2 = 0x5b;
	const dup2 = 0x5c;
	const dup2_x1 = 0x5d;
	const dup2_x2 = 0x5e;
	const swap = 0x5f;
	// MATH
	const iadd = 0x60;
	const ladd = 0x61;
	const fadd = 0x62;
	const dadd = 0x63;
	const isub = 0x64;
	const lsub = 0x65;
	const fsub = 0x66;
	const dsub = 0x67;
	const imul = 0x68;
	const lmul = 0x69;
	const fmul = 0x6a;
	const dmul = 0x6b;
	const idiv = 0x6c;
	const i_ldiv = 0x6d;
	const fdiv = 0x6e;
	const ddiv = 0x6f;
	const irem = 0x70;
	const lrem = 0x71;
	const frem = 0x72;
	const drem = 0x73;
	const ineg = 0x74;
	const lneg = 0x75;
	const fneg = 0x76;
	const dneg = 0x77;
	const ishl = 0x78;
	const lshl = 0x79;
	const ishr = 0x7a;
	const lshr = 0x7b;
	const iushr = 0x7c;
	const lushr = 0x7d;
	const iand = 0x7e;
	const land = 0x7f;
	const ior = 0x80;
	const lor = 0x81;
	const ixor = 0x82;
	const lxor = 0x83;
	const iinc = 0x84;
	// CONVERSIONS
	const i2l = 0x85;
	const i2f = 0x86;
	const i2d = 0x87;
	const l2i = 0x88;
	const l2f = 0x89;
	const l2d = 0x8a;
	const f2i = 0x8b;
	const f2l = 0x8c;
	const f2d = 0x8d;
	const d2i = 0x8e;
	const d2l = 0x8f;
	const d2f = 0x90;
	const i2b = 0x91;
	const i2c = 0x92;
	const i2s = 0x93;
	const lcmp = 0x94;
	const fcmpl = 0x95;
	const fcmpg = 0x96;
	const dcmpl = 0x97;
	const dcmpg = 0x98;
	// CONTROL
	const ifeq = 0x99;
	const ifne = 0x9a;
	const iflt = 0x9b;
	const ifge = 0x9c;
	const ifgt = 0x9d;
	const ifle = 0x9e;
	const if_icmpeq = 0x9f;
	const if_icmpne = 0xa0;
	const if_icmplt = 0xa1;
	const if_icmpge = 0xa2;
	const if_icmpgt = 0xa3;
	const if_icmple = 0xa4;
	const if_acmpeq = 0xa5;
	const if_acmpne = 0xa6;
	const i_goto = 0xa7;
	const jsr = 0xa8;
	const ret = 0xa9;
	const tableswitch = 0xaa;
	const lookupswitch = 0xab;
	const ireturn = 0xac;
	const lreturn = 0xad;
	const freturn = 0xae;
	const dreturn = 0xaf;
	const areturn = 0xb0;
	const i_return = 0xb1;
	// REFERENCES
	const getstatic = 0xb2;
	const putstatic = 0xb3;
	const getfield = 0xb4;
	const putfield = 0xb5;
	const invokevirtual = 0xb6;
	const invokespecial = 0xb7;
	const invokestatic = 0xb8;
	const invokeinterface = 0xb9;
	const invokedynamic = 0xba;
	const i_new = 0xbb;
	const newarray = 0xbc;
	const anewarray = 0xbd;
	const arraylength = 0xbe;
	const athrow = 0xbf;
	const checkcast = 0xc0;
	const instanceof = 0xc1;
	const monitorenter = 0xc2;
	const monitorexit = 0xc3;
	// EXTENDED
	const wide = 0xc4;
	const multianewarray = 0xc5;
	const ifnull = 0xc6;
	const ifnonnull = 0xc7;
	const goto_w = 0xc8;
	const jsr_w = 0xc9;
	// RESERVED
	const breakpoint = 0xca;
	const unused = 0xcb;
	const impdep1 = 0xfe;
	const impdep2 = 0xff;
    }
}

struct Instruction {
	pub opcode: Opcode,
	pub args: u8,
	pub name: &'static str
}

static INSTRUCTION_PRINTERS: &'static [Instruction] = &[
	// Constants
	Instruction{opcode: Opcode::nop, args: 0, name: "nop"},
	Instruction{opcode: Opcode::aconst_null, args: 0, name: "aconst_null"},
	Instruction{opcode: Opcode::iconst_m1, args: 0, name: "iconst_m1"},
	Instruction{opcode: Opcode::iconst_0, args: 0, name: "iconst_0"},
	Instruction{opcode: Opcode::iconst_1, args: 0, name: "iconst_1"},
	Instruction{opcode: Opcode::iconst_2, args: 0, name: "iconst_2"},
	Instruction{opcode: Opcode::iconst_3, args: 0, name: "iconst_3"},
	Instruction{opcode: Opcode::iconst_4, args: 0, name: "iconst_4"},
	Instruction{opcode: Opcode::iconst_5, args: 0, name: "iconst_5"},
	Instruction{opcode: Opcode::lconst_0, args: 0, name: "lconst_0"},
	Instruction{opcode: Opcode::lconst_1, args: 0, name: "lconst_1"},
	Instruction{opcode: Opcode::fconst_0, args: 0, name: "fconst_0"},
	Instruction{opcode: Opcode::fconst_1, args: 0, name: "fconst_1"},
	Instruction{opcode: Opcode::fconst_2, args: 0, name: "fconst_2"},
	Instruction{opcode: Opcode::dconst_0, args: 0, name: "dconst_0"},
	Instruction{opcode: Opcode::dconst_1, args: 0, name: "dconst_1"},
	Instruction{opcode: Opcode::bipush, args: 1, name: "bipush"},
	Instruction{opcode: Opcode::sipush, args: 2, name: "sipush"},
	Instruction{opcode: Opcode::ldc, args: 1, name: "ldc"},
	Instruction{opcode: Opcode::ldc_w, args: 2, name: "ldc_w"},
	Instruction{opcode: Opcode::ldc2_w, args: 2, name: "ldc2_w"},
	// Loads
	Instruction{opcode: Opcode::iload, args: 1, name: "iload"},
	Instruction{opcode: Opcode::lload, args: 1, name: "lload"},
	Instruction{opcode: Opcode::fload, args: 1, name: "fload"},
	Instruction{opcode: Opcode::dload, args: 1, name: "dload"},
	Instruction{opcode: Opcode::aload, args: 1, name: "aload"},
	Instruction{opcode: Opcode::iload_0, args: 0, name: "iload_0"},
	Instruction{opcode: Opcode::iload_1, args: 0, name: "iload_1"},
	Instruction{opcode: Opcode::iload_2, args: 0, name: "iload_2"},
	Instruction{opcode: Opcode::iload_3, args: 0, name: "iload_3"},
	Instruction{opcode: Opcode::lload_0, args: 0, name: "lload_0"},
	Instruction{opcode: Opcode::lload_1, args: 0, name: "lload_1"},
	Instruction{opcode: Opcode::lload_2, args: 0, name: "lload_2"},
	Instruction{opcode: Opcode::lload_3, args: 0, name: "lload_3"},
	Instruction{opcode: Opcode::fload_0, args: 0, name: "fload_0"},
	Instruction{opcode: Opcode::fload_1, args: 0, name: "fload_1"},
	Instruction{opcode: Opcode::fload_2, args: 0, name: "fload_2"},
	Instruction{opcode: Opcode::fload_3, args: 0, name: "fload_3"},
	Instruction{opcode: Opcode::dload_0, args: 0, name: "dload_0"},
	Instruction{opcode: Opcode::dload_1, args: 0, name: "dload_1"},
	Instruction{opcode: Opcode::dload_2, args: 0, name: "dload_2"},
	Instruction{opcode: Opcode::dload_3, args: 0, name: "dload_3"},
	Instruction{opcode: Opcode::aload_0, args: 0, name: "aload_0"},
	Instruction{opcode: Opcode::aload_1, args: 0, name: "aload_1"},
	Instruction{opcode: Opcode::aload_2, args: 0, name: "aload_2"},
	Instruction{opcode: Opcode::aload_3, args: 0, name: "aload_3"},
	Instruction{opcode: Opcode::iaload, args: 0, name: "iaload"},
	Instruction{opcode: Opcode::laload, args: 0, name: "laload"},
	Instruction{opcode: Opcode::faload, args: 0, name: "faload"},
	Instruction{opcode: Opcode::daload, args: 0, name: "daload"},
	Instruction{opcode: Opcode::aaload, args: 0, name: "aaload"},
	Instruction{opcode: Opcode::baload, args: 0, name: "baload"},
	Instruction{opcode: Opcode::caload, args: 0, name: "caload"},
	Instruction{opcode: Opcode::saload, args: 0, name: "saload"},
	// Stores
	Instruction{opcode: Opcode::istore, args: 1, name: "istore"},
	Instruction{opcode: Opcode::lstore, args: 1, name: "lstore"},
	Instruction{opcode: Opcode::fstore, args: 1, name: "fstore"},
	Instruction{opcode: Opcode::dstore, args: 1, name: "dstore"},
	Instruction{opcode: Opcode::astore, args: 1, name: "astore"},
	Instruction{opcode: Opcode::istore_0, args: 0, name: "istore_0"},
	Instruction{opcode: Opcode::istore_1, args: 0, name: "istore_1"},
	Instruction{opcode: Opcode::istore_2, args: 0, name: "istore_2"},
	Instruction{opcode: Opcode::istore_3, args: 0, name: "istore_3"},
	Instruction{opcode: Opcode::lstore_0, args: 0, name: "lstore_0"},
	Instruction{opcode: Opcode::lstore_1, args: 0, name: "lstore_1"},
	Instruction{opcode: Opcode::lstore_2, args: 0, name: "lstore_2"},
	Instruction{opcode: Opcode::lstore_3, args: 0, name: "lstore_3"},
	Instruction{opcode: Opcode::fstore_0, args: 0, name: "fstore_0"},
	Instruction{opcode: Opcode::fstore_1, args: 0, name: "fstore_1"},
	Instruction{opcode: Opcode::fstore_2, args: 0, name: "fstore_2"},
	Instruction{opcode: Opcode::fstore_3, args: 0, name: "fstore_3"},
	Instruction{opcode: Opcode::dstore_0, args: 0, name: "dstore_0"},
	Instruction{opcode: Opcode::dstore_1, args: 0, name: "dstore_1"},
	Instruction{opcode: Opcode::dstore_2, args: 0, name: "dstore_2"},
	Instruction{opcode: Opcode::dstore_3, args: 0, name: "dstore_3"},
	Instruction{opcode: Opcode::astore_0, args: 0, name: "astore_0"},
	Instruction{opcode: Opcode::astore_1, args: 0, name: "astore_1"},
	Instruction{opcode: Opcode::astore_2, args: 0, name: "astore_2"},
	Instruction{opcode: Opcode::astore_3, args: 0, name: "astore_3"},
	Instruction{opcode: Opcode::iastore, args: 0, name: "iastore"},
	Instruction{opcode: Opcode::lastore, args: 0, name: "lastore"},
	Instruction{opcode: Opcode::fastore, args: 0, name: "fastore"},
	Instruction{opcode: Opcode::dastore, args: 0, name: "dastore"},
	Instruction{opcode: Opcode::aastore, args: 0, name: "aastore"},
	Instruction{opcode: Opcode::bastore, args: 0, name: "bastore"},
	Instruction{opcode: Opcode::castore, args: 0, name: "castore"},
	Instruction{opcode: Opcode::sastore, args: 0, name: "sastore"},
	// Stack
	Instruction{opcode: Opcode::pop, args: 0, name: "pop"},
	Instruction{opcode: Opcode::pop2, args: 0, name: "pop2"},
	Instruction{opcode: Opcode::dup, args: 0, name: "dup"},
	Instruction{opcode: Opcode::dup_x1, args: 0, name: "dup_x1"},
	Instruction{opcode: Opcode::dup_x2, args: 0, name: "dup_x2"},
	Instruction{opcode: Opcode::dup2, args: 0, name: "dup2"},
	Instruction{opcode: Opcode::dup2_x1, args: 0, name: "dup2_x1"},
	Instruction{opcode: Opcode::dup2_x2, args: 0, name: "dup2_x2"},
	Instruction{opcode: Opcode::swap, args: 0, name: "swap"},
	// Math
	Instruction{opcode: Opcode::iadd, args: 0, name: "iadd"}, 
	Instruction{opcode: Opcode::ladd, args: 0, name: "ladd"}, 
	Instruction{opcode: Opcode::fadd, args: 0, name: "fadd"}, 
	Instruction{opcode: Opcode::dadd, args: 0, name: "dadd"}, 
	Instruction{opcode: Opcode::isub, args: 0, name: "isub"}, 
	Instruction{opcode: Opcode::lsub, args: 0, name: "lsub"}, 
	Instruction{opcode: Opcode::fsub, args: 0, name: "fsub"}, 
	Instruction{opcode: Opcode::dsub, args: 0, name: "dsub"}, 
	Instruction{opcode: Opcode::imul, args: 0, name: "imul"}, 
	Instruction{opcode: Opcode::lmul, args: 0, name: "lmul"}, 
	Instruction{opcode: Opcode::fmul, args: 0, name: "fmul"}, 
	Instruction{opcode: Opcode::dmul, args: 0, name: "dmul"}, 
	Instruction{opcode: Opcode::idiv, args: 0, name: "idiv"}, 
	Instruction{opcode: Opcode::i_ldiv, args: 0, name: "i_ldiv"}, 
	Instruction{opcode: Opcode::fdiv, args: 0, name: "fdiv"}, 
	Instruction{opcode: Opcode::ddiv, args: 0, name: "ddiv"}, 
	Instruction{opcode: Opcode::irem, args: 0, name: "irem"}, 
	Instruction{opcode: Opcode::lrem, args: 0, name: "lrem"}, 
	Instruction{opcode: Opcode::frem, args: 0, name: "frem"}, 
	Instruction{opcode: Opcode::drem, args: 0, name: "drem"}, 
	Instruction{opcode: Opcode::ineg, args: 0, name: "ineg"}, 
	Instruction{opcode: Opcode::lneg, args: 0, name: "lneg"}, 
	Instruction{opcode: Opcode::fneg, args: 0, name: "fneg"}, 
	Instruction{opcode: Opcode::dneg, args: 0, name: "dneg"}, 
	Instruction{opcode: Opcode::ishl, args: 0, name: "ishl"}, 
	Instruction{opcode: Opcode::lshl, args: 0, name: "lshl"}, 
	Instruction{opcode: Opcode::ishr, args: 0, name: "ishr"}, 
	Instruction{opcode: Opcode::lshr, args: 0, name: "lshr"}, 
	Instruction{opcode: Opcode::iushr, args: 0, name: "iushr"}, 
	Instruction{opcode: Opcode::lushr, args: 0, name: "lushr"}, 
	Instruction{opcode: Opcode::iand, args: 0, name: "iand"}, 
	Instruction{opcode: Opcode::land, args: 0, name: "land"}, 
	Instruction{opcode: Opcode::ior, args: 0, name: "ior"}, 
	Instruction{opcode: Opcode::lor, args: 0, name: "lor"}, 
	Instruction{opcode: Opcode::ixor, args: 0, name: "ixor"}, 
	Instruction{opcode: Opcode::lxor, args: 0, name: "lxor"}, 
	Instruction{opcode: Opcode::iinc, args: 2, name: "iinc"},

	Instruction{opcode: Opcode::i2l, args: 0, name: "i2l"},
	Instruction{opcode: Opcode::i2f, args: 0, name: "i2f"},
	Instruction{opcode: Opcode::i2d, args: 0, name: "i2d"},
	Instruction{opcode: Opcode::l2i, args: 0, name: "l2i"},
	Instruction{opcode: Opcode::l2f, args: 0, name: "l2f"},
	Instruction{opcode: Opcode::l2d, args: 0, name: "l2d"},
	Instruction{opcode: Opcode::f2i, args: 0, name: "f2i"},
	Instruction{opcode: Opcode::f2l, args: 0, name: "f2l"},
	Instruction{opcode: Opcode::f2d, args: 0, name: "f2d"},
	Instruction{opcode: Opcode::d2i, args: 0, name: "d2i"},
	Instruction{opcode: Opcode::d2l, args: 0, name: "d2l"},
	Instruction{opcode: Opcode::d2f, args: 0, name: "d2f"},
	Instruction{opcode: Opcode::i2b, args: 0, name: "i2b"},
	Instruction{opcode: Opcode::i2c, args: 0, name: "i2c"},
	Instruction{opcode: Opcode::i2s, args: 0, name: "i2s"},
	Instruction{opcode: Opcode::lcmp, args: 0, name: "lcmp"},
	Instruction{opcode: Opcode::fcmpl, args: 0, name: "fcmpl"},
	Instruction{opcode: Opcode::fcmpg, args: 0, name: "fcmpg"},
	Instruction{opcode: Opcode::dcmpl, args: 0, name: "dcmpl"},
	Instruction{opcode: Opcode::dcmpg, args: 0, name: "dcmpg"},
	Instruction{opcode: Opcode::ifeq, args: 2, name: "ifeq"},
	Instruction{opcode: Opcode::ifne, args: 2, name: "ifne"},
	Instruction{opcode: Opcode::iflt, args: 2, name: "iflt"},
	Instruction{opcode: Opcode::ifge, args: 2, name: "ifge"},
	Instruction{opcode: Opcode::ifgt, args: 2, name: "ifgt"},
	Instruction{opcode: Opcode::ifle, args: 2, name: "ifle"},
	Instruction{opcode: Opcode::if_icmpeq, args: 2, name: "if_icmpeq"},
	Instruction{opcode: Opcode::if_icmpne, args: 2, name: "if_icmpne"},
	Instruction{opcode: Opcode::if_icmplt, args: 2, name: "if_icmplt"},
	Instruction{opcode: Opcode::if_icmpge, args: 2, name: "if_icmpge"},
	Instruction{opcode: Opcode::if_icmpgt, args: 2, name: "if_icmpgt"},
	Instruction{opcode: Opcode::if_icmple, args: 2, name: "if_icmple"},
	Instruction{opcode: Opcode::if_acmpeq, args: 2, name: "if_acmpeq"},
	Instruction{opcode: Opcode::if_acmpne, args: 2, name: "if_acmpne"},
	Instruction{opcode: Opcode::i_goto, args: 2, name: "goto"},
	Instruction{opcode: Opcode::jsr, args: 2, name: "jsr"},
	Instruction{opcode: Opcode::ret, args: 1, name: "ret"},
	Instruction{opcode: Opcode::tableswitch, args: 0, name: "tableswitch"},
	Instruction{opcode: Opcode::lookupswitch, args: 0, name: "lookupswitch"},
	Instruction{opcode: Opcode::ireturn, args: 0, name: "ireturn"},
	Instruction{opcode: Opcode::lreturn, args: 0, name: "lreturn"},
	Instruction{opcode: Opcode::freturn, args: 0, name: "freturn"},
	Instruction{opcode: Opcode::dreturn, args: 0, name: "dreturn"},
	Instruction{opcode: Opcode::areturn, args: 0, name: "areturn"},
	Instruction{opcode: Opcode::i_return, args: 0, name: "return"},
	Instruction{opcode: Opcode::getstatic, args: 2, name: "getstatic"},
	Instruction{opcode: Opcode::putstatic, args: 2, name: "putstatic"},
	Instruction{opcode: Opcode::getfield, args: 2, name: "getfield"},
	Instruction{opcode: Opcode::putfield, args: 2, name: "putfield"},
	Instruction{opcode: Opcode::invokevirtual, args: 2, name: "invokevirtual"},
	Instruction{opcode: Opcode::invokespecial, args: 2, name: "invokespecial"},
	Instruction{opcode: Opcode::invokestatic, args: 2, name: "invokestatic"},
	Instruction{opcode: Opcode::invokeinterface, args: 4, name: "invokeinterface"},
	Instruction{opcode: Opcode::invokedynamic, args: 4, name: "invokedynamic"},
	Instruction{opcode: Opcode::i_new, args: 2, name: "i_new"},
	Instruction{opcode: Opcode::newarray, args: 1, name: "newarray"},
	Instruction{opcode: Opcode::anewarray, args: 2, name: "anewarray"},
	Instruction{opcode: Opcode::arraylength, args: 0, name: "arraylength"},
	Instruction{opcode: Opcode::athrow, args: 0, name: "athrow"},
	Instruction{opcode: Opcode::checkcast, args: 2, name: "checkcast"},
	Instruction{opcode: Opcode::instanceof, args: 2, name: "instanceof"},
	Instruction{opcode: Opcode::monitorenter, args: 0, name: "monitorenter"},
	Instruction{opcode: Opcode::monitorexit, args: 0, name: "monitorexit"},
	Instruction{opcode: Opcode::wide, args: 0, name: "wide"},
	Instruction{opcode: Opcode::multianewarray, args: 3, name: "multianewarray"},
	Instruction{opcode: Opcode::ifnull, args: 2, name: "ifnull"},
	Instruction{opcode: Opcode::ifnonnull, args: 2, name: "ifnonnull"},
	Instruction{opcode: Opcode::goto_w, args: 4, name: "goto_w"},
	Instruction{opcode: Opcode::jsr_w, args: 4, name: "jsr_w"},
	Instruction{opcode: Opcode::breakpoint, args: 0, name: "breakpoint"},
	Instruction{opcode: Opcode::unused, args: 0, name: "unused"},
	Instruction{opcode: Opcode::unused, args: 0, name: "unused"},
	Instruction{opcode: Opcode::unused, args: 0, name: "unused"},
	Instruction{opcode: Opcode::unused, args: 0, name: "unused"},
	Instruction{opcode: Opcode::unused, args: 0, name: "unused"},
	Instruction{opcode: Opcode::unused, args: 0, name: "unused"},
	Instruction{opcode: Opcode::unused, args: 0, name: "unused"},
	Instruction{opcode: Opcode::unused, args: 0, name: "unused"},
	Instruction{opcode: Opcode::unused, args: 0, name: "unused"},
	Instruction{opcode: Opcode::unused, args: 0, name: "unused"},
	Instruction{opcode: Opcode::unused, args: 0, name: "unused"},
	Instruction{opcode: Opcode::unused, args: 0, name: "unused"},
	Instruction{opcode: Opcode::unused, args: 0, name: "unused"},
	Instruction{opcode: Opcode::unused, args: 0, name: "unused"},
	Instruction{opcode: Opcode::unused, args: 0, name: "unused"},
	Instruction{opcode: Opcode::unused, args: 0, name: "unused"},
	Instruction{opcode: Opcode::unused, args: 0, name: "unused"},
	Instruction{opcode: Opcode::unused, args: 0, name: "unused"},
	Instruction{opcode: Opcode::unused, args: 0, name: "unused"},
	Instruction{opcode: Opcode::unused, args: 0, name: "unused"},
	Instruction{opcode: Opcode::unused, args: 0, name: "unused"},
	Instruction{opcode: Opcode::unused, args: 0, name: "unused"},
	Instruction{opcode: Opcode::unused, args: 0, name: "unused"},
	Instruction{opcode: Opcode::unused, args: 0, name: "unused"},
	Instruction{opcode: Opcode::unused, args: 0, name: "unused"},
	Instruction{opcode: Opcode::unused, args: 0, name: "unused"},
	Instruction{opcode: Opcode::unused, args: 0, name: "unused"},
	Instruction{opcode: Opcode::unused, args: 0, name: "unused"},
	Instruction{opcode: Opcode::unused, args: 0, name: "unused"},
	Instruction{opcode: Opcode::unused, args: 0, name: "unused"},
	Instruction{opcode: Opcode::unused, args: 0, name: "unused"},
	Instruction{opcode: Opcode::unused, args: 0, name: "unused"},
	Instruction{opcode: Opcode::unused, args: 0, name: "unused"},
	Instruction{opcode: Opcode::unused, args: 0, name: "unused"},
	Instruction{opcode: Opcode::unused, args: 0, name: "unused"},
	Instruction{opcode: Opcode::unused, args: 0, name: "unused"},
	Instruction{opcode: Opcode::unused, args: 0, name: "unused"},
	Instruction{opcode: Opcode::unused, args: 0, name: "unused"},
	Instruction{opcode: Opcode::unused, args: 0, name: "unused"},
	Instruction{opcode: Opcode::unused, args: 0, name: "unused"},
	Instruction{opcode: Opcode::unused, args: 0, name: "unused"},
	Instruction{opcode: Opcode::unused, args: 0, name: "unused"},
	Instruction{opcode: Opcode::unused, args: 0, name: "unused"},
	Instruction{opcode: Opcode::unused, args: 0, name: "unused"},
	Instruction{opcode: Opcode::unused, args: 0, name: "unused"},
	Instruction{opcode: Opcode::unused, args: 0, name: "unused"},
	Instruction{opcode: Opcode::unused, args: 0, name: "unused"},
	Instruction{opcode: Opcode::unused, args: 0, name: "unused"},
	Instruction{opcode: Opcode::unused, args: 0, name: "unused"},
	Instruction{opcode: Opcode::unused, args: 0, name: "unused"},
	Instruction{opcode: Opcode::unused, args: 0, name: "unused"},
	Instruction{opcode: Opcode::impdep1, args: 0, name: "impdep1"},
	Instruction{opcode: Opcode::impdep2, args: 0, name: "impdep2"},
];

pub struct CodePrinter {
	
}

impl CodePrinter {
    pub fn print_code(bytes: &Vec<u8>) {
		let mut byte_array = ByteArray::new(bytes.clone(), 0);
		while byte_array.current < byte_array.len()  {
			let byte = byte_array.read_u8();
			let index = byte as usize;
			if index < INSTRUCTION_PRINTERS.len() {
				let instruction = INSTRUCTION_PRINTERS.get(index).unwrap();
				println!("         {}: {}", byte_array.current - 1, instruction.name);
				byte_array.current = byte_array.current + (instruction.args as usize);
			} else {
				println!("         {}: ?{:02x}", byte_array.current - 1, byte);
			}
		}
	}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn instructions_index_test() {
        for index in 0..INSTRUCTION_PRINTERS.len() {
			let instruction = INSTRUCTION_PRINTERS.get(index).unwrap();
			if instruction.opcode != Opcode::unused {
				assert_eq!(INSTRUCTION_PRINTERS[index].opcode.bits() as usize, index);
			}
		}
    }
}