use crate::ospv::{ * };

//-----------------------------------------------------
impl VarDecorationSetBind {
  fn box_set(set: u32) -> Box<Self> {
    Box::new(Self { set, binding: 0 })
  }

  fn box_binding(binding: u32) -> Box<Self> {
    Box::new(Self { set: 0, binding })
  }
}

//-----------------------------------------------------
impl VarDecorationMatrix {
  fn box_row() -> Box<Self> {
    Box::new(VarDecorationMatrix::Row( VarDecorationMatrixRow { value: 0 }))
  }

  fn box_column() -> Box<Self> {
    Box::new(VarDecorationMatrix::Column(VarDecorationMatrixColumn { value: 0 }))
  }

  fn set_stride(&mut self, stride: u32) {
    match self {
      VarDecorationMatrix::Column(x) => x.value = stride,
      VarDecorationMatrix::Row(x) => x.value = stride,
    }
  }
}

//-----------------------------------------------------
impl Default for VarDecoration {
  fn default() -> Self {
    VarDecoration { array_stride: None, block_type: None, location: None, matrix: None, name: None, offset: None, relaxed_precision: None, set_bind: None, spec_id: None, visibility: None }
  }
}

impl VarDecoration {
  pub fn set_name(&mut self, name: &String) {
    if !name.is_empty() {
      self.name = Some(Box::new(name.clone()));
    }
  }

  pub fn relax_precision(&mut self) {
    self.relaxed_precision = Some(Box::new(true));
  }

  pub fn set_binding(&mut self, binding: u32) {
    match &mut self.set_bind {
      Some(sb) => sb.as_mut().binding = binding,
      None => self.set_bind = Some(VarDecorationSetBind::box_binding(binding))
    }
  }

  pub fn set_descriptor_set(&mut self, set: u32) {
    match &mut self.set_bind {
      Some(sb) => sb.as_mut().set = set,
      None => self.set_bind = Some(VarDecorationSetBind::box_set(set))
    }
  }

  pub fn set_spec_id(&mut self, id: u32) {
    self.spec_id = Some(Box::new(id));
  }

  pub fn set_location(&mut self, loc: u32) {
    self.location = Some(Box::new(loc));
  }

  pub fn set_matrix_row(&mut self) {
    self.matrix = Some(VarDecorationMatrix::box_row());
  }

  pub fn set_matrix_column(&mut self) {
    self.matrix = Some(VarDecorationMatrix::box_column());
  }

  pub fn set_matrix_stride(&mut self, stride: u32) {
    if let Some(sb) = &mut self.matrix {
      sb.as_mut().set_stride(stride);
    }
  }

  pub fn set_block_type(&mut self, block: VarDecorationBlockType) {
    self.block_type = Some(Box::new(block));
  }

  pub fn set_array_stride(&mut self, stride: u32) {
    self.array_stride = Some(Box::new(stride));
  }

  pub fn set_visibility(&mut self, visibility: VarDecorationVisibility) {
    self.visibility = Some(Box::new(visibility));
  }

  pub fn set_offset(&mut self, offset: u32) {
    self.offset = Some(Box::new(offset));
  }
}

//-----------------------------------------------------
impl Default for StructuralDecoration {
  fn default() -> Self {
    StructuralDecoration { decoration: VarDecoration::default(), members: None }
  }
}

impl StructuralDecoration {
  pub fn create_or_get_member(&mut self, idx: u32) -> &mut VarDecoration {
    if matches!(self.members, None) {
      self.members = Some(Box::new(Vec::new()));
    }

    let members = self.members.as_mut().unwrap().as_mut();
    let us_idx = idx as usize;
    loop {
      if us_idx < members.len() {
        break;
      }
      members.push(VarDecoration::default());
    };
    &mut members[us_idx]
  }

  pub fn set_name(&mut self, name: &String) {
    self.decoration.set_name(name);
  }

  pub fn set_member_name(&mut self, idx: u32, name: &String) {
    self.create_or_get_member(idx).set_name(name);
  }
}
