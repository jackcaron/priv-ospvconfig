use rspirv::dr::{ Instruction, Operand };
use rspirv::spirv::{ Decoration };
use schemas::ospv::{ StructuralDecoration, VarDecoration, VarDecorationBlockType, VarDecorationVisibility };

//-----------------------------------------------------
fn apply_decoration(var_decor: &mut VarDecoration, inst: &Instruction, op_idx: usize) {
  if let Operand:: Decoration(decor) = inst.operands[op_idx] {
    match decor {
      Decoration::RelaxedPrecision => var_decor.relax_precision(),
      Decoration::SpecId => {
        if let Operand:: LiteralInt32(spec_id) = inst.operands[op_idx + 1] {
          var_decor.set_spec_id(spec_id);
        }
      },
      Decoration::Location => {
        if let Operand:: LiteralInt32(loc) = inst.operands[op_idx + 1] {
          var_decor.set_location(loc);
        }
      },

      Decoration::Block => var_decor.set_block_type(VarDecorationBlockType::Block),
      Decoration::BufferBlock => var_decor.set_block_type(VarDecorationBlockType::BufferBlock),

      Decoration::RowMajor => var_decor.set_matrix_row(),
      Decoration::ColMajor => var_decor.set_matrix_column(),
      Decoration::MatrixStride => {
        if let Operand:: LiteralInt32(stride) = inst.operands[op_idx + 1] {
          var_decor.set_matrix_stride(stride);
        }
      },

      Decoration::ArrayStride => {
        if let Operand:: LiteralInt32(stride) = inst.operands[op_idx + 1] {
          var_decor.set_array_stride(stride);
        }
      },

      Decoration::NonWritable => var_decor.set_visibility(VarDecorationVisibility::ReadOnly),
      Decoration::NonReadable => var_decor.set_visibility(VarDecorationVisibility::WriteOnly),

      Decoration::Binding => {
        if let Operand:: LiteralInt32(binding) = inst.operands[op_idx + 1] {
          var_decor.set_binding(binding);
        }
      },
      Decoration::DescriptorSet => {
        if let Operand:: LiteralInt32(set) = inst.operands[op_idx + 1] {
          var_decor.set_descriptor_set(set);
        }
      },

      Decoration::Offset => {
        if let Operand:: LiteralInt32(offset) = inst.operands[op_idx + 1] {
          var_decor.set_offset(offset);
        }
      },

      _ => { }
    }
  }
}

//-----------------------------------------------------
pub(crate) fn struct_decore_apply_decoration(st_decore: &mut StructuralDecoration, inst: &Instruction) {
  apply_decoration(&mut st_decore.decoration, inst, 1);
}

pub(crate) fn struct_decore_apply_member_decoration(st_decore: &mut StructuralDecoration, idx: u32, inst: &Instruction) {
  apply_decoration(st_decore.create_or_get_member(idx), inst, 2);
}
