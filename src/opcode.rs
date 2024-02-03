pub type Opcode = u8;

/// Push the null object reference to the stack.
pub const ACONST_NULL: Opcode = 0x01;

/// Load reference from array.
// pub const AALOAD: Opcode = 0x32;

/// Store into reference array.
// pub const AASTORE: Opcode = 0x53;

/// Load reference from local variable.
pub const ALOAD: Opcode = 0x19;
pub const ALOAD_0: Opcode = 0x2a;
pub const ALOAD_1: Opcode = 0x2b;
pub const ALOAD_2: Opcode = 0x2c;
pub const ALOAD_3: Opcode = 0x2d;

/// Create a new array of reference.
// pub const ANEWARRAY: Opcode = 0xbd;

/// Return reference from method.
pub const ARETURN: Opcode = 0xb0;

/// Get length of array.
// pub const ARRAYLENGTH: Opcode = 0xbe;

/// Store reference into local variable.
pub const ASTORE: Opcode = 0x3a;
pub const ASTORE_0: Opcode = 0x4b;
pub const ASTORE_1: Opcode = 0x4c;
pub const ASTORE_2: Opcode = 0x4d;
pub const ASTORE_3: Opcode = 0x4e;

/// Throw exception or error.
pub const ATHROW: Opcode = 0xbf;

/// Load byte or boolean from array.
pub const BALOAD: Opcode = 0x33;

/// Store into byte or boolean array.
pub const BASTORE: Opcode = 0x54;

/// Push byte.
pub const BIPUSH: Opcode = 0x10;

/// Load char from array.
pub const CALOAD: Opcode = 0x34;

/// Store into char array.
pub const CASTORE: Opcode = 0x55;

/// Check whether object is of given type.
pub const CHECKCAST: Opcode = 0xc0;

/// Convert double to float.
pub const D2F: Opcode = 0x90;

/// Convert double to int.
pub const D2I: Opcode = 0x8e;

/// Convert double to long.
pub const D2L: Opcode = 0x8f;

/// Add double.
pub const DADD: Opcode = 0x63;

/// Load double from array.
pub const DALOAD: Opcode = 0x31;

/// Store into double array.
pub const DASTORE: Opcode = 0x52;

/// Compare double.
pub const DCMPG: Opcode = 0x98;

/// Compare double.
pub const DCMPL: Opcode = 0x97;

/// Push double.
pub const DCONST_0: Opcode = 0xe;

/// Push double.
pub const DCONST_1: Opcode = 0xf;

/// Divide double.
pub const DDIV: Opcode = 0x6f;

/// Load double from local variable.
pub const DLOAD: Opcode = 0x18;
pub const DLOAD_0: Opcode = 0x26;
pub const DLOAD_1: Opcode = 0x27;
pub const DLOAD_2: Opcode = 0x28;
pub const DLOAD_3: Opcode = 0x29;

/// Multiply double.
pub const DMUL: Opcode = 0x6b;

/// Negate double.
pub const DNEG: Opcode = 0x77;

/// Remainder of double.
pub const DREM: Opcode = 0x73;

/// Return double from method.
pub const DRETURN: Opcode = 0xaf;

/// Store double into local variable.
pub const DSTORE: Opcode = 0x39;
pub const DSTORE_0: Opcode = 0x47;
pub const DSTORE_1: Opcode = 0x48;
pub const DSTORE_2: Opcode = 0x49;
pub const DSTORE_3: Opcode = 0x4a;

/// Subtract double.
pub const DSUB: Opcode = 0x67;

/// Duplicate the top operand stack value.
pub const DUP: Opcode = 0x59;
/// Duplicate the top operand stack value and insert two values down.
pub const DUP_X1: Opcode = 0x5a;
/// Duplicate the top operand stack value and insert two or three values down.
pub const DUP_X2: Opcode = 0x5b;
/// Duplicate the top one or two operand stack values.
pub const DUP2: Opcode = 0x5c;
/// Duplicate the top one or two operand stack values and insert two or three values down.
pub const DUP2_X1: Opcode = 0x5d;
/// Duplicate the top one or two operand stack values and insert two, three, or four values down.
pub const DUP2_X2: Opcode = 0x5e;

