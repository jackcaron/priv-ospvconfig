use rspirv::dr::{ Instruction, Operand };
use rspirv::spirv::{ Dim, ImageFormat, Op, StorageClass, Word };
use schemas::ospv::{
  ConstantValues, ConstantValuesFloat32, ConstantValuesFloat64, ConstantValuesInt32, ConstantValuesInt64,
  Image, ImageDepth, ImageDim, ImageFormatRef, ImageSampled,
  StorageClassRef,
  Type, TypeAccelerationStructure, TypeArray, TypeBool, TypeFloat, TypeImage,
  TypeInt, TypeMatrix, TypePointer, TypeRuntimeArray, TypeSampledImage, TypeSampler,
  TypeSpecConstant, TypeSpecConstantBool, TypeStruct, TypeUnknown, TypeVector, TypeVoid
};

//-----------------------------------------------------
type ByteSize = u8;

pub fn get_id_ref(op: &Operand) -> Word {
  if let Operand::IdRef(val) = *op { val } else { 0 }
}

fn get_op_int32(op: &Operand, default: u32) -> u32 {
  if let Operand::LiteralInt32(val) = *op { val } else { default }
}

#[repr(C)]
union b32 {
  u: u32,
  i: i32
}

#[repr(C)]
union b64 {
  u: u64,
  i: i64,
  hl: (i32, i32)
}

fn get_constant_value(op: &Operand) -> ConstantValues {
  match *op {
    Operand::LiteralInt32(x) => {
      let value = unsafe { let b = b32 { u: x }; b.i };
      ConstantValues::Int32(ConstantValuesInt32 { value })
    },
    Operand::LiteralInt64(x) => {
      let (low, high) = unsafe { let b = b64 { u: x }; b.hl };
      ConstantValues::Int64(ConstantValuesInt64 { high, low })
    },
    Operand::LiteralFloat32(x) => ConstantValues::Float32(ConstantValuesFloat32 { value: x }),
    Operand::LiteralFloat64(x) => ConstantValues::Float64(ConstantValuesFloat64 { value: x }),
    _ => ConstantValues::Int32(ConstantValuesInt32 { value: 0 })
  }
}

fn extract_image_depth(depth: u32) -> ImageDepth {
  match depth {
    1 => ImageDepth::Depth,
    2 => ImageDepth::Unknown,
    _ => ImageDepth::NotDepth
  }
}

fn extract_image_dim(dim: Dim) -> ImageDim {
  match dim {
    Dim::Dim1D => ImageDim::Dim1d,
    Dim::Dim2D => ImageDim::Dim2d,
    Dim::Dim3D => ImageDim::Dim3d,
    Dim::DimBuffer => ImageDim::DimBuffer,
    Dim::DimCube => ImageDim::DimCube,
    Dim::DimRect => ImageDim::DimRect,
    Dim::DimSubpassData => ImageDim::DimSubpassData
  }
}

