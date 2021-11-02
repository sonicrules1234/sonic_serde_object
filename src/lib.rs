
sonic_serde_macros::sonic_serde_obj!();
/*
//use std::collections::HashMap;
use std::collections::BTreeMap;
use std::collections::HashMap;
use std::time::SystemTime;
use serde::{Deserialize, Serialize};
//use thiserror::Error;
#[derive(Debug, Hash, PartialOrd, Ord, Serialize, Eq, PartialEq, Deserialize, Clone)]
pub enum SonicSerdeObject {
    String(String),
    Vec(Vec<SonicSerdeObject>),
    Map(BTreeMap<SonicSerdeObject, SonicSerdeObject>),
    U8(u8),
    Bool(bool),
    SystemTime(SystemTime),
    VecU8(Vec<u8>),
    //U128(u128),
}

impl SonicSerdeObject {
    pub fn new_vec() -> Self {
        Self::Vec(Vec::new())
    }
    pub fn new_map() -> Self {
        Self::Map(BTreeMap::new())
    }
    pub fn new_map_with(key: impl Into<SonicSerdeObject>, value: impl Into<SonicSerdeObject>) -> Self {
        let mut x = Self::Map(BTreeMap::new());
        x.insert(key.into(), value.into());
        x
    }
    pub fn from_str(val_str: impl Into<String>) -> Self {
        Self::String(val_str.into())
    }
    pub fn is_vec(&self) -> bool {
        match self {
            Self::Vec(_x) => true,
            Self::Map(_x) => false,
            Self::U8(_x) => false,
            Self::String(_x) => false,
            Self::SystemTime(_x) => false,
            Self::Bool(_x) => false,
            Self::VecU8(_x) => false,
        }
    }
    pub fn is_string(&self) -> bool {
        match self {
            Self::Vec(_x) => false,
            Self::Map(_x) => false,
            Self::U8(_x) => false,
            Self::String(_x) => true,
            Self::SystemTime(_x) => false,
            Self::Bool(_x) => false,
            Self::VecU8(_x) => false,
        }
    }
    pub fn is_map(&self) -> bool {
        match self {
            Self::Vec(_x) => false,
            Self::Map(_x) => true,
            Self::U8(_x) => false,
            Self::String(_x) => false,
            Self::SystemTime(_x) => false,
            Self::Bool(_x) => false,
        }
    }
    pub fn is_u8(&self) -> bool {
        match self {
            Self::Vec(_x) => false,
            Self::Map(_x) => false,
            Self::U8(_x) => true,
            Self::String(_x) => false,
            Self::SystemTime(_x) => false,
            Self::Bool(_x) => false,
        }
    }
    pub fn is_bool(&self) -> bool {
        match self {
            Self::Vec(_x) => false,
            Self::Map(_x) => false,
            Self::U8(_x) => false,
            Self::String(_x) => false,
            Self::SystemTime(_x) => false,
            Self::Bool(_x) => true,
        }
    }
    pub fn as_str(&self) -> Result<&str, SonicSerdeObjectError> {
        match self {
            Self::Vec(_x) => Err(SonicSerdeObjectError::NotA("str".to_string())),
            Self::Map(_x) => Err(SonicSerdeObjectError::NotA("str".to_string())),
            Self::U8(_x) => Err(SonicSerdeObjectError::NotA("str".to_string())),
            Self::String(x) => Ok(x.as_str()),
            Self::SystemTime(_x) => Err(SonicSerdeObjectError::NotA("str".to_string())),
            Self::Bool(_x) => Err(SonicSerdeObjectError::NotA("str".to_string())),
        }
    }
    pub fn as_vec(&self) -> Result<Vec<SonicSerdeObject>, SonicSerdeObjectError> {
        match self {
            Self::Vec(x) => Ok(x.clone()),
            Self::Map(_x) => Err(SonicSerdeObjectError::NotA("vec".to_string())),
            Self::U8(_x) => Err(SonicSerdeObjectError::NotA("vec".to_string())),
            Self::String(_x) => Err(SonicSerdeObjectError::NotA("vec".to_string())),
            Self::SystemTime(_x) => Err(SonicSerdeObjectError::NotA("vec".to_string())),
            Self::Bool(_x) => Err(SonicSerdeObjectError::NotA("vec".to_string())),
        }
    }
    pub fn as_map(&self) -> Result<BTreeMap<SonicSerdeObject, SonicSerdeObject>, SonicSerdeObjectError> {
        match self {
            Self::Vec(_x) => Err(SonicSerdeObjectError::NotA("map".to_string())),
            Self::Map(x) => Ok(x.clone()),
            Self::U8(_x) => Err(SonicSerdeObjectError::NotA("map".to_string())),
            Self::String(_x) => Err(SonicSerdeObjectError::NotA("map".to_string())),
            Self::SystemTime(_x) => Err(SonicSerdeObjectError::NotA("map".to_string())),
            Self::Bool(_x) => Err(SonicSerdeObjectError::NotA("map".to_string())),
        }
    }
    pub fn as_system_time(&self) -> Result<SystemTime, SonicSerdeObjectError> {
        match self {
            Self::Vec(_x) => Err(SonicSerdeObjectError::NotA("systemtime".to_string())),
            Self::Map(_x) => Err(SonicSerdeObjectError::NotA("systemtime".to_string())), 
            Self::U8(_x) => Err(SonicSerdeObjectError::NotA("systemtime".to_string())),
            Self::String(_x) => Err(SonicSerdeObjectError::NotA("systemtime".to_string())),
            Self::SystemTime(x) => Ok(x.clone()),
            Self::Bool(_x) => Err(SonicSerdeObjectError::NotA("systemtime".to_string())),
        }
    }
    pub fn as_bool(&self) -> Result<bool, SonicSerdeObjectError> {
        match self {
            Self::Vec(_x) => Err(SonicSerdeObjectError::NotA("bool".to_string())),
            Self::Map(_x) => Err(SonicSerdeObjectError::NotA("bool".to_string())), 
            Self::U8(_x) => Err(SonicSerdeObjectError::NotA("bool".to_string())),
            Self::String(_x) => Err(SonicSerdeObjectError::NotA("bool".to_string())),
            Self::SystemTime(_x) => Err(SonicSerdeObjectError::NotA("bool".to_string())),
            Self::Bool(x) => Ok(x.clone()),
        }
    }
    pub fn push(&mut self, val: impl Into<SonicSerdeObject>) {
        if self.is_vec() {
            let mut y = self.as_vec().unwrap();
            y.push(val.into());
            *self = Self::Vec(y);
        }
    }
    pub fn insert(&mut self, key: impl Into<SonicSerdeObject>, val: impl Into<SonicSerdeObject>) {
        if self.is_map() {
            let mut x = self.as_map().unwrap();
            x.insert(key.into(), val.into());
            *self = Self::Map(x);
        }
    }
}

impl AsMut<SonicSerdeObject> for SonicSerdeObject {
    fn as_mut(&mut self) -> &mut SonicSerdeObject {
        self
    }
}

impl AsRef<SonicSerdeObject> for SonicSerdeObject {
    fn as_ref(&self) -> &SonicSerdeObject {
        self
    }
}

impl From<u8> for SonicSerdeObject {
    fn from(u8_val: u8) -> SonicSerdeObject {
        SonicSerdeObject::U8(u8_val)
    }
}

impl From<bool> for SonicSerdeObject {
    fn from(bool_val: bool) -> SonicSerdeObject {
        SonicSerdeObject::Bool(bool_val)
    }
}

impl From<String> for SonicSerdeObject {
    fn from(string_val: String) -> SonicSerdeObject {
        SonicSerdeObject::String(string_val)
    }
}

impl From<&String> for SonicSerdeObject {
    fn from(string_val: &String) -> SonicSerdeObject {
        SonicSerdeObject::String(string_val.to_string())
    }
}

impl From<&str> for SonicSerdeObject {
    fn from(string_val: &str) -> SonicSerdeObject {
        SonicSerdeObject::String(string_val.to_string())
    }
}

impl<T> From<Vec<T>> for SonicSerdeObject where SonicSerdeObject: std::convert::From<T> {
    fn from(vec_val: Vec<T>) -> SonicSerdeObject {
        let mut out = SonicSerdeObject::new_vec();
        for item in vec_val {
            let x: SonicSerdeObject = item.into();
            out.push(x);
        }
        out
    }
}

impl<K, V> From<HashMap<K, V>> for SonicSerdeObject where SonicSerdeObject: std::convert::From<K>, SonicSerdeObject: std::convert::From<V> {
    fn from(hashmap_val: HashMap<K, V>) -> SonicSerdeObject {
        let mut out: SonicSerdeObject = SonicSerdeObject::new_map();
        for item in hashmap_val.into_iter() {
            let a: SonicSerdeObject = item.0.into();
            let b: SonicSerdeObject = item.1.into();
            out.insert(a, b);
        }
        out
    }
}

impl From<SystemTime> for SonicSerdeObject {
    fn from(sys_time_val: SystemTime) -> SonicSerdeObject {
        SonicSerdeObject::SystemTime(sys_time_val)
    }
}

//#[derive(Debug, Hash, Serialize, Eq, PartialEq, Deserialize, Clone)]
//pub struct SonicSerdeMap {
//    keys: Vec<SonicSerdeObject>,
//    values: Vec<SonicSerdeObject>,
//}
#[derive(Debug)]
pub enum SonicSerdeObjectError {
    NotA(String),
}
*/
/*
impl SonicSerdeMap {
    pub fn new() -> Self {
        Self {
            keys: Vec::<SonicSerdeObject>::new(),
            values: Vec::<SonicSerdeObject>::new(),
        }
    }
    pub fn get(&self, key: SonicSerdeObject) -> Option<SonicSerdeObject> {
        let pos: usize;
        match self.keys.clone().into_iter().position(|x| x == key) {
            Some(x) => {
                Some(self.values[x].clone())
            },
            None => {
                None
                //Err(SonicSerdeObjectError::NoSuchKey("No such key".to_string()))
            }
        }

    }
    pub fn remove(&mut self, key: SonicSerdeObject) -> Option<SonicSerdeObject> {
        match self.keys.clone().into_iter().position(|x| x == key) {
            Some(x) => {
                self.keys.remove(x);
                Some(self.values.remove(x))
            },
            None => {
                None
                //Err(SonicSerdeObjectError::NoSuchKey("No such key".to_string()))
            }
        }
    }
    pub fn insert(&mut self, key: SonicSerdeObject, value: SonicSerdeObject) {
        self.remove(key.clone());
        self.keys.push(key.clone());
        self.values.push(value);
    }
    pub fn contains_key(&self, key: SonicSerdeObject) -> bool {
        self.keys.contains(&key)
    }
    pub fn keys(&self) -> Vec<SonicSerdeObject> {
        self.keys.clone()
    }
}
*/