/// Convert float to double.
pub const F2D: Opcode = 0x8d;

/// Convert float to int.
pub const F2I: Opcode = 0x8b;

/// Convert float to long.
pub const F2L: Opcode = 0x8c;

/// Add float.
pub const FADD: Opcode = 0x62;

/// Load float from array.
pub const FALOAD: Opcode = 0x30;

/// Store float into array.
pub const FASTORE: Opcode = 0x51;

/// Compare float.
pub const FCMPG: Opcode = 0x96;

/// Compare float.
pub const FCMPL: Opcode = 0x95;

/// Push float.
pub const FCONSTF_0: Opcode = 0xb;
/// Push float.
pub const FCONSTF_1: Opcode = 0xc;
/// Push float.
pub const FCONSTF_2: Opcode = 0xd;

/// Divide float.
pub const FDIV: Opcode = 0x6e;

/// Load float from local variable.
pub const FLOAD: Opcode = 0x17;
pub const FLOAD_0: Opcode = 0x22;
pub const FLOAD_1: Opcode = 0x23;
pub const FLOAD_2: Opcode = 0x24;
pub const FLOAD_3: Opcode = 0x25;

/// Multiply float.
pub const FMUL: Opcode = 0x6a;

/// Negate float.
pub const FNEG: Opcode = 0x76;

/// Remainder float.
pub const FREM: Opcode = 0x72;

/// Return float from method.
pub const FRETURN: Opcode = 0xae;

/// Store float into local variable.
pub const FSTORE: Opcode = 0x38;
pub const FSTORE_0: Opcode = 0x43;
pub const FSTORE_1: Opcode = 0x44;
pub const FSTORE_2: Opcode = 0x45;
pub const FSTORE_3: Opcode = 0x46;

/// Subtract float.
pub const FSUB: Opcode = 0x66;

/// Fetch field from object.
pub const GETFIELD: Opcode = 0xb4;

/// Get static field from class.
pub const GETSTATIC: Opcode = 0xb2;

/// Branch always.
pub const GOTO: Opcode = 0xa7;

/// Branch always (wide index).
pub const GOTO_W: Opcode = 0xc8;

/// Convert int to byte.
pub const I2B: Opcode = 0x91;

/// Convert int to char.
pub const I2C: Opcode = 0x92;

/// Convert int to double.
pub const I2D: Opcode = 0x87;

/// Convert int to float.
pub const I2F: Opcode = 0x86;

/// Convert int to long;
pub const I2L: Opcode = 0x85;

/// Convert int to short.
pub const I2S: Opcode = 0x93;

/// Add int.
pub const IADD: Opcode = 0x60;

/// Load int from array.
pub const AILOAD: Opcode = 0x2e;

/// Boolean AND int.
pub const IAND: Opcode = 0x7e;

/// Store into int array.
pub const IASTORE: Opcode = 0x4f;

/// Push int constant.
pub const ICONST_M1: Opcode = 0x2;
/// Push int constant.
pub const ICONST_0: Opcode = 0x3;
/// Push int constant.
pub const ICONST_1: Opcode = 0x4;
/// Push int constant.
pub const ICONST_2: Opcode = 0x5;
/// Push int constant.
pub const ICONST_3: Opcode = 0x6;
/// Push int constant.
pub const ICONST_4: Opcode = 0x7;
/// Push int constant.
pub const ICONST_5: Opcode = 0x8;

/// Divide int.
pub const IDIV: Opcode = 0x6c;

/// Branch if reference comparison succeeds.
pub const IF_ACMPEQ: Opcode = 0xa5;
/// Branch if reference comparison succeeds.
pub const IF_ACMPNE: Opcode = 0xa6;

