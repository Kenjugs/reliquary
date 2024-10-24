// This file is generated by rust-protobuf 3.4.0. Do not edit
// .proto file is parsed by pure
// @generated

// https://github.com/rust-lang/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy::all)]

#![allow(unused_attributes)]
#![cfg_attr(rustfmt, rustfmt::skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unused_results)]
#![allow(unused_mut)]

//! Generated file from `FAJCMNEPFKP.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:FAJCMNEPFKP)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct FAJCMNEPFKP {
    // message oneof groups
    pub HGBJCGHHIHF: ::std::option::Option<fajcmnepfkp::HGBJCGHHIHF>,
    // special fields
    // @@protoc_insertion_point(special_field:FAJCMNEPFKP.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a FAJCMNEPFKP {
    fn default() -> &'a FAJCMNEPFKP {
        <FAJCMNEPFKP as ::protobuf::Message>::default_instance()
    }
}

impl FAJCMNEPFKP {
    pub fn new() -> FAJCMNEPFKP {
        ::std::default::Default::default()
    }

    // .JBGAPLLMHGD MKCHFNEOOMM = 3;

    pub fn MKCHFNEOOMM(&self) -> &super::JBGAPLLMHGD::JBGAPLLMHGD {
        match self.HGBJCGHHIHF {
            ::std::option::Option::Some(fajcmnepfkp::HGBJCGHHIHF::MKCHFNEOOMM(ref v)) => v,
            _ => <super::JBGAPLLMHGD::JBGAPLLMHGD as ::protobuf::Message>::default_instance(),
        }
    }

    pub fn clear_MKCHFNEOOMM(&mut self) {
        self.HGBJCGHHIHF = ::std::option::Option::None;
    }

