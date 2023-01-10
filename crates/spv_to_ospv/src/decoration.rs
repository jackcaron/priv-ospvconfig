use rspirv::dr::{ Instruction, Operand };
use rspirv::spirv::{ Decoration };
use schemas::ospv::{
  StructuralDecoration,
  VarDecoration, VarDecorationBlockType, VarDecorationMatrix, VarDecorationMatrixRow, VarDecorationMatrixColumn, VarDecorationSetBind, VarDecorationVisibility
};

//-----------------------------------------------------
// var decoration
pub fn decoration_set_name(decoration: &mut VarDecoration, name: &String) {
  if !name.is_empty() {
    decoration.name = Some(Box::new(name.clone()));
  }
}

pub fn decoration_relax_precision(decoration: &mut VarDecoration) {
  decoration.relaxed_precision = Some(Box::new(true));
}

pub fn decoration_set_binding(decoration: &mut VarDecoration, binding: u32) {
  match &mut decoration.set_bind {
    Some(sb) => sb.as_mut().binding = binding,
    None => decoration.set_bind = Some(Box::new( VarDecorationSetBind { binding, set: 0 } ))
  }
}

pub fn decoration_set_descriptor_set(decoration: &mut VarDecoration, set: u32) {
  match &mut decoration.set_bind {
    Some(sb) => sb.as_mut().set = set,
    None => decoration.set_bind = Some(Box::new( VarDecorationSetBind { binding: 0, set } ))
  }
}

pub fn decoration_set_spec_id(decoration: &mut VarDecoration, id: u32) {
  decoration.spec_id = Some(Box::new(id));
}

pub fn decoration_set_location(decoration: &mut VarDecoration, loc: u32) {
  decoration.location = Some(Box::new(loc));
}

pub fn decoration_set_matrix_row(decoration: &mut VarDecoration) {
  decoration.matrix = Some(Box::new(VarDecorationMatrix::Row( VarDecorationMatrixRow { value: 0 })));
}

pub fn decoration_set_matrix_column(decoration: &mut VarDecoration) {
  decoration.matrix = Some(Box::new(VarDecorationMatrix::Column(VarDecorationMatrixColumn { value: 0 })));
}

pub fn decoration_set_matrix_stride(decoration: &mut VarDecoration, stride: u32) {
  if let Some(sb) = &mut decoration.matrix {
    match &mut sb.as_mut() {
      VarDecorationMatrix::Column(x) => x.value = stride,
      VarDecorationMatrix::Row(x) => x.value = stride
    }
  }
}

pub fn decoration_set_block_type(decoration: &mut VarDecoration, block: VarDecorationBlockType) {
  decoration.block_type = Some(Box::new(block));
}

pub fn decoration_set_array_stride(decoration: &mut VarDecoration, stride: u32) {
  decoration.array_stride = Some(Box::new(stride));
}

pub fn decoration_set_visibility(decoration: &mut VarDecoration, visibility: VarDecorationVisibility) {
  decoration.visibility = Some(Box::new(visibility));
}

pub fn decoration_set_offset(decoration: &mut VarDecoration, offset: u32) {
  decoration.offset = Some(Box::new(offset));
}

fn default_decoration() -> VarDecoration {
  VarDecoration { array_stride: None, block_type: None, location: None, matrix: None, name: None, offset: None, relaxed_precision: None, set_bind: None, spec_id: None, visibility: None }
}

//-----------------------------------------------------
fn apply_decoration(var_decor: &mut VarDecoration, inst: &Instruction, op_idx: usize) {
  if let Operand:: Decoration(decor) = inst.operands[op_idx] {
    match decor {
      Decoration::RelaxedPrecision => decoration_relax_precision(var_decor),
      Decoration::SpecId => {
        if let Operand:: LiteralInt32(spec_id) = inst.operands[op_idx + 1] {
          decoration_set_spec_id(var_decor, spec_id);
        }
      },
      Decoration::Location => {
        if let Operand:: LiteralInt32(loc) = inst.operands[op_idx + 1] {
          decoration_set_location(var_decor, loc);
        }
      },

      Decoration::Block => decoration_set_block_type(var_decor, VarDecorationBlockType::Block),
      Decoration::BufferBlock => decoration_set_block_type(var_decor, VarDecorationBlockType::BufferBlock),

      Decoration::RowMajor => decoration_set_matrix_row(var_decor),
      Decoration::ColMajor => decoration_set_matrix_column(var_decor),
      Decoration::MatrixStride => {
        if let Operand:: LiteralInt32(stride) = inst.operands[op_idx + 1] {
          decoration_set_matrix_stride(var_decor, stride);
        }
      },

      Decoration::ArrayStride => {
        if let Operand:: LiteralInt32(stride) = inst.operands[op_idx + 1] {
          decoration_set_array_stride(var_decor, stride);
        }
      },

      Decoration::NonWritable => decoration_set_visibility(var_decor, VarDecorationVisibility::ReadOnly),
      Decoration::NonReadable => decoration_set_visibility(var_decor, VarDecorationVisibility::WriteOnly),

      Decoration::Binding => {
        if let Operand:: LiteralInt32(binding) = inst.operands[op_idx + 1] {
          decoration_set_binding(var_decor, binding);
        }
      },
      Decoration::DescriptorSet => {
        if let Operand:: LiteralInt32(set) = inst.operands[op_idx + 1] {
          decoration_set_descriptor_set(var_decor, set);
        }
      },

      Decoration::Offset => {
        if let Operand:: LiteralInt32(offset) = inst.operands[op_idx + 1] {
          decoration_set_offset(var_decor, offset);
        }
      },

      _ => { }
    }
  }
}

//-----------------------------------------------------
fn struct_decore_create_or_get_member(st_decore: &mut StructuralDecoration, idx: u32) -> &mut VarDecoration {
  if matches!(st_decore.members, None) {
    st_decore.members = Some(Box::new(Vec::new()));
  }

  let members = st_decore.members.as_mut().unwrap().as_mut();

  let us_idx = idx as usize;
  loop {
    if us_idx < members.len() {
      break;
    }
    members.push(default_decoration());
  };
  &mut members[us_idx]
}

pub fn struct_decore_set_name(st_decore: &mut StructuralDecoration, name: &String) {
  decoration_set_name(&mut st_decore.decoration, name);
}

pub fn struct_decore_set_member_name(st_decore: &mut StructuralDecoration, idx: u32, name: &String) {
  decoration_set_name(struct_decore_create_or_get_member(st_decore, idx), name);
}

pub fn struct_decore_apply_decoration(st_decore: &mut StructuralDecoration, inst: &Instruction) {
  apply_decoration(&mut st_decore.decoration, inst, 1);
}

pub fn struct_decore_apply_member_decoration(st_decore: &mut StructuralDecoration, idx: u32, inst: &Instruction) {
  apply_decoration(struct_decore_create_or_get_member(st_decore, idx), inst, 2);
}

//-----------------------------------------------------
pub fn default_structural_decoration() -> StructuralDecoration {
  StructuralDecoration { decoration: default_decoration(), members: None }
}