/// Branch if comparison succeeds.
pub const IF_ICMPEQ: Opcode = 0x9f;
/// Branch if comparison succeeds.
pub const IF_ICMPNE: Opcode = 0xa0;
/// Branch if comparison succeeds.
pub const IF_ICMPLT: Opcode = 0xa1;
/// Branch if comparison succeeds.
pub const IF_ICMPGE: Opcode = 0xa2;
/// Branch if comparison succeeds.
pub const IF_ICMPGT: Opcode = 0xa3;
/// Branch if comparison succeeds.
pub const IF_ICMPLE: Opcode = 0xa4;

/// Branch if int comparison with zero succeeds.
pub const IFEQ: Opcode = 0x99;
/// Branch if int comparison with zero succeeds.
pub const IFNE: Opcode = 0x9a;
/// Branch if int comparison with zero succeeds.
pub const IFLT: Opcode = 0x9b;
/// Branch if int comparison with zero succeeds.
pub const IFGE: Opcode = 0x9c;
/// Branch if int comparison with zero succeeds.
pub const IFGT: Opcode = 0x9d;
/// Branch if int comparison with zero succeeds.
pub const IFLE: Opcode = 0x9e;

/// Branch if reference non null.
pub const IFNONNULL: Opcode = 0xc7;

/// Branch if reference is null.
pub const IFNULL: Opcode = 0xc6;

/// Increment local variable by constant.
pub const IINC: Opcode = 0x84;

/// Load int from local variable.
pub const ILOAD: Opcode = 0x15;
/// Load int from local variable.
pub const ILOAD_0: Opcode = 0x1a;
/// Load int from local variable.
pub const ILOAD_1: Opcode = 0x1b;
/// Load int from local variable.
pub const ILOAD_2: Opcode = 0x1c;
/// Load int from local variable.
pub const ILOAD_3: Opcode = 0x1d;

/// Multiply int.
pub const IMUL: Opcode = 0x68;

/// Negate int.
pub const INEG: Opcode = 0x74;

/// Determine if object is of given type.
pub const INSTANCEOF: Opcode = 0xc1;

/// Invoke dynamic method.
pub const INVOKEDYNAMIC: Opcode = 0xba;

/// Invoke interface method.
pub const INVOKEINTERFACE: Opcode = 0xb9;

/// Invoke instance method; special handling for superclass, private,
/// and instance initialization method invocations.
pub const INVOKESPECIAL: Opcode = 0xb7;

/// Invoke a class (static) method.
pub const INVOKESTATIC: Opcode = 0xb8;

/// Invoke instance method; dispatch based on class.
pub const INVOKEVIRTUAL: Opcode = 0xb6;

/// Boolean or int.
pub const IOR: Opcode = 0x80;

/// Remainder int.
pub const IREM: Opcode = 0x70;

/// Return int from method.
pub const IRETURN: Opcode = 0xac;

/// Shift left int.
pub const ISHL: Opcode = 0x78;

/// Arithimetic shift right int.
pub const ISHR: Opcode = 0x7a;

/// Store int into local variable.
pub const ISTORE: Opcode = 0x36;
/// Store int into local variable.
pub const ISTORE_0: Opcode = 0x3b;
/// Store int into local variable.
pub const ISTORE_1: Opcode = 0x3c;
/// Store int into local variable.
pub const ISTORE_2: Opcode = 0x3d;
/// Store int into local variable.
pub const ISTORE_3: Opcode = 0x3e;

/// Subtract int.
pub const ISUB: Opcode = 0x64;

/// Logical shift right int.
pub const IUSHR: Opcode = 0x7c;

/// Boolean xor int.
pub const IXOR: Opcode = 0x82;

/// Jump subroutine.
pub const JSR: Opcode = 0xa8;

/// Jump subroutine (wide index).
pub const JSR_W: Opcode = 0xc9;

/// Convert long to double.
pub const L2D: Opcode = 0x8a;

/// Convert long to float.
pub const L2F: Opcode = 0x89;

/// Convert long to int.
pub const L2I: Opcode = 0x88;