fn extract_image_format(format: ImageFormat) -> ImageFormatRef {
  match format {
    ImageFormat::R11fG11fB10f => ImageFormatRef::R11fG11fB10f,
    ImageFormat::R16 => ImageFormatRef::R16,
    ImageFormat::R16Snorm => ImageFormatRef::R16snorm,
    ImageFormat::R16f => ImageFormatRef::R16f,
    ImageFormat::R16i => ImageFormatRef::R16i,
    ImageFormat::R16ui => ImageFormatRef::R16ui,
    ImageFormat::R32f => ImageFormatRef::R32f,
    ImageFormat::R32i => ImageFormatRef::R32i,
    ImageFormat::R32ui => ImageFormatRef::R32ui,
    ImageFormat::R64i => ImageFormatRef::R64i,
    ImageFormat::R64ui => ImageFormatRef::R64ui,
    ImageFormat::R8 => ImageFormatRef::R8,
    ImageFormat::R8Snorm => ImageFormatRef::R8snorm,
    ImageFormat::R8i => ImageFormatRef::R8i,
    ImageFormat::R8ui => ImageFormatRef::R8ui,
    ImageFormat::Rg16 => ImageFormatRef::Rg16,
    ImageFormat::Rg16Snorm => ImageFormatRef::Rg16snorm,
    ImageFormat::Rg16f => ImageFormatRef::Rg16f,
    ImageFormat::Rg16i => ImageFormatRef::Rg16i,
    ImageFormat::Rg16ui => ImageFormatRef::Rg16ui,
    ImageFormat::Rg32f => ImageFormatRef::Rg32f,
    ImageFormat::Rg32i => ImageFormatRef::Rg32i,
    ImageFormat::Rg32ui => ImageFormatRef::Rg32ui,
    ImageFormat::Rg8 => ImageFormatRef::Rg8,
    ImageFormat::Rg8Snorm => ImageFormatRef::Rg8snorm,
    ImageFormat::Rg8i => ImageFormatRef::Rg8i,
    ImageFormat::Rg8ui => ImageFormatRef::Rg8ui,
    ImageFormat::Rgb10A2 => ImageFormatRef::Rgb10a2,
    ImageFormat::Rgb10a2ui => ImageFormatRef::Rgb10a2ui,
    ImageFormat::Rgba16 => ImageFormatRef::Rgba16,
    ImageFormat::Rgba16Snorm => ImageFormatRef::Rgba16snorm,
    ImageFormat::Rgba16f => ImageFormatRef::Rgba16f,
    ImageFormat::Rgba16i => ImageFormatRef::Rgba16i,
    ImageFormat::Rgba16ui => ImageFormatRef::Rgba16ui,
    ImageFormat::Rgba32f => ImageFormatRef::Rgba32f,
    ImageFormat::Rgba32i => ImageFormatRef::Rgba32i,
    ImageFormat::Rgba32ui => ImageFormatRef::Rgba32ui,
    ImageFormat::Rgba8 => ImageFormatRef::Rgba8,
    ImageFormat::Rgba8Snorm => ImageFormatRef::Rgba8snorm,
    ImageFormat::Rgba8i => ImageFormatRef::Rgba8i,
    ImageFormat::Rgba8ui => ImageFormatRef::Rgba8ui,
    ImageFormat::Unknown => ImageFormatRef::Unknown
  }
}

fn extract_image_sample(sampled: u32) -> ImageSampled {
  match sampled {
    1 => ImageSampled::Sampler,
    2 => ImageSampled::NoSampler,
    _ => ImageSampled::RunTime
  }
}

fn extract_storage_class(class: StorageClass) -> StorageClassRef {
  match class {
    StorageClass::AtomicCounter => StorageClassRef::AtomicCounter,
    StorageClass::CallableDataKHR => StorageClassRef::CallableDataNv,
    StorageClass::CodeSectionINTEL => StorageClassRef::CodeSectionIntel,
    StorageClass::CrossWorkgroup => StorageClassRef::CrossWorkgroup,
    StorageClass::Function => StorageClassRef::Function,
    StorageClass::Generic => StorageClassRef::Generic,
    StorageClass::HitAttributeKHR => StorageClassRef::HitAttributeNv,
    StorageClass::Image => StorageClassRef::Image,
    StorageClass::IncomingCallableDataKHR => StorageClassRef::IncomingCallableDataNv,
    StorageClass::IncomingRayPayloadKHR => StorageClassRef::IncomingRayPayloadNv,
    StorageClass::Input => StorageClassRef::Input,
    StorageClass::Output => StorageClassRef::Output,
    StorageClass::PhysicalStorageBuffer => StorageClassRef::PhysicalStorageBuffer,
    StorageClass::Private => StorageClassRef::Private,
    StorageClass::PushConstant => StorageClassRef::PushConstant,
    StorageClass::RayPayloadKHR => StorageClassRef::RayPayloadNv,
    StorageClass::ShaderRecordBufferKHR => StorageClassRef::ShaderRecordBufferNv,
    StorageClass::StorageBuffer => StorageClassRef::StorageBuffer,
    StorageClass::Uniform => StorageClassRef::Uniform,
    StorageClass::UniformConstant => StorageClassRef::UniformConstant,
    StorageClass::Workgroup => StorageClassRef::Workgroup
  }
}

