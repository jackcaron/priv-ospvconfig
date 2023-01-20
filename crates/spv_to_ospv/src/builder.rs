use schemas::ospv::{
  ExecModel,
  Ospv,
  StructuralDecoration,
  Type, TypeRuntimeArray, TypeSampledImage, TypeStruct
};
use std::collections::{ HashMap };

//-----------------------------------------------------
fn default_ospv(filename: &str) -> Ospv {
  Ospv {
    decoration: HashMap::new(),
    entries: Vec::new(),
    source_file: filename.to_string(),
    types: Vec::new()
  }
}

//-----------------------------------------------------
pub(crate) struct OspvBuilder {
  type_hash: HashMap<u32, u32>,
  ospv: Ospv
}

impl OspvBuilder {
  pub fn new(filename: &str) -> Self {
    Self {
      type_hash: HashMap::new(),
      ospv: default_ospv(filename)
    }
  }

  fn insert_type(&mut self, idx: &u32, tp: Type) -> u32 {
    let nb = self.ospv.types.len() as u32;
    self.ospv.types.push(tp);
    self.type_hash.insert(*idx, nb);
    nb
  }

  fn add_type(&mut self, idx: &u32, tp: &Type) -> u32 {
    if !self.type_hash.contains_key(idx) {
      self.insert_type(idx, tp.clone())
    }
    else {
      *self.type_hash.get(idx).unwrap()
    }
  }

  fn add_types_recurse(&mut self, idx: &u32, types: &HashMap<u32, Type>) -> u32 {
    if let Some(tp) = types.get(idx) {
      match tp {
        Type::AccelerationStructure(_) |
        Type::Bool(_) |
        Type::Void(_) |
        Type::Float(_) |
        Type::Int(_) |
        Type::Sampler(_) |
        Type::SpecConstantBool(_) => self.add_type(idx, tp),

        Type::Vector(v) => {
          let mut new = v.clone();
          new.ref_ = self.add_types_recurse(&v.ref_, types);
          self.insert_type(idx, Type::Vector(new))
        },

        Type::Matrix(m) => {
          let mut new = m.clone();
          new.ref_ = self.add_types_recurse(&m.ref_, types);
          self.insert_type(idx, Type::Matrix(new))
        },

        Type::Struct(s) => {
          let refs = s.refs.iter().map(|rf| self.add_types_recurse(rf, types)).collect();
          self.insert_type(idx, Type::Struct(TypeStruct { refs }))
        },

        Type::Pointer(p) => {
          let mut new = p.clone();
          new.ref_ = self.add_types_recurse(&p.ref_, types);
          self.insert_type(idx, Type::Pointer(new))
        },

        Type::Array(a) => {
          let mut new = a.clone();
          new.ref_ = self.add_types_recurse(&a.ref_, types);
          self.insert_type(idx, Type::Array(new))
        },

        Type::RuntimeArray(r) => {
          let ref_ = self.add_types_recurse(&r.ref_, types);
          self.insert_type(idx, Type::RuntimeArray(TypeRuntimeArray { ref_ }))
        },

        Type::Image(i) => {
          let mut new = i.clone();
          new.ref_ = self.add_types_recurse(&i.ref_, types);
          self.insert_type(idx, Type::Image(new))
        },

        Type::SampledImage(s) => {
          let ref_ = self.add_types_recurse(&s.ref_, types);
          self.insert_type(idx, Type::SampledImage(TypeSampledImage { ref_ }))
        },

        Type::Variable(v) => {
          let mut new = v.clone();
          new.ref_ = self.add_types_recurse(&v.ref_, types);
          self.insert_type(idx, Type::Variable(new))
        },

        Type::SpecConstant(s) => {
          let mut new = s.clone();
          new.ref_ = self.add_types_recurse(&s.ref_, types);
          self.insert_type(idx, Type::SpecConstant(new))
        },

        _ => 0xffffffff
      }
    }
    else {
      0xffffffff
    }
  }

  pub fn add_types(mut self, types: &HashMap<u32, Type>) -> Self {
    for id in types.keys() {
      self.add_types_recurse(id, types);
    }
    self
  }

  pub fn add_entries(mut self, entries: &Vec<ExecModel>) -> Self {
    self.ospv.entries = entries
            .iter()
            .map(|x| ExecModel {
              model: x.model.clone(),
              name: x.name.clone(),
              parameters: x.parameters.iter().map(|i| *self.type_hash.get(i).unwrap()).collect()
            }).collect();
    self
  }

  pub fn set_decoration(&mut self, idx: u32, struct_decore: &StructuralDecoration) {
    if let Some(ridx) = self.type_hash.get(&idx) {
      self.ospv.decoration.insert(ridx.to_string(), struct_decore.clone());
    }
  }

  pub fn get_ospv(self) -> Ospv {
    self.ospv
  }
}