/// Long add.
pub const LADD: Opcode = 0x61;

/// Load long from array.
pub const LALOAD: Opcode = 0x2f;

/// Boolean and long.
pub const LAND: Opcode = 0x7f;

/// Store into long array.
pub const LASTORE: Opcode = 0x50;

/// Compare long.
pub const LCMP: Opcode = 0x94;

/// Push long constant.
pub const LCONST_0: Opcode = 0x9;
/// Push long constant.
pub const LCONST_1: Opcode = 0xa;

/// Push item from run-time constant pool.
pub const LDC: Opcode = 0x12;

/// Push item from run-time constant pool (wide index).
pub const LDC_W: Opcode = 0x13;

/// Push long or double from run-time constant pool (wide index).
pub const LDC2_W: Opcode = 0x14;

/// Divide long.
pub const LDIV: Opcode =  0x6d;

/// Load long from local variable.
pub const LLOAD: Opcode = 0x16;
/// Load long from local variable.
pub const LLOAD_0: Opcode = 0x1e;
/// Load long from local variable.
pub const LLOAD_1: Opcode = 0x1f;
/// Load long from local variable.
pub const LLOAD_2: Opcode = 0x20;
/// Load long from local variable.
pub const LLOAD_3: Opcode = 0x21;

/// Multiply long.
pub const LMUL: Opcode = 0x69;

/// Negate long.
pub const LNEG: Opcode = 0x75;

/// Access jump table by key match and jump.
pub const LOOKUPSWTICH: Opcode = 0xab;

/// Boolean OR long.
pub const LOR: Opcode = 0x81;

/// Remainder long.
pub const LREM: Opcode = 0x71;

/// Return long from method.
pub const LRETURN: Opcode = 0xad;

/// Shift left long.
pub const LSHL: Opcode = 0x79;

/// Arithimetic shift right long.
pub const LSHR: Opcode = 0x7b;

/// Store long into local variable.
pub const LSTORE: Opcode = 0x37;
/// Store long into local variable.
pub const LSTORE_0: Opcode = 0x3f;
/// Store long into local variable.
pub const LSTORE_1: Opcode = 0x40;
/// Store long into local variable.
pub const LSTORE_2: Opcode = 0x41;
/// Store long into local variable.
pub const LSTORE_3: Opcode = 0x42;

/// Subtract long.
pub const LSUB: Opcode = 0x65;

/// Logical shift right long.
pub const LUSHR: Opcode = 0x7d;

/// Boolean xor long.
pub const LXOR: Opcode = 0x83;

/// Enter monitor for object.
pub const MONITORENTER: Opcode = 0xc2;

/// Exit monitor for object.
pub const MONITOREXIT: Opcode = 0xc3;

/// Create new multidimensional array.
pub const MULTIANEWARRAY: Opcode = 0xc5;

/// Create new object.
pub const NEW: Opcode = 0xbb;

/// Create new array.
pub const NEWARRAY: Opcode = 0xbc;

/// Do nothing.
pub const NOP: Opcode = 0x0;

/// Pop the top operand stack value.
pub const POP: Opcode = 0x57;

/// Pop one or two operand stack values.
pub const POP2: Opcode = 0x58;

/// Set field in object.
pub const PUTFIELD: Opcode = 0xb5;

/// Set static field in class.
pub const PUTSTATIC: Opcode = 0xb3;

/// Return from subroutine.
pub const RET: Opcode = 0xa9;

/// Return void from method.
pub const RETURN: Opcode = 0xb1;

/// Load short from array.
pub const SALOAD: Opcode = 0x35;

/// Store into short array.
pub const SASTORE: Opcode = 0x56;

/// Push short.
pub const SIPUSH: Opcode = 0x11;

/// Swap the top two operand stack values.
pub const SWAP: Opcode = 0x5f;

/// Access jump table by index and jump.
pub const TABLESWITCH: Opcode = 0xaa;

/// Extend local variable index by additional bytes.
pub const WIDE: Opcode = 0xc4;
