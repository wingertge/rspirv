// AUTOMATICALLY GENERATED from the SPIR-V JSON grammar:
//   external/spirv.core.grammar.json.
// DO NOT MODIFY!

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Extension {
    pub name: String,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ExtInstImport {
    pub name: String,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ExtInst {
    pub set: spirv::Word,
    pub instruction: u32,
    pub operand_1_operand_2: Vec<spirv::Word>,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MemoryModel {
    pub addressing_model: spirv::AddressingModel,
    pub memory_model: spirv::MemoryModel,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EntryPoint {
    pub execution_model: spirv::ExecutionModel,
    pub entry_point: spirv::Word,
    pub name: String,
    pub interface: Vec<spirv::Word>,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ExecutionMode {
    pub entry_point: spirv::Word,
    pub mode: spirv::ExecutionMode,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Capability {
    pub capability: spirv::Capability,
}
#[derive(Clone, Debug)]
pub struct Function {
    pub function_control: spirv::FunctionControl,
    pub function_type: spirv::Word,
}
#[derive(Clone, Debug)]
pub struct FunctionParameter {}
#[derive(Clone, Debug)]
pub struct FunctionEnd {}
#[derive(Clone, Debug)]
pub struct FunctionCall {
    pub function: spirv::Word,
    pub argument_0_argument_1: Vec<spirv::Word>,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ExecutionModeId {
    pub entry_point: spirv::Word,
    pub mode: spirv::ExecutionMode,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ExtInstWithForwardRefsKHR {
    pub set: spirv::Word,
    pub instruction: u32,
    pub operand_1_operand_2: Vec<spirv::Word>,
}
#[derive(Clone, Debug)]
pub struct CooperativeMatrixPerElementOpNV {
    pub matrix: spirv::Word,
    pub func: spirv::Word,
    pub operands: Vec<spirv::Word>,
}