    pub fn has_MKCHFNEOOMM(&self) -> bool {
        match self.HGBJCGHHIHF {
            ::std::option::Option::Some(fajcmnepfkp::HGBJCGHHIHF::MKCHFNEOOMM(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_MKCHFNEOOMM(&mut self, v: super::JBGAPLLMHGD::JBGAPLLMHGD) {
        self.HGBJCGHHIHF = ::std::option::Option::Some(fajcmnepfkp::HGBJCGHHIHF::MKCHFNEOOMM(v))
    }

    // Mutable pointer to the field.
    pub fn mut_MKCHFNEOOMM(&mut self) -> &mut super::JBGAPLLMHGD::JBGAPLLMHGD {
        if let ::std::option::Option::Some(fajcmnepfkp::HGBJCGHHIHF::MKCHFNEOOMM(_)) = self.HGBJCGHHIHF {
        } else {
            self.HGBJCGHHIHF = ::std::option::Option::Some(fajcmnepfkp::HGBJCGHHIHF::MKCHFNEOOMM(super::JBGAPLLMHGD::JBGAPLLMHGD::new()));
        }
        match self.HGBJCGHHIHF {
            ::std::option::Option::Some(fajcmnepfkp::HGBJCGHHIHF::MKCHFNEOOMM(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_MKCHFNEOOMM(&mut self) -> super::JBGAPLLMHGD::JBGAPLLMHGD {
        if self.has_MKCHFNEOOMM() {
            match self.HGBJCGHHIHF.take() {
                ::std::option::Option::Some(fajcmnepfkp::HGBJCGHHIHF::MKCHFNEOOMM(v)) => v,
                _ => panic!(),
            }
        } else {
            super::JBGAPLLMHGD::JBGAPLLMHGD::new()
        }
    }

    // .MBOAMPMKCFH HDOJPDGNEBO = 2;

    pub fn HDOJPDGNEBO(&self) -> &super::MBOAMPMKCFH::MBOAMPMKCFH {
        match self.HGBJCGHHIHF {
            ::std::option::Option::Some(fajcmnepfkp::HGBJCGHHIHF::HDOJPDGNEBO(ref v)) => v,
            _ => <super::MBOAMPMKCFH::MBOAMPMKCFH as ::protobuf::Message>::default_instance(),
        }
    }

    pub fn clear_HDOJPDGNEBO(&mut self) {
        self.HGBJCGHHIHF = ::std::option::Option::None;
    }

    pub fn has_HDOJPDGNEBO(&self) -> bool {
        match self.HGBJCGHHIHF {
            ::std::option::Option::Some(fajcmnepfkp::HGBJCGHHIHF::HDOJPDGNEBO(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_HDOJPDGNEBO(&mut self, v: super::MBOAMPMKCFH::MBOAMPMKCFH) {
        self.HGBJCGHHIHF = ::std::option::Option::Some(fajcmnepfkp::HGBJCGHHIHF::HDOJPDGNEBO(v))
    }

    // Mutable pointer to the field.
    pub fn mut_HDOJPDGNEBO(&mut self) -> &mut super::MBOAMPMKCFH::MBOAMPMKCFH {
        if let ::std::option::Option::Some(fajcmnepfkp::HGBJCGHHIHF::HDOJPDGNEBO(_)) = self.HGBJCGHHIHF {
        } else {
            self.HGBJCGHHIHF = ::std::option::Option::Some(fajcmnepfkp::HGBJCGHHIHF::HDOJPDGNEBO(super::MBOAMPMKCFH::MBOAMPMKCFH::new()));
        }
        match self.HGBJCGHHIHF {
            ::std::option::Option::Some(fajcmnepfkp::HGBJCGHHIHF::HDOJPDGNEBO(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_HDOJPDGNEBO(&mut self) -> super::MBOAMPMKCFH::MBOAMPMKCFH {
        if self.has_HDOJPDGNEBO() {
            match self.HGBJCGHHIHF.take() {
                ::std::option::Option::Some(fajcmnepfkp::HGBJCGHHIHF::HDOJPDGNEBO(v)) => v,
                _ => panic!(),
            }
        } else {
            super::MBOAMPMKCFH::MBOAMPMKCFH::new()
        }
    }

    // .MBOAMPMKCFH OHJHHAFMGIL = 15;

    pub fn OHJHHAFMGIL(&self) -> &super::MBOAMPMKCFH::MBOAMPMKCFH {
        match self.HGBJCGHHIHF {
            ::std::option::Option::Some(fajcmnepfkp::HGBJCGHHIHF::OHJHHAFMGIL(ref v)) => v,
            _ => <super::MBOAMPMKCFH::MBOAMPMKCFH as ::protobuf::Message>::default_instance(),
        }
    }

    pub fn clear_OHJHHAFMGIL(&mut self) {
        self.HGBJCGHHIHF = ::std::option::Option::None;
    }

    pub fn has_OHJHHAFMGIL(&self) -> bool {
        match self.HGBJCGHHIHF {
            ::std::option::Option::Some(fajcmnepfkp::HGBJCGHHIHF::OHJHHAFMGIL(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_OHJHHAFMGIL(&mut self, v: super::MBOAMPMKCFH::MBOAMPMKCFH) {
        self.HGBJCGHHIHF = ::std::option::Option::Some(fajcmnepfkp::HGBJCGHHIHF::OHJHHAFMGIL(v))
    }

    // Mutable pointer to the field.
    pub fn mut_OHJHHAFMGIL(&mut self) -> &mut super::MBOAMPMKCFH::MBOAMPMKCFH {
        if let ::std::option::Option::Some(fajcmnepfkp::HGBJCGHHIHF::OHJHHAFMGIL(_)) = self.HGBJCGHHIHF {
        } else {
            self.HGBJCGHHIHF = ::std::option::Option::Some(fajcmnepfkp::HGBJCGHHIHF::OHJHHAFMGIL(super::MBOAMPMKCFH::MBOAMPMKCFH::new()));
        }
        match self.HGBJCGHHIHF {
            ::std::option::Option::Some(fajcmnepfkp::HGBJCGHHIHF::OHJHHAFMGIL(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_OHJHHAFMGIL(&mut self) -> super::MBOAMPMKCFH::MBOAMPMKCFH {
        if self.has_OHJHHAFMGIL() {
            match self.HGBJCGHHIHF.take() {
                ::std::option::Option::Some(fajcmnepfkp::HGBJCGHHIHF::OHJHHAFMGIL(v)) => v,
                _ => panic!(),
            }
        } else {
            super::MBOAMPMKCFH::MBOAMPMKCFH::new()
        }
    }

    // .OBHLEDPAPAL JFNKLMJNGLL = 5;

    pub fn JFNKLMJNGLL(&self) -> &super::OBHLEDPAPAL::OBHLEDPAPAL {
        match self.HGBJCGHHIHF {
            ::std::option::Option::Some(fajcmnepfkp::HGBJCGHHIHF::JFNKLMJNGLL(ref v)) => v,
            _ => <super::OBHLEDPAPAL::OBHLEDPAPAL as ::protobuf::Message>::default_instance(),
        }
    }

    pub fn clear_JFNKLMJNGLL(&mut self) {
        self.HGBJCGHHIHF = ::std::option::Option::None;
    }

    pub fn has_JFNKLMJNGLL(&self) -> bool {
        match self.HGBJCGHHIHF {
            ::std::option::Option::Some(fajcmnepfkp::HGBJCGHHIHF::JFNKLMJNGLL(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_JFNKLMJNGLL(&mut self, v: super::OBHLEDPAPAL::OBHLEDPAPAL) {
        self.HGBJCGHHIHF = ::std::option::Option::Some(fajcmnepfkp::HGBJCGHHIHF::JFNKLMJNGLL(v))
    }

    // Mutable pointer to the field.
    pub fn mut_JFNKLMJNGLL(&mut self) -> &mut super::OBHLEDPAPAL::OBHLEDPAPAL {
        if let ::std::option::Option::Some(fajcmnepfkp::HGBJCGHHIHF::JFNKLMJNGLL(_)) = self.HGBJCGHHIHF {
        } else {
            self.HGBJCGHHIHF = ::std::option::Option::Some(fajcmnepfkp::HGBJCGHHIHF::JFNKLMJNGLL(super::OBHLEDPAPAL::OBHLEDPAPAL::new()));
        }
        match self.HGBJCGHHIHF {
            ::std::option::Option::Some(fajcmnepfkp::HGBJCGHHIHF::JFNKLMJNGLL(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_JFNKLMJNGLL(&mut self) -> super::OBHLEDPAPAL::OBHLEDPAPAL {
        if self.has_JFNKLMJNGLL() {
            match self.HGBJCGHHIHF.take() {
                ::std::option::Option::Some(fajcmnepfkp::HGBJCGHHIHF::JFNKLMJNGLL(v)) => v,
                _ => panic!(),
            }
        } else {
            super::OBHLEDPAPAL::OBHLEDPAPAL::new()
        }
    }

    // .FCDNGMIFCNO NLBKJHGCHDJ = 1;

    pub fn NLBKJHGCHDJ(&self) -> &super::FCDNGMIFCNO::FCDNGMIFCNO {
        match self.HGBJCGHHIHF {
            ::std::option::Option::Some(fajcmnepfkp::HGBJCGHHIHF::NLBKJHGCHDJ(ref v)) => v,
            _ => <super::FCDNGMIFCNO::FCDNGMIFCNO as ::protobuf::Message>::default_instance(),
        }
    }

    pub fn clear_NLBKJHGCHDJ(&mut self) {
        self.HGBJCGHHIHF = ::std::option::Option::None;
    }

    pub fn has_NLBKJHGCHDJ(&self) -> bool {
        match self.HGBJCGHHIHF {
            ::std::option::Option::Some(fajcmnepfkp::HGBJCGHHIHF::NLBKJHGCHDJ(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_NLBKJHGCHDJ(&mut self, v: super::FCDNGMIFCNO::FCDNGMIFCNO) {
        self.HGBJCGHHIHF = ::std::option::Option::Some(fajcmnepfkp::HGBJCGHHIHF::NLBKJHGCHDJ(v))
    }

    // Mutable pointer to the field.
    pub fn mut_NLBKJHGCHDJ(&mut self) -> &mut super::FCDNGMIFCNO::FCDNGMIFCNO {
        if let ::std::option::Option::Some(fajcmnepfkp::HGBJCGHHIHF::NLBKJHGCHDJ(_)) = self.HGBJCGHHIHF {
        } else {
            self.HGBJCGHHIHF = ::std::option::Option::Some(fajcmnepfkp::HGBJCGHHIHF::NLBKJHGCHDJ(super::FCDNGMIFCNO::FCDNGMIFCNO::new()));
        }
        match self.HGBJCGHHIHF {
            ::std::option::Option::Some(fajcmnepfkp::HGBJCGHHIHF::NLBKJHGCHDJ(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_NLBKJHGCHDJ(&mut self) -> super::FCDNGMIFCNO::FCDNGMIFCNO {
        if self.has_NLBKJHGCHDJ() {
            match self.HGBJCGHHIHF.take() {
                ::std::option::Option::Some(fajcmnepfkp::HGBJCGHHIHF::NLBKJHGCHDJ(v)) => v,
                _ => panic!(),
            }
        } else {
            super::FCDNGMIFCNO::FCDNGMIFCNO::new()
        }
    }

    // .AMHGAANHDMN LODOEOEBHPK = 10;

    pub fn LODOEOEBHPK(&self) -> &super::AMHGAANHDMN::AMHGAANHDMN {
        match self.HGBJCGHHIHF {
            ::std::option::Option::Some(fajcmnepfkp::HGBJCGHHIHF::LODOEOEBHPK(ref v)) => v,
            _ => <super::AMHGAANHDMN::AMHGAANHDMN as ::protobuf::Message>::default_instance(),
        }
    }

    pub fn clear_LODOEOEBHPK(&mut self) {
        self.HGBJCGHHIHF = ::std::option::Option::None;
    }

    pub fn has_LODOEOEBHPK(&self) -> bool {
        match self.HGBJCGHHIHF {
            ::std::option::Option::Some(fajcmnepfkp::HGBJCGHHIHF::LODOEOEBHPK(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_LODOEOEBHPK(&mut self, v: super::AMHGAANHDMN::AMHGAANHDMN) {
        self.HGBJCGHHIHF = ::std::option::Option::Some(fajcmnepfkp::HGBJCGHHIHF::LODOEOEBHPK(v))
    }

    // Mutable pointer to the field.
    pub fn mut_LODOEOEBHPK(&mut self) -> &mut super::AMHGAANHDMN::AMHGAANHDMN {
        if let ::std::option::Option::Some(fajcmnepfkp::HGBJCGHHIHF::LODOEOEBHPK(_)) = self.HGBJCGHHIHF {
        } else {
            self.HGBJCGHHIHF = ::std::option::Option::Some(fajcmnepfkp::HGBJCGHHIHF::LODOEOEBHPK(super::AMHGAANHDMN::AMHGAANHDMN::new()));
        }
        match self.HGBJCGHHIHF {
            ::std::option::Option::Some(fajcmnepfkp::HGBJCGHHIHF::LODOEOEBHPK(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_LODOEOEBHPK(&mut self) -> super::AMHGAANHDMN::AMHGAANHDMN {
        if self.has_LODOEOEBHPK() {
            match self.HGBJCGHHIHF.take() {
                ::std::option::Option::Some(fajcmnepfkp::HGBJCGHHIHF::LODOEOEBHPK(v)) => v,
                _ => panic!(),
            }
        } else {
            super::AMHGAANHDMN::AMHGAANHDMN::new()
        }
    }

    // .GIPDAFLPJNN MGFCGKNCCCM = 4;

    pub fn MGFCGKNCCCM(&self) -> &super::GIPDAFLPJNN::GIPDAFLPJNN {
        match self.HGBJCGHHIHF {
            ::std::option::Option::Some(fajcmnepfkp::HGBJCGHHIHF::MGFCGKNCCCM(ref v)) => v,
            _ => <super::GIPDAFLPJNN::GIPDAFLPJNN as ::protobuf::Message>::default_instance(),
        }
    }

    pub fn clear_MGFCGKNCCCM(&mut self) {
        self.HGBJCGHHIHF = ::std::option::Option::None;
    }

    pub fn has_MGFCGKNCCCM(&self) -> bool {
        match self.HGBJCGHHIHF {
            ::std::option::Option::Some(fajcmnepfkp::HGBJCGHHIHF::MGFCGKNCCCM(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_MGFCGKNCCCM(&mut self, v: super::GIPDAFLPJNN::GIPDAFLPJNN) {
        self.HGBJCGHHIHF = ::std::option::Option::Some(fajcmnepfkp::HGBJCGHHIHF::MGFCGKNCCCM(v))
    }

    // Mutable pointer to the field.
    pub fn mut_MGFCGKNCCCM(&mut self) -> &mut super::GIPDAFLPJNN::GIPDAFLPJNN {
        if let ::std::option::Option::Some(fajcmnepfkp::HGBJCGHHIHF::MGFCGKNCCCM(_)) = self.HGBJCGHHIHF {
        } else {
            self.HGBJCGHHIHF = ::std::option::Option::Some(fajcmnepfkp::HGBJCGHHIHF::MGFCGKNCCCM(super::GIPDAFLPJNN::GIPDAFLPJNN::new()));
        }
        match self.HGBJCGHHIHF {
            ::std::option::Option::Some(fajcmnepfkp::HGBJCGHHIHF::MGFCGKNCCCM(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_MGFCGKNCCCM(&mut self) -> super::GIPDAFLPJNN::GIPDAFLPJNN {
        if self.has_MGFCGKNCCCM() {
            match self.HGBJCGHHIHF.take() {
                ::std::option::Option::Some(fajcmnepfkp::HGBJCGHHIHF::MGFCGKNCCCM(v)) => v,
                _ => panic!(),
            }
        } else {
            super::GIPDAFLPJNN::GIPDAFLPJNN::new()
        }
    }

    // uint32 LNPKBCGHFBA = 7;

    pub fn LNPKBCGHFBA(&self) -> u32 {
        match self.HGBJCGHHIHF {
            ::std::option::Option::Some(fajcmnepfkp::HGBJCGHHIHF::LNPKBCGHFBA(v)) => v,
            _ => 0,
        }
    }

    pub fn clear_LNPKBCGHFBA(&mut self) {
        self.HGBJCGHHIHF = ::std::option::Option::None;
    }

    pub fn has_LNPKBCGHFBA(&self) -> bool {
        match self.HGBJCGHHIHF {
            ::std::option::Option::Some(fajcmnepfkp::HGBJCGHHIHF::LNPKBCGHFBA(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_LNPKBCGHFBA(&mut self, v: u32) {
        self.HGBJCGHHIHF = ::std::option::Option::Some(fajcmnepfkp::HGBJCGHHIHF::LNPKBCGHFBA(v))
    }

    // uint32 GFBBLGMOCDN = 1850;

    pub fn GFBBLGMOCDN(&self) -> u32 {
        match self.HGBJCGHHIHF {
            ::std::option::Option::Some(fajcmnepfkp::HGBJCGHHIHF::GFBBLGMOCDN(v)) => v,
            _ => 0,
        }
    }

    pub fn clear_GFBBLGMOCDN(&mut self) {
        self.HGBJCGHHIHF = ::std::option::Option::None;
    }

    pub fn has_GFBBLGMOCDN(&self) -> bool {
        match self.HGBJCGHHIHF {
            ::std::option::Option::Some(fajcmnepfkp::HGBJCGHHIHF::GFBBLGMOCDN(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_GFBBLGMOCDN(&mut self, v: u32) {
        self.HGBJCGHHIHF = ::std::option::Option::Some(fajcmnepfkp::HGBJCGHHIHF::GFBBLGMOCDN(v))
    }

    // .JHHKEBFANLL FIAAEFDIDDI = 336;

    pub fn FIAAEFDIDDI(&self) -> &super::JHHKEBFANLL::JHHKEBFANLL {
        match self.HGBJCGHHIHF {
            ::std::option::Option::Some(fajcmnepfkp::HGBJCGHHIHF::FIAAEFDIDDI(ref v)) => v,
            _ => <super::JHHKEBFANLL::JHHKEBFANLL as ::protobuf::Message>::default_instance(),
        }
    }

    pub fn clear_FIAAEFDIDDI(&mut self) {
        self.HGBJCGHHIHF = ::std::option::Option::None;
    }

    pub fn has_FIAAEFDIDDI(&self) -> bool {
        match self.HGBJCGHHIHF {
            ::std::option::Option::Some(fajcmnepfkp::HGBJCGHHIHF::FIAAEFDIDDI(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_FIAAEFDIDDI(&mut self, v: super::JHHKEBFANLL::JHHKEBFANLL) {
        self.HGBJCGHHIHF = ::std::option::Option::Some(fajcmnepfkp::HGBJCGHHIHF::FIAAEFDIDDI(v))
    }

    // Mutable pointer to the field.
    pub fn mut_FIAAEFDIDDI(&mut self) -> &mut super::JHHKEBFANLL::JHHKEBFANLL {
        if let ::std::option::Option::Some(fajcmnepfkp::HGBJCGHHIHF::FIAAEFDIDDI(_)) = self.HGBJCGHHIHF {
        } else {
            self.HGBJCGHHIHF = ::std::option::Option::Some(fajcmnepfkp::HGBJCGHHIHF::FIAAEFDIDDI(super::JHHKEBFANLL::JHHKEBFANLL::new()));
        }
        match self.HGBJCGHHIHF {
            ::std::option::Option::Some(fajcmnepfkp::HGBJCGHHIHF::FIAAEFDIDDI(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_FIAAEFDIDDI(&mut self) -> super::JHHKEBFANLL::JHHKEBFANLL {
        if self.has_FIAAEFDIDDI() {
            match self.HGBJCGHHIHF.take() {
                ::std::option::Option::Some(fajcmnepfkp::HGBJCGHHIHF::FIAAEFDIDDI(v)) => v,
                _ => panic!(),
            }
        } else {
            super::JHHKEBFANLL::JHHKEBFANLL::new()
        }
    }

    // .HMIDIIBGJLJ AENAJJCDGOD = 702;

    pub fn AENAJJCDGOD(&self) -> &super::HMIDIIBGJLJ::HMIDIIBGJLJ {
        match self.HGBJCGHHIHF {
            ::std::option::Option::Some(fajcmnepfkp::HGBJCGHHIHF::AENAJJCDGOD(ref v)) => v,
            _ => <super::HMIDIIBGJLJ::HMIDIIBGJLJ as ::protobuf::Message>::default_instance(),
        }
    }

    pub fn clear_AENAJJCDGOD(&mut self) {
        self.HGBJCGHHIHF = ::std::option::Option::None;
    }

    pub fn has_AENAJJCDGOD(&self) -> bool {
        match self.HGBJCGHHIHF {
            ::std::option::Option::Some(fajcmnepfkp::HGBJCGHHIHF::AENAJJCDGOD(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_AENAJJCDGOD(&mut self, v: super::HMIDIIBGJLJ::HMIDIIBGJLJ) {
        self.HGBJCGHHIHF = ::std::option::Option::Some(fajcmnepfkp::HGBJCGHHIHF::AENAJJCDGOD(v))
    }

    // Mutable pointer to the field.
    pub fn mut_AENAJJCDGOD(&mut self) -> &mut super::HMIDIIBGJLJ::HMIDIIBGJLJ {
        if let ::std::option::Option::Some(fajcmnepfkp::HGBJCGHHIHF::AENAJJCDGOD(_)) = self.HGBJCGHHIHF {
        } else {
            self.HGBJCGHHIHF = ::std::option::Option::Some(fajcmnepfkp::HGBJCGHHIHF::AENAJJCDGOD(super::HMIDIIBGJLJ::HMIDIIBGJLJ::new()));
        }
        match self.HGBJCGHHIHF {
            ::std::option::Option::Some(fajcmnepfkp::HGBJCGHHIHF::AENAJJCDGOD(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_AENAJJCDGOD(&mut self) -> super::HMIDIIBGJLJ::HMIDIIBGJLJ {
        if self.has_AENAJJCDGOD() {
            match self.HGBJCGHHIHF.take() {
                ::std::option::Option::Some(fajcmnepfkp::HGBJCGHHIHF::AENAJJCDGOD(v)) => v,
                _ => panic!(),
            }
        } else {
            super::HMIDIIBGJLJ::HMIDIIBGJLJ::new()
        }
    }

    // uint32 POPEMHPKLJO = 1716;

    pub fn POPEMHPKLJO(&self) -> u32 {
        match self.HGBJCGHHIHF {
            ::std::option::Option::Some(fajcmnepfkp::HGBJCGHHIHF::POPEMHPKLJO(v)) => v,
            _ => 0,
        }
    }

    pub fn clear_POPEMHPKLJO(&mut self) {
        self.HGBJCGHHIHF = ::std::option::Option::None;
    }

    pub fn has_POPEMHPKLJO(&self) -> bool {
        match self.HGBJCGHHIHF {
            ::std::option::Option::Some(fajcmnepfkp::HGBJCGHHIHF::POPEMHPKLJO(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_POPEMHPKLJO(&mut self, v: u32) {
        self.HGBJCGHHIHF = ::std::option::Option::Some(fajcmnepfkp::HGBJCGHHIHF::POPEMHPKLJO(v))
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(12);
        let mut oneofs = ::std::vec::Vec::with_capacity(1);
        fields.push(::protobuf::reflect::rt::v2::make_oneof_message_has_get_mut_set_accessor::<_, super::JBGAPLLMHGD::JBGAPLLMHGD>(
            "MKCHFNEOOMM",
            FAJCMNEPFKP::has_MKCHFNEOOMM,
            FAJCMNEPFKP::MKCHFNEOOMM,
            FAJCMNEPFKP::mut_MKCHFNEOOMM,
            FAJCMNEPFKP::set_MKCHFNEOOMM,
        ));
        fields.push(::protobuf::reflect::rt::v2::make_oneof_message_has_get_mut_set_accessor::<_, super::MBOAMPMKCFH::MBOAMPMKCFH>(
            "HDOJPDGNEBO",
            FAJCMNEPFKP::has_HDOJPDGNEBO,
            FAJCMNEPFKP::HDOJPDGNEBO,
            FAJCMNEPFKP::mut_HDOJPDGNEBO,
            FAJCMNEPFKP::set_HDOJPDGNEBO,
        ));
        fields.push(::protobuf::reflect::rt::v2::make_oneof_message_has_get_mut_set_accessor::<_, super::MBOAMPMKCFH::MBOAMPMKCFH>(
            "OHJHHAFMGIL",
            FAJCMNEPFKP::has_OHJHHAFMGIL,
            FAJCMNEPFKP::OHJHHAFMGIL,
            FAJCMNEPFKP::mut_OHJHHAFMGIL,
            FAJCMNEPFKP::set_OHJHHAFMGIL,
        ));
        fields.push(::protobuf::reflect::rt::v2::make_oneof_message_has_get_mut_set_accessor::<_, super::OBHLEDPAPAL::OBHLEDPAPAL>(
            "JFNKLMJNGLL",
            FAJCMNEPFKP::has_JFNKLMJNGLL,
            FAJCMNEPFKP::JFNKLMJNGLL,
            FAJCMNEPFKP::mut_JFNKLMJNGLL,
            FAJCMNEPFKP::set_JFNKLMJNGLL,
        ));
        fields.push(::protobuf::reflect::rt::v2::make_oneof_message_has_get_mut_set_accessor::<_, super::FCDNGMIFCNO::FCDNGMIFCNO>(
            "NLBKJHGCHDJ",
            FAJCMNEPFKP::has_NLBKJHGCHDJ,
            FAJCMNEPFKP::NLBKJHGCHDJ,
            FAJCMNEPFKP::mut_NLBKJHGCHDJ,
            FAJCMNEPFKP::set_NLBKJHGCHDJ,
        ));
        fields.push(::protobuf::reflect::rt::v2::make_oneof_message_has_get_mut_set_accessor::<_, super::AMHGAANHDMN::AMHGAANHDMN>(
            "LODOEOEBHPK",
            FAJCMNEPFKP::has_LODOEOEBHPK,
            FAJCMNEPFKP::LODOEOEBHPK,
            FAJCMNEPFKP::mut_LODOEOEBHPK,
            FAJCMNEPFKP::set_LODOEOEBHPK,
        ));
        fields.push(::protobuf::reflect::rt::v2::make_oneof_message_has_get_mut_set_accessor::<_, super::GIPDAFLPJNN::GIPDAFLPJNN>(
            "MGFCGKNCCCM",
            FAJCMNEPFKP::has_MGFCGKNCCCM,
            FAJCMNEPFKP::MGFCGKNCCCM,
            FAJCMNEPFKP::mut_MGFCGKNCCCM,
            FAJCMNEPFKP::set_MGFCGKNCCCM,
        ));
        fields.push(::protobuf::reflect::rt::v2::make_oneof_copy_has_get_set_simpler_accessors::<_, _>(
            "LNPKBCGHFBA",
            FAJCMNEPFKP::has_LNPKBCGHFBA,
            FAJCMNEPFKP::LNPKBCGHFBA,
            FAJCMNEPFKP::set_LNPKBCGHFBA,
        ));
        fields.push(::protobuf::reflect::rt::v2::make_oneof_copy_has_get_set_simpler_accessors::<_, _>(
            "GFBBLGMOCDN",
            FAJCMNEPFKP::has_GFBBLGMOCDN,
            FAJCMNEPFKP::GFBBLGMOCDN,
            FAJCMNEPFKP::set_GFBBLGMOCDN,
        ));
        fields.push(::protobuf::reflect::rt::v2::make_oneof_message_has_get_mut_set_accessor::<_, super::JHHKEBFANLL::JHHKEBFANLL>(
            "FIAAEFDIDDI",
            FAJCMNEPFKP::has_FIAAEFDIDDI,
            FAJCMNEPFKP::FIAAEFDIDDI,
            FAJCMNEPFKP::mut_FIAAEFDIDDI,
            FAJCMNEPFKP::set_FIAAEFDIDDI,
        ));
        fields.push(::protobuf::reflect::rt::v2::make_oneof_message_has_get_mut_set_accessor::<_, super::HMIDIIBGJLJ::HMIDIIBGJLJ>(
            "AENAJJCDGOD",
            FAJCMNEPFKP::has_AENAJJCDGOD,
            FAJCMNEPFKP::AENAJJCDGOD,
            FAJCMNEPFKP::mut_AENAJJCDGOD,
            FAJCMNEPFKP::set_AENAJJCDGOD,
        ));
        fields.push(::protobuf::reflect::rt::v2::make_oneof_copy_has_get_set_simpler_accessors::<_, _>(
            "POPEMHPKLJO",
            FAJCMNEPFKP::has_POPEMHPKLJO,
            FAJCMNEPFKP::POPEMHPKLJO,
            FAJCMNEPFKP::set_POPEMHPKLJO,
        ));
        oneofs.push(fajcmnepfkp::HGBJCGHHIHF::generated_oneof_descriptor_data());
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<FAJCMNEPFKP>(
            "FAJCMNEPFKP",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for FAJCMNEPFKP {
    const NAME: &'static str = "FAJCMNEPFKP";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                26 => {
                    self.HGBJCGHHIHF = ::std::option::Option::Some(fajcmnepfkp::HGBJCGHHIHF::MKCHFNEOOMM(is.read_message()?));
                },
                18 => {
                    self.HGBJCGHHIHF = ::std::option::Option::Some(fajcmnepfkp::HGBJCGHHIHF::HDOJPDGNEBO(is.read_message()?));
                },
                122 => {
                    self.HGBJCGHHIHF = ::std::option::Option::Some(fajcmnepfkp::HGBJCGHHIHF::OHJHHAFMGIL(is.read_message()?));
                },
                42 => {
                    self.HGBJCGHHIHF = ::std::option::Option::Some(fajcmnepfkp::HGBJCGHHIHF::JFNKLMJNGLL(is.read_message()?));
                },
                10 => {
                    self.HGBJCGHHIHF = ::std::option::Option::Some(fajcmnepfkp::HGBJCGHHIHF::NLBKJHGCHDJ(is.read_message()?));
                },
                82 => {
                    self.HGBJCGHHIHF = ::std::option::Option::Some(fajcmnepfkp::HGBJCGHHIHF::LODOEOEBHPK(is.read_message()?));
                },
                34 => {
                    self.HGBJCGHHIHF = ::std::option::Option::Some(fajcmnepfkp::HGBJCGHHIHF::MGFCGKNCCCM(is.read_message()?));
                },
                56 => {
                    self.HGBJCGHHIHF = ::std::option::Option::Some(fajcmnepfkp::HGBJCGHHIHF::LNPKBCGHFBA(is.read_uint32()?));
                },
                14800 => {
                    self.HGBJCGHHIHF = ::std::option::Option::Some(fajcmnepfkp::HGBJCGHHIHF::GFBBLGMOCDN(is.read_uint32()?));
                },
                2690 => {
                    self.HGBJCGHHIHF = ::std::option::Option::Some(fajcmnepfkp::HGBJCGHHIHF::FIAAEFDIDDI(is.read_message()?));
                },
                5618 => {
                    self.HGBJCGHHIHF = ::std::option::Option::Some(fajcmnepfkp::HGBJCGHHIHF::AENAJJCDGOD(is.read_message()?));
                },
                13728 => {
                    self.HGBJCGHHIHF = ::std::option::Option::Some(fajcmnepfkp::HGBJCGHHIHF::POPEMHPKLJO(is.read_uint32()?));
                },
                tag => {
                    ::protobuf::rt::read_unknown_or_skip_group(tag, is, self.special_fields.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u64 {
        let mut my_size = 0;
        if let ::std::option::Option::Some(ref v) = self.HGBJCGHHIHF {
            match v {
                &fajcmnepfkp::HGBJCGHHIHF::MKCHFNEOOMM(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
                },
                &fajcmnepfkp::HGBJCGHHIHF::HDOJPDGNEBO(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
                },
                &fajcmnepfkp::HGBJCGHHIHF::OHJHHAFMGIL(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
                },
                &fajcmnepfkp::HGBJCGHHIHF::JFNKLMJNGLL(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
                },
                &fajcmnepfkp::HGBJCGHHIHF::NLBKJHGCHDJ(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
                },
                &fajcmnepfkp::HGBJCGHHIHF::LODOEOEBHPK(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
                },
                &fajcmnepfkp::HGBJCGHHIHF::MGFCGKNCCCM(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
                },
                &fajcmnepfkp::HGBJCGHHIHF::LNPKBCGHFBA(v) => {
                    my_size += ::protobuf::rt::uint32_size(7, v);
                },
                &fajcmnepfkp::HGBJCGHHIHF::GFBBLGMOCDN(v) => {
                    my_size += ::protobuf::rt::uint32_size(1850, v);
                },
                &fajcmnepfkp::HGBJCGHHIHF::FIAAEFDIDDI(ref v) => {
                    let len = v.compute_size();
                    my_size += 2 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
                },
                &fajcmnepfkp::HGBJCGHHIHF::AENAJJCDGOD(ref v) => {
                    let len = v.compute_size();
                    my_size += 2 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
                },
                &fajcmnepfkp::HGBJCGHHIHF::POPEMHPKLJO(v) => {
                    my_size += ::protobuf::rt::uint32_size(1716, v);
                },
            };
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if let ::std::option::Option::Some(ref v) = self.HGBJCGHHIHF {
            match v {
                &fajcmnepfkp::HGBJCGHHIHF::MKCHFNEOOMM(ref v) => {
                    ::protobuf::rt::write_message_field_with_cached_size(3, v, os)?;
                },
                &fajcmnepfkp::HGBJCGHHIHF::HDOJPDGNEBO(ref v) => {
                    ::protobuf::rt::write_message_field_with_cached_size(2, v, os)?;
                },
                &fajcmnepfkp::HGBJCGHHIHF::OHJHHAFMGIL(ref v) => {
                    ::protobuf::rt::write_message_field_with_cached_size(15, v, os)?;
                },
                &fajcmnepfkp::HGBJCGHHIHF::JFNKLMJNGLL(ref v) => {
                    ::protobuf::rt::write_message_field_with_cached_size(5, v, os)?;
                },
                &fajcmnepfkp::HGBJCGHHIHF::NLBKJHGCHDJ(ref v) => {
                    ::protobuf::rt::write_message_field_with_cached_size(1, v, os)?;
                },
                &fajcmnepfkp::HGBJCGHHIHF::LODOEOEBHPK(ref v) => {
                    ::protobuf::rt::write_message_field_with_cached_size(10, v, os)?;
                },
                &fajcmnepfkp::HGBJCGHHIHF::MGFCGKNCCCM(ref v) => {
                    ::protobuf::rt::write_message_field_with_cached_size(4, v, os)?;
                },
                &fajcmnepfkp::HGBJCGHHIHF::LNPKBCGHFBA(v) => {
                    os.write_uint32(7, v)?;
                },
                &fajcmnepfkp::HGBJCGHHIHF::GFBBLGMOCDN(v) => {
                    os.write_uint32(1850, v)?;
                },
                &fajcmnepfkp::HGBJCGHHIHF::FIAAEFDIDDI(ref v) => {
                    ::protobuf::rt::write_message_field_with_cached_size(336, v, os)?;
                },
                &fajcmnepfkp::HGBJCGHHIHF::AENAJJCDGOD(ref v) => {
                    ::protobuf::rt::write_message_field_with_cached_size(702, v, os)?;
                },
                &fajcmnepfkp::HGBJCGHHIHF::POPEMHPKLJO(v) => {
                    os.write_uint32(1716, v)?;
                },
            };
        }
        os.write_unknown_fields(self.special_fields.unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn special_fields(&self) -> &::protobuf::SpecialFields {
        &self.special_fields
    }

    fn mut_special_fields(&mut self) -> &mut ::protobuf::SpecialFields {
        &mut self.special_fields
    }

    fn new() -> FAJCMNEPFKP {
        FAJCMNEPFKP::new()
    }

    fn clear(&mut self) {
        self.HGBJCGHHIHF = ::std::option::Option::None;
        self.HGBJCGHHIHF = ::std::option::Option::None;
        self.HGBJCGHHIHF = ::std::option::Option::None;
        self.HGBJCGHHIHF = ::std::option::Option::None;
        self.HGBJCGHHIHF = ::std::option::Option::None;
        self.HGBJCGHHIHF = ::std::option::Option::None;
        self.HGBJCGHHIHF = ::std::option::Option::None;
        self.HGBJCGHHIHF = ::std::option::Option::None;
        self.HGBJCGHHIHF = ::std::option::Option::None;
        self.HGBJCGHHIHF = ::std::option::Option::None;
        self.HGBJCGHHIHF = ::std::option::Option::None;
        self.HGBJCGHHIHF = ::std::option::Option::None;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static FAJCMNEPFKP {
        static instance: FAJCMNEPFKP = FAJCMNEPFKP {
            HGBJCGHHIHF: ::std::option::Option::None,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for FAJCMNEPFKP {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("FAJCMNEPFKP").unwrap()).clone()
    }
}

impl ::std::fmt::Display for FAJCMNEPFKP {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for FAJCMNEPFKP {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

/// Nested message and enums of message `FAJCMNEPFKP`
pub mod fajcmnepfkp {

    #[derive(Clone,PartialEq,Debug)]
    #[non_exhaustive]
    // @@protoc_insertion_point(oneof:FAJCMNEPFKP.HGBJCGHHIHF)
    pub enum HGBJCGHHIHF {
        // @@protoc_insertion_point(oneof_field:FAJCMNEPFKP.MKCHFNEOOMM)
        MKCHFNEOOMM(super::super::JBGAPLLMHGD::JBGAPLLMHGD),
        // @@protoc_insertion_point(oneof_field:FAJCMNEPFKP.HDOJPDGNEBO)
        HDOJPDGNEBO(super::super::MBOAMPMKCFH::MBOAMPMKCFH),
        // @@protoc_insertion_point(oneof_field:FAJCMNEPFKP.OHJHHAFMGIL)
        OHJHHAFMGIL(super::super::MBOAMPMKCFH::MBOAMPMKCFH),
        // @@protoc_insertion_point(oneof_field:FAJCMNEPFKP.JFNKLMJNGLL)
        JFNKLMJNGLL(super::super::OBHLEDPAPAL::OBHLEDPAPAL),
        // @@protoc_insertion_point(oneof_field:FAJCMNEPFKP.NLBKJHGCHDJ)
        NLBKJHGCHDJ(super::super::FCDNGMIFCNO::FCDNGMIFCNO),
        // @@protoc_insertion_point(oneof_field:FAJCMNEPFKP.LODOEOEBHPK)
        LODOEOEBHPK(super::super::AMHGAANHDMN::AMHGAANHDMN),
        // @@protoc_insertion_point(oneof_field:FAJCMNEPFKP.MGFCGKNCCCM)
        MGFCGKNCCCM(super::super::GIPDAFLPJNN::GIPDAFLPJNN),
        // @@protoc_insertion_point(oneof_field:FAJCMNEPFKP.LNPKBCGHFBA)
        LNPKBCGHFBA(u32),
        // @@protoc_insertion_point(oneof_field:FAJCMNEPFKP.GFBBLGMOCDN)
        GFBBLGMOCDN(u32),
        // @@protoc_insertion_point(oneof_field:FAJCMNEPFKP.FIAAEFDIDDI)
        FIAAEFDIDDI(super::super::JHHKEBFANLL::JHHKEBFANLL),
        // @@protoc_insertion_point(oneof_field:FAJCMNEPFKP.AENAJJCDGOD)
        AENAJJCDGOD(super::super::HMIDIIBGJLJ::HMIDIIBGJLJ),
        // @@protoc_insertion_point(oneof_field:FAJCMNEPFKP.POPEMHPKLJO)
        POPEMHPKLJO(u32),
    }

    impl ::protobuf::Oneof for HGBJCGHHIHF {
    }

    impl ::protobuf::OneofFull for HGBJCGHHIHF {
        fn descriptor() -> ::protobuf::reflect::OneofDescriptor {
            static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::OneofDescriptor> = ::protobuf::rt::Lazy::new();
            descriptor.get(|| <super::FAJCMNEPFKP as ::protobuf::MessageFull>::descriptor().oneof_by_name("HGBJCGHHIHF").unwrap()).clone()
        }
    }

    impl HGBJCGHHIHF {
        pub(in super) fn generated_oneof_descriptor_data() -> ::protobuf::reflect::GeneratedOneofDescriptorData {
            ::protobuf::reflect::GeneratedOneofDescriptorData::new::<HGBJCGHHIHF>("HGBJCGHHIHF")
        }
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11FAJCMNEPFKP.proto\x1a\x11AMHGAANHDMN.proto\x1a\x11FCDNGMIFCNO.prot\
    o\x1a\x11GIPDAFLPJNN.proto\x1a\x11HMIDIIBGJLJ.proto\x1a\x11JBGAPLLMHGD.p\
    roto\x1a\x11JHHKEBFANLL.proto\x1a\x11MBOAMPMKCFH.proto\x1a\x11OBHLEDPAPA\
    L.proto\"\xce\x04\n\x0bFAJCMNEPFKP\x120\n\x0bMKCHFNEOOMM\x18\x03\x20\x01\
    (\x0b2\x0c.JBGAPLLMHGDH\0R\x0bMKCHFNEOOMM\x120\n\x0bHDOJPDGNEBO\x18\x02\
    \x20\x01(\x0b2\x0c.MBOAMPMKCFHH\0R\x0bHDOJPDGNEBO\x120\n\x0bOHJHHAFMGIL\
    \x18\x0f\x20\x01(\x0b2\x0c.MBOAMPMKCFHH\0R\x0bOHJHHAFMGIL\x120\n\x0bJFNK\
    LMJNGLL\x18\x05\x20\x01(\x0b2\x0c.OBHLEDPAPALH\0R\x0bJFNKLMJNGLL\x120\n\
    \x0bNLBKJHGCHDJ\x18\x01\x20\x01(\x0b2\x0c.FCDNGMIFCNOH\0R\x0bNLBKJHGCHDJ\
    \x120\n\x0bLODOEOEBHPK\x18\n\x20\x01(\x0b2\x0c.AMHGAANHDMNH\0R\x0bLODOEO\
    EBHPK\x120\n\x0bMGFCGKNCCCM\x18\x04\x20\x01(\x0b2\x0c.GIPDAFLPJNNH\0R\
    \x0bMGFCGKNCCCM\x12\"\n\x0bLNPKBCGHFBA\x18\x07\x20\x01(\rH\0R\x0bLNPKBCG\
    HFBA\x12#\n\x0bGFBBLGMOCDN\x18\xba\x0e\x20\x01(\rH\0R\x0bGFBBLGMOCDN\x12\
    1\n\x0bFIAAEFDIDDI\x18\xd0\x02\x20\x01(\x0b2\x0c.JHHKEBFANLLH\0R\x0bFIAA\
    EFDIDDI\x121\n\x0bAENAJJCDGOD\x18\xbe\x05\x20\x01(\x0b2\x0c.HMIDIIBGJLJH\
    \0R\x0bAENAJJCDGOD\x12#\n\x0bPOPEMHPKLJO\x18\xb4\r\x20\x01(\rH\0R\x0bPOP\
    EMHPKLJOB\r\n\x0bHGBJCGHHIHFb\x06proto3\
";

/// `FileDescriptorProto` object which was a source for this generated file
fn file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto {
    static file_descriptor_proto_lazy: ::protobuf::rt::Lazy<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::rt::Lazy::new();
    file_descriptor_proto_lazy.get(|| {
        ::protobuf::Message::parse_from_bytes(file_descriptor_proto_data).unwrap()
    })
}

/// `FileDescriptor` object which allows dynamic access to files
pub fn file_descriptor() -> &'static ::protobuf::reflect::FileDescriptor {
    static generated_file_descriptor_lazy: ::protobuf::rt::Lazy<::protobuf::reflect::GeneratedFileDescriptor> = ::protobuf::rt::Lazy::new();
    static file_descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::FileDescriptor> = ::protobuf::rt::Lazy::new();
    file_descriptor.get(|| {
        let generated_file_descriptor = generated_file_descriptor_lazy.get(|| {
            let mut deps = ::std::vec::Vec::with_capacity(8);
            deps.push(super::AMHGAANHDMN::file_descriptor().clone());
            deps.push(super::FCDNGMIFCNO::file_descriptor().clone());
            deps.push(super::GIPDAFLPJNN::file_descriptor().clone());
            deps.push(super::HMIDIIBGJLJ::file_descriptor().clone());
            deps.push(super::JBGAPLLMHGD::file_descriptor().clone());
            deps.push(super::JHHKEBFANLL::file_descriptor().clone());
            deps.push(super::MBOAMPMKCFH::file_descriptor().clone());
            deps.push(super::OBHLEDPAPAL::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(FAJCMNEPFKP::generated_message_descriptor_data());
            let mut enums = ::std::vec::Vec::with_capacity(0);
            ::protobuf::reflect::GeneratedFileDescriptor::new_generated(
                file_descriptor_proto(),
                deps,
                messages,
                enums,
            )
        });
        ::protobuf::reflect::FileDescriptor::new_generated_2(generated_file_descriptor)
    })
}
