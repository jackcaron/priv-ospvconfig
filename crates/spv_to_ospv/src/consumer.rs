
use rspirv::binary::{ Consumer, ParseAction };
use rspirv::dr::{ Instruction, ModuleHeader, Operand };
use rspirv::spirv::{ ExecutionModel, Op, Word };
use schemas::ospv::{ ExecModel, ExecutionModel as ExecutionModelRef, Ospv, StructuralDecoration, Type };
use std::collections::HashMap;

use crate::builder::{ OspvBuilder };
use crate::decoration::{ default_structural_decoration, struct_decore_apply_decoration, struct_decore_apply_member_decoration, struct_decore_set_member_name, struct_decore_set_name };
use crate::types::{ extract_type, get_id_ref };

//-----------------------------------------------------
fn map_execution_model(model: ExecutionModel) -> ExecutionModelRef {
  match model {
    ExecutionModel::AnyHitKHR => ExecutionModelRef::AnyHitNv,
    ExecutionModel::CallableKHR => ExecutionModelRef::CallableNv,
    ExecutionModel::ClosestHitKHR => ExecutionModelRef::ClosestHitNv,
    ExecutionModel::Fragment => ExecutionModelRef::Fragment,
    ExecutionModel::Geometry => ExecutionModelRef::Geometry,
    ExecutionModel::GLCompute => ExecutionModelRef::Glcompute,
    ExecutionModel::IntersectionKHR => ExecutionModelRef::IntersectionNv,
    ExecutionModel::Kernel => ExecutionModelRef::Kernel,
    ExecutionModel::MeshNV => ExecutionModelRef::MeshNv,
    ExecutionModel::MissKHR => ExecutionModelRef::MissNv,
    ExecutionModel::RayGenerationKHR => ExecutionModelRef::RayGenerationNv,
    ExecutionModel::TaskNV => ExecutionModelRef::TaskNv,
    ExecutionModel::TessellationControl => ExecutionModelRef::TessellationControl,
    ExecutionModel::TessellationEvaluation => ExecutionModelRef::TessellationEvaluation,
    ExecutionModel::Vertex => ExecutionModelRef::Vertex
  }
}

fn get_exec_model(inst: &Instruction) -> ExecModel {
  let model = if let Operand::ExecutionModel(model) = inst.operands[0] { map_execution_model(model) } else { ExecutionModelRef::Kernel };
  let name = if let Operand::LiteralString(name) = &inst.operands[2] { name.clone() } else { "main".to_string() };

  let parameters = inst.operands.iter().skip(3).map(get_id_ref).collect();
  ExecModel { model, name, parameters }
}

//-----------------------------------------------------
pub struct SpvConsumer {
  type_map: HashMap<Word, Type>,
  decoration_map: HashMap<Word, StructuralDecoration>,
  exec_models: Vec<ExecModel>
}

impl SpvConsumer {
  pub fn new() -> SpvConsumer {
    SpvConsumer {
      type_map: HashMap::new(),
      decoration_map: HashMap::new(),
      exec_models: Vec::new()
    }
  }

  fn create_or_get_decoration(&mut self, id: Word) -> &mut StructuralDecoration {
    self.decoration_map.entry(id).or_insert_with(default_structural_decoration)
  }

  fn consume_decoration(&mut self, inst: &Instruction) {
    let id = get_id_ref(&inst.operands[0]);
    if id == 0 {
      return;
    }

    let struct_deco = self.create_or_get_decoration(id);
    match inst.class.opcode {
      Op::Name => {
        if let Operand::LiteralString(name) = &inst.operands[1] {
          struct_decore_set_name(struct_deco, name);
        }
      },
      Op::MemberName => {
        if let Operand::LiteralInt32(idx) = &inst.operands[1] {
          if let Operand::LiteralString(name) = &inst.operands[2] {
            struct_decore_set_member_name(struct_deco, *idx, name);
          }
        }
      },
      Op::Decorate => {
        struct_decore_apply_decoration(struct_deco, &inst);
      },
      Op::MemberDecorate => {
        if let Operand::LiteralInt32(idx) = &inst.operands[1] {
          struct_decore_apply_member_decoration(struct_deco, *idx, &inst);
        }
      },
      _ => {}
    };
  }

  pub fn to_ospv(&self, file: &str) -> Ospv {
    let mut builder = OspvBuilder::new(file)
            .add_types(&self.type_map)
            .add_entries(&self.exec_models);

    for (ref_id, struct_deco) in &self.decoration_map {
      builder.set_decoration(*ref_id, struct_deco);
    }
    builder.get_ospv()
  }
}

impl Consumer for SpvConsumer {
  fn initialize(&mut self) -> ParseAction {
    ParseAction::Continue
  }

  fn finalize(&mut self) -> ParseAction {
    ParseAction::Continue
  }

  fn consume_header(&mut self, _module: ModuleHeader) -> ParseAction {
    ParseAction::Continue
  }

  fn consume_instruction(&mut self, inst: Instruction) -> ParseAction {
    if let Some(ref_id) = inst.result_id {
      self.type_map.insert(ref_id, extract_type(&inst));
      return ParseAction::Continue;
    }

    if inst.class.opcode == Op::EntryPoint {
      self.exec_models.push(get_exec_model(&inst));
      return ParseAction::Continue;
    }

    if inst.operands.is_empty() {
      return ParseAction::Continue;
    }

    self.consume_decoration(&inst);
    ParseAction::Continue
  }
}
