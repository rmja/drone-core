#![feature(decl_macro)]

extern crate drone;

use drone::reg;
use drone::reg::prelude::*;
use std as core;

reg!(0xDEAD_BEEF 0x20 TestReg);

fn assert_copy<T: Copy>() {}
fn assert_clone<T: Clone>() {}

fn main() {
  assert_copy::<TestReg<Ur>>();
  //~^ ERROR `TestReg<drone::reg::Ur>: std::marker::Copy` is not satisfied
  assert_clone::<TestReg<Ur>>();
  //~^ ERROR `TestReg<drone::reg::Ur>: std::clone::Clone` is not satisfied
  assert_copy::<TestReg<Sr>>();
  //~^ ERROR `TestReg<drone::reg::Sr>: std::marker::Copy` is not satisfied
  assert_clone::<TestReg<Sr>>();
  //~^ ERROR `TestReg<drone::reg::Sr>: std::clone::Clone` is not satisfied
}