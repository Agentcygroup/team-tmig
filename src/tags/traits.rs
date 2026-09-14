#![allow(non_camel_case_types)]
use crate::tags::types::*;
use crate::tags::error::*;

pub trait tags_Ops {
    fn id(&self) -> tags_Id;
    fn kind(&self) -> tags_Kind;
    fn validate(&self) -> tags_Result<()>;
}