//-----------------------------------------------------
pub fn extract_type(inst: &Instruction) -> Type {
  match inst.class.opcode {
    Op::TypeVoid => Type::Void(TypeVoid {}),
    Op::TypeBool => Type::Bool(TypeBool {}),

    Op::TypeFloat => {
      let bit_depth = get_op_int32(&inst.operands[0], 32) as ByteSize;
      Type::Float(TypeFloat { size: bit_depth })
    },
    Op::TypeInt => {
      let bit_depth = get_op_int32(&inst.operands[0], 32) as ByteSize;
      let signed = get_op_int32(&inst.operands[1], 0) != 0;
      Type::Int(TypeInt { signed, size: bit_depth })
    },

    Op::TypeVector => {
      let ref_ = get_id_ref(&inst.operands[0]);
      let size = get_op_int32(&inst.operands[1], 0) as ByteSize;
      Type::Vector(TypeVector { ref_, size })
    },
    Op::TypeMatrix => {
      let ref_ = get_id_ref(&inst.operands[0]);
      let size = get_op_int32(&inst.operands[1], 0) as ByteSize;
      Type::Matrix(TypeMatrix { ref_, size })
    },

    Op::TypeStruct => {
      let types = inst.operands.iter().map(get_id_ref).collect();
      Type::Struct(TypeStruct { refs: types })
    },

    Op::TypePointer => {
      let class: StorageClassRef = if let Operand::StorageClass(class) = inst.operands[0] { extract_storage_class(class) } else { StorageClassRef::UniformConstant };
      let ref_ = get_id_ref(&inst.operands[1]);
      Type::Pointer(TypePointer { ref_, class })
    },

    Op::TypeArray => {
      let ref_ = get_id_ref(&inst.operands[0]);
      let size = get_id_ref(&inst.operands[1]);
      Type::Array(TypeArray { ref_, size })
    },
    Op::TypeRuntimeArray => {
      let ref_ = get_id_ref(&inst.operands[0]);
      Type::RuntimeArray(TypeRuntimeArray { ref_ })
    },

    Op::TypeImage => {
      let ref_ = get_id_ref(&inst.operands[0]);
      let dim = if let Operand::Dim(dim) = inst.operands[1] { extract_image_dim(dim) } else { ImageDim::Dim1d };
      let depth = extract_image_depth(get_op_int32(&inst.operands[2], 0));
      let arrayed = get_op_int32(&inst.operands[3], 0) != 0;
      let multisampled = get_op_int32(&inst.operands[4], 0) != 0;
      let sampled = extract_image_sample(get_op_int32(&inst.operands[5], 0));
      let format = if let Operand::ImageFormat(format) = inst.operands[6] { extract_image_format(format) } else { ImageFormatRef::Unknown };

      let image = Image { arrayed, depth, dim, format, multisampled, sampled };
      Type::Image(TypeImage { image, ref_ })
    },

    Op::TypeSampler => Type::Sampler(TypeSampler {}),
    Op::TypeSampledImage => {
      let ref_ = get_id_ref(&inst.operands[0]);
      Type::SampledImage(TypeSampledImage { ref_ })
    },

    Op::TypeAccelerationStructureKHR => Type::AccelerationStructure(TypeAccelerationStructure {}),

    Op::SpecConstantTrue => Type::SpecConstantBool(TypeSpecConstantBool { value: true }),
    Op::SpecConstantFalse => Type::SpecConstantBool(TypeSpecConstantBool { value: false }),
    Op::SpecConstant => {
      let ref_ = inst.result_type.unwrap_or(0);
      let value = get_constant_value(&inst.operands[0]);
      Type::SpecConstant(TypeSpecConstant { ref_, value })
    },

    _ => Type::Unknown(TypeUnknown {})
  }
}
