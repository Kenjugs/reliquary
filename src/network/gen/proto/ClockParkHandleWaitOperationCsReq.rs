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

//! Generated file from `ClockParkHandleWaitOperationCsReq.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:ClockParkHandleWaitOperationCsReq)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct ClockParkHandleWaitOperationCsReq {
    // message fields
    // @@protoc_insertion_point(field:ClockParkHandleWaitOperationCsReq.LKGNGHCPJAG)
    pub LKGNGHCPJAG: u32,
    // @@protoc_insertion_point(field:ClockParkHandleWaitOperationCsReq.HIAGEINLAHL)
    pub HIAGEINLAHL: u32,
    // message oneof groups
    pub AOPHFJENLGB: ::std::option::Option<clock_park_handle_wait_operation_cs_req::AOPHFJENLGB>,
    // special fields
    // @@protoc_insertion_point(special_field:ClockParkHandleWaitOperationCsReq.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a ClockParkHandleWaitOperationCsReq {
    fn default() -> &'a ClockParkHandleWaitOperationCsReq {
        <ClockParkHandleWaitOperationCsReq as ::protobuf::Message>::default_instance()
    }
}

impl ClockParkHandleWaitOperationCsReq {
    pub fn new() -> ClockParkHandleWaitOperationCsReq {
        ::std::default::Default::default()
    }

    // .LMCBNBOMNKK DBHAFLBPODA = 13;

    pub fn DBHAFLBPODA(&self) -> &super::LMCBNBOMNKK::LMCBNBOMNKK {
        match self.AOPHFJENLGB {
            ::std::option::Option::Some(clock_park_handle_wait_operation_cs_req::AOPHFJENLGB::DBHAFLBPODA(ref v)) => v,
            _ => <super::LMCBNBOMNKK::LMCBNBOMNKK as ::protobuf::Message>::default_instance(),
        }
    }

    pub fn clear_DBHAFLBPODA(&mut self) {
        self.AOPHFJENLGB = ::std::option::Option::None;
    }

    pub fn has_DBHAFLBPODA(&self) -> bool {
        match self.AOPHFJENLGB {
            ::std::option::Option::Some(clock_park_handle_wait_operation_cs_req::AOPHFJENLGB::DBHAFLBPODA(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_DBHAFLBPODA(&mut self, v: super::LMCBNBOMNKK::LMCBNBOMNKK) {
        self.AOPHFJENLGB = ::std::option::Option::Some(clock_park_handle_wait_operation_cs_req::AOPHFJENLGB::DBHAFLBPODA(v))
    }

    // Mutable pointer to the field.
    pub fn mut_DBHAFLBPODA(&mut self) -> &mut super::LMCBNBOMNKK::LMCBNBOMNKK {
        if let ::std::option::Option::Some(clock_park_handle_wait_operation_cs_req::AOPHFJENLGB::DBHAFLBPODA(_)) = self.AOPHFJENLGB {
        } else {
            self.AOPHFJENLGB = ::std::option::Option::Some(clock_park_handle_wait_operation_cs_req::AOPHFJENLGB::DBHAFLBPODA(super::LMCBNBOMNKK::LMCBNBOMNKK::new()));
        }
        match self.AOPHFJENLGB {
            ::std::option::Option::Some(clock_park_handle_wait_operation_cs_req::AOPHFJENLGB::DBHAFLBPODA(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_DBHAFLBPODA(&mut self) -> super::LMCBNBOMNKK::LMCBNBOMNKK {
        if self.has_DBHAFLBPODA() {
            match self.AOPHFJENLGB.take() {
                ::std::option::Option::Some(clock_park_handle_wait_operation_cs_req::AOPHFJENLGB::DBHAFLBPODA(v)) => v,
                _ => panic!(),
            }
        } else {
            super::LMCBNBOMNKK::LMCBNBOMNKK::new()
        }
    }

    // .CAHPBPEEPGB EMJEDJFEFHJ = 9;

    pub fn EMJEDJFEFHJ(&self) -> &super::CAHPBPEEPGB::CAHPBPEEPGB {
        match self.AOPHFJENLGB {
            ::std::option::Option::Some(clock_park_handle_wait_operation_cs_req::AOPHFJENLGB::EMJEDJFEFHJ(ref v)) => v,
            _ => <super::CAHPBPEEPGB::CAHPBPEEPGB as ::protobuf::Message>::default_instance(),
        }
    }

    pub fn clear_EMJEDJFEFHJ(&mut self) {
        self.AOPHFJENLGB = ::std::option::Option::None;
    }

    pub fn has_EMJEDJFEFHJ(&self) -> bool {
        match self.AOPHFJENLGB {
            ::std::option::Option::Some(clock_park_handle_wait_operation_cs_req::AOPHFJENLGB::EMJEDJFEFHJ(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_EMJEDJFEFHJ(&mut self, v: super::CAHPBPEEPGB::CAHPBPEEPGB) {
        self.AOPHFJENLGB = ::std::option::Option::Some(clock_park_handle_wait_operation_cs_req::AOPHFJENLGB::EMJEDJFEFHJ(v))
    }

    // Mutable pointer to the field.
    pub fn mut_EMJEDJFEFHJ(&mut self) -> &mut super::CAHPBPEEPGB::CAHPBPEEPGB {
        if let ::std::option::Option::Some(clock_park_handle_wait_operation_cs_req::AOPHFJENLGB::EMJEDJFEFHJ(_)) = self.AOPHFJENLGB {
        } else {
            self.AOPHFJENLGB = ::std::option::Option::Some(clock_park_handle_wait_operation_cs_req::AOPHFJENLGB::EMJEDJFEFHJ(super::CAHPBPEEPGB::CAHPBPEEPGB::new()));
        }
        match self.AOPHFJENLGB {
            ::std::option::Option::Some(clock_park_handle_wait_operation_cs_req::AOPHFJENLGB::EMJEDJFEFHJ(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_EMJEDJFEFHJ(&mut self) -> super::CAHPBPEEPGB::CAHPBPEEPGB {
        if self.has_EMJEDJFEFHJ() {
            match self.AOPHFJENLGB.take() {
                ::std::option::Option::Some(clock_park_handle_wait_operation_cs_req::AOPHFJENLGB::EMJEDJFEFHJ(v)) => v,
                _ => panic!(),
            }
        } else {
            super::CAHPBPEEPGB::CAHPBPEEPGB::new()
        }
    }

    // .AJBFJOEIMDM ONMPCHLDDGA = 10;

    pub fn ONMPCHLDDGA(&self) -> &super::AJBFJOEIMDM::AJBFJOEIMDM {
        match self.AOPHFJENLGB {
            ::std::option::Option::Some(clock_park_handle_wait_operation_cs_req::AOPHFJENLGB::ONMPCHLDDGA(ref v)) => v,
            _ => <super::AJBFJOEIMDM::AJBFJOEIMDM as ::protobuf::Message>::default_instance(),
        }
    }

    pub fn clear_ONMPCHLDDGA(&mut self) {
        self.AOPHFJENLGB = ::std::option::Option::None;
    }

    pub fn has_ONMPCHLDDGA(&self) -> bool {
        match self.AOPHFJENLGB {
            ::std::option::Option::Some(clock_park_handle_wait_operation_cs_req::AOPHFJENLGB::ONMPCHLDDGA(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_ONMPCHLDDGA(&mut self, v: super::AJBFJOEIMDM::AJBFJOEIMDM) {
        self.AOPHFJENLGB = ::std::option::Option::Some(clock_park_handle_wait_operation_cs_req::AOPHFJENLGB::ONMPCHLDDGA(v))
    }

    // Mutable pointer to the field.
    pub fn mut_ONMPCHLDDGA(&mut self) -> &mut super::AJBFJOEIMDM::AJBFJOEIMDM {
        if let ::std::option::Option::Some(clock_park_handle_wait_operation_cs_req::AOPHFJENLGB::ONMPCHLDDGA(_)) = self.AOPHFJENLGB {
        } else {
            self.AOPHFJENLGB = ::std::option::Option::Some(clock_park_handle_wait_operation_cs_req::AOPHFJENLGB::ONMPCHLDDGA(super::AJBFJOEIMDM::AJBFJOEIMDM::new()));
        }
        match self.AOPHFJENLGB {
            ::std::option::Option::Some(clock_park_handle_wait_operation_cs_req::AOPHFJENLGB::ONMPCHLDDGA(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_ONMPCHLDDGA(&mut self) -> super::AJBFJOEIMDM::AJBFJOEIMDM {
        if self.has_ONMPCHLDDGA() {
            match self.AOPHFJENLGB.take() {
                ::std::option::Option::Some(clock_park_handle_wait_operation_cs_req::AOPHFJENLGB::ONMPCHLDDGA(v)) => v,
                _ => panic!(),
            }
        } else {
            super::AJBFJOEIMDM::AJBFJOEIMDM::new()
        }
    }

    // .ACJICCNKCLK ENCLLODMEJH = 5;

    pub fn ENCLLODMEJH(&self) -> &super::ACJICCNKCLK::ACJICCNKCLK {
        match self.AOPHFJENLGB {
            ::std::option::Option::Some(clock_park_handle_wait_operation_cs_req::AOPHFJENLGB::ENCLLODMEJH(ref v)) => v,
            _ => <super::ACJICCNKCLK::ACJICCNKCLK as ::protobuf::Message>::default_instance(),
        }
    }

    pub fn clear_ENCLLODMEJH(&mut self) {
        self.AOPHFJENLGB = ::std::option::Option::None;
    }

    pub fn has_ENCLLODMEJH(&self) -> bool {
        match self.AOPHFJENLGB {
            ::std::option::Option::Some(clock_park_handle_wait_operation_cs_req::AOPHFJENLGB::ENCLLODMEJH(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_ENCLLODMEJH(&mut self, v: super::ACJICCNKCLK::ACJICCNKCLK) {
        self.AOPHFJENLGB = ::std::option::Option::Some(clock_park_handle_wait_operation_cs_req::AOPHFJENLGB::ENCLLODMEJH(v))
    }

    // Mutable pointer to the field.
    pub fn mut_ENCLLODMEJH(&mut self) -> &mut super::ACJICCNKCLK::ACJICCNKCLK {
        if let ::std::option::Option::Some(clock_park_handle_wait_operation_cs_req::AOPHFJENLGB::ENCLLODMEJH(_)) = self.AOPHFJENLGB {
        } else {
            self.AOPHFJENLGB = ::std::option::Option::Some(clock_park_handle_wait_operation_cs_req::AOPHFJENLGB::ENCLLODMEJH(super::ACJICCNKCLK::ACJICCNKCLK::new()));
        }
        match self.AOPHFJENLGB {
            ::std::option::Option::Some(clock_park_handle_wait_operation_cs_req::AOPHFJENLGB::ENCLLODMEJH(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_ENCLLODMEJH(&mut self) -> super::ACJICCNKCLK::ACJICCNKCLK {
        if self.has_ENCLLODMEJH() {
            match self.AOPHFJENLGB.take() {
                ::std::option::Option::Some(clock_park_handle_wait_operation_cs_req::AOPHFJENLGB::ENCLLODMEJH(v)) => v,
                _ => panic!(),
            }
        } else {
            super::ACJICCNKCLK::ACJICCNKCLK::new()
        }
    }

    // .MBHPHEIJPMG MHEGJKOFOLB = 14;

    pub fn MHEGJKOFOLB(&self) -> &super::MBHPHEIJPMG::MBHPHEIJPMG {
        match self.AOPHFJENLGB {
            ::std::option::Option::Some(clock_park_handle_wait_operation_cs_req::AOPHFJENLGB::MHEGJKOFOLB(ref v)) => v,
            _ => <super::MBHPHEIJPMG::MBHPHEIJPMG as ::protobuf::Message>::default_instance(),
        }
    }

    pub fn clear_MHEGJKOFOLB(&mut self) {
        self.AOPHFJENLGB = ::std::option::Option::None;
    }

    pub fn has_MHEGJKOFOLB(&self) -> bool {
        match self.AOPHFJENLGB {
            ::std::option::Option::Some(clock_park_handle_wait_operation_cs_req::AOPHFJENLGB::MHEGJKOFOLB(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_MHEGJKOFOLB(&mut self, v: super::MBHPHEIJPMG::MBHPHEIJPMG) {
        self.AOPHFJENLGB = ::std::option::Option::Some(clock_park_handle_wait_operation_cs_req::AOPHFJENLGB::MHEGJKOFOLB(v))
    }

    // Mutable pointer to the field.
    pub fn mut_MHEGJKOFOLB(&mut self) -> &mut super::MBHPHEIJPMG::MBHPHEIJPMG {
        if let ::std::option::Option::Some(clock_park_handle_wait_operation_cs_req::AOPHFJENLGB::MHEGJKOFOLB(_)) = self.AOPHFJENLGB {
        } else {
            self.AOPHFJENLGB = ::std::option::Option::Some(clock_park_handle_wait_operation_cs_req::AOPHFJENLGB::MHEGJKOFOLB(super::MBHPHEIJPMG::MBHPHEIJPMG::new()));
        }
        match self.AOPHFJENLGB {
            ::std::option::Option::Some(clock_park_handle_wait_operation_cs_req::AOPHFJENLGB::MHEGJKOFOLB(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_MHEGJKOFOLB(&mut self) -> super::MBHPHEIJPMG::MBHPHEIJPMG {
        if self.has_MHEGJKOFOLB() {
            match self.AOPHFJENLGB.take() {
                ::std::option::Option::Some(clock_park_handle_wait_operation_cs_req::AOPHFJENLGB::MHEGJKOFOLB(v)) => v,
                _ => panic!(),
            }
        } else {
            super::MBHPHEIJPMG::MBHPHEIJPMG::new()
        }
    }

    // .KFJJAGAICDE KEKMKBFHEED = 3;

    pub fn KEKMKBFHEED(&self) -> &super::KFJJAGAICDE::KFJJAGAICDE {
        match self.AOPHFJENLGB {
            ::std::option::Option::Some(clock_park_handle_wait_operation_cs_req::AOPHFJENLGB::KEKMKBFHEED(ref v)) => v,
            _ => <super::KFJJAGAICDE::KFJJAGAICDE as ::protobuf::Message>::default_instance(),
        }
    }

    pub fn clear_KEKMKBFHEED(&mut self) {
        self.AOPHFJENLGB = ::std::option::Option::None;
    }

    pub fn has_KEKMKBFHEED(&self) -> bool {
        match self.AOPHFJENLGB {
            ::std::option::Option::Some(clock_park_handle_wait_operation_cs_req::AOPHFJENLGB::KEKMKBFHEED(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_KEKMKBFHEED(&mut self, v: super::KFJJAGAICDE::KFJJAGAICDE) {
        self.AOPHFJENLGB = ::std::option::Option::Some(clock_park_handle_wait_operation_cs_req::AOPHFJENLGB::KEKMKBFHEED(v))
    }

    // Mutable pointer to the field.
    pub fn mut_KEKMKBFHEED(&mut self) -> &mut super::KFJJAGAICDE::KFJJAGAICDE {
        if let ::std::option::Option::Some(clock_park_handle_wait_operation_cs_req::AOPHFJENLGB::KEKMKBFHEED(_)) = self.AOPHFJENLGB {
        } else {
            self.AOPHFJENLGB = ::std::option::Option::Some(clock_park_handle_wait_operation_cs_req::AOPHFJENLGB::KEKMKBFHEED(super::KFJJAGAICDE::KFJJAGAICDE::new()));
        }
        match self.AOPHFJENLGB {
            ::std::option::Option::Some(clock_park_handle_wait_operation_cs_req::AOPHFJENLGB::KEKMKBFHEED(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_KEKMKBFHEED(&mut self) -> super::KFJJAGAICDE::KFJJAGAICDE {
        if self.has_KEKMKBFHEED() {
            match self.AOPHFJENLGB.take() {
                ::std::option::Option::Some(clock_park_handle_wait_operation_cs_req::AOPHFJENLGB::KEKMKBFHEED(v)) => v,
                _ => panic!(),
            }
        } else {
            super::KFJJAGAICDE::KFJJAGAICDE::new()
        }
    }

    // .DBIACBDCPHE FHBOCHDINAB = 4;

    pub fn FHBOCHDINAB(&self) -> &super::DBIACBDCPHE::DBIACBDCPHE {
        match self.AOPHFJENLGB {
            ::std::option::Option::Some(clock_park_handle_wait_operation_cs_req::AOPHFJENLGB::FHBOCHDINAB(ref v)) => v,
            _ => <super::DBIACBDCPHE::DBIACBDCPHE as ::protobuf::Message>::default_instance(),
        }
    }

    pub fn clear_FHBOCHDINAB(&mut self) {
        self.AOPHFJENLGB = ::std::option::Option::None;
    }

    pub fn has_FHBOCHDINAB(&self) -> bool {
        match self.AOPHFJENLGB {
            ::std::option::Option::Some(clock_park_handle_wait_operation_cs_req::AOPHFJENLGB::FHBOCHDINAB(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_FHBOCHDINAB(&mut self, v: super::DBIACBDCPHE::DBIACBDCPHE) {
        self.AOPHFJENLGB = ::std::option::Option::Some(clock_park_handle_wait_operation_cs_req::AOPHFJENLGB::FHBOCHDINAB(v))
    }

    // Mutable pointer to the field.
    pub fn mut_FHBOCHDINAB(&mut self) -> &mut super::DBIACBDCPHE::DBIACBDCPHE {
        if let ::std::option::Option::Some(clock_park_handle_wait_operation_cs_req::AOPHFJENLGB::FHBOCHDINAB(_)) = self.AOPHFJENLGB {
        } else {
            self.AOPHFJENLGB = ::std::option::Option::Some(clock_park_handle_wait_operation_cs_req::AOPHFJENLGB::FHBOCHDINAB(super::DBIACBDCPHE::DBIACBDCPHE::new()));
        }
        match self.AOPHFJENLGB {
            ::std::option::Option::Some(clock_park_handle_wait_operation_cs_req::AOPHFJENLGB::FHBOCHDINAB(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_FHBOCHDINAB(&mut self) -> super::DBIACBDCPHE::DBIACBDCPHE {
        if self.has_FHBOCHDINAB() {
            match self.AOPHFJENLGB.take() {
                ::std::option::Option::Some(clock_park_handle_wait_operation_cs_req::AOPHFJENLGB::FHBOCHDINAB(v)) => v,
                _ => panic!(),
            }
        } else {
            super::DBIACBDCPHE::DBIACBDCPHE::new()
        }
    }

    // uint32 FMPDFFAGKFO = 2;

    pub fn FMPDFFAGKFO(&self) -> u32 {
        match self.AOPHFJENLGB {
            ::std::option::Option::Some(clock_park_handle_wait_operation_cs_req::AOPHFJENLGB::FMPDFFAGKFO(v)) => v,
            _ => 0,
        }
    }

    pub fn clear_FMPDFFAGKFO(&mut self) {
        self.AOPHFJENLGB = ::std::option::Option::None;
    }

    pub fn has_FMPDFFAGKFO(&self) -> bool {
        match self.AOPHFJENLGB {
            ::std::option::Option::Some(clock_park_handle_wait_operation_cs_req::AOPHFJENLGB::FMPDFFAGKFO(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_FMPDFFAGKFO(&mut self, v: u32) {
        self.AOPHFJENLGB = ::std::option::Option::Some(clock_park_handle_wait_operation_cs_req::AOPHFJENLGB::FMPDFFAGKFO(v))
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(10);
        let mut oneofs = ::std::vec::Vec::with_capacity(1);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "LKGNGHCPJAG",
            |m: &ClockParkHandleWaitOperationCsReq| { &m.LKGNGHCPJAG },
            |m: &mut ClockParkHandleWaitOperationCsReq| { &mut m.LKGNGHCPJAG },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "HIAGEINLAHL",
            |m: &ClockParkHandleWaitOperationCsReq| { &m.HIAGEINLAHL },
            |m: &mut ClockParkHandleWaitOperationCsReq| { &mut m.HIAGEINLAHL },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_oneof_message_has_get_mut_set_accessor::<_, super::LMCBNBOMNKK::LMCBNBOMNKK>(
            "DBHAFLBPODA",
            ClockParkHandleWaitOperationCsReq::has_DBHAFLBPODA,
            ClockParkHandleWaitOperationCsReq::DBHAFLBPODA,
            ClockParkHandleWaitOperationCsReq::mut_DBHAFLBPODA,
            ClockParkHandleWaitOperationCsReq::set_DBHAFLBPODA,
        ));
        fields.push(::protobuf::reflect::rt::v2::make_oneof_message_has_get_mut_set_accessor::<_, super::CAHPBPEEPGB::CAHPBPEEPGB>(
            "EMJEDJFEFHJ",
            ClockParkHandleWaitOperationCsReq::has_EMJEDJFEFHJ,
            ClockParkHandleWaitOperationCsReq::EMJEDJFEFHJ,
            ClockParkHandleWaitOperationCsReq::mut_EMJEDJFEFHJ,
            ClockParkHandleWaitOperationCsReq::set_EMJEDJFEFHJ,
        ));
        fields.push(::protobuf::reflect::rt::v2::make_oneof_message_has_get_mut_set_accessor::<_, super::AJBFJOEIMDM::AJBFJOEIMDM>(
            "ONMPCHLDDGA",
            ClockParkHandleWaitOperationCsReq::has_ONMPCHLDDGA,
            ClockParkHandleWaitOperationCsReq::ONMPCHLDDGA,
            ClockParkHandleWaitOperationCsReq::mut_ONMPCHLDDGA,
            ClockParkHandleWaitOperationCsReq::set_ONMPCHLDDGA,
        ));
        fields.push(::protobuf::reflect::rt::v2::make_oneof_message_has_get_mut_set_accessor::<_, super::ACJICCNKCLK::ACJICCNKCLK>(
            "ENCLLODMEJH",
            ClockParkHandleWaitOperationCsReq::has_ENCLLODMEJH,
            ClockParkHandleWaitOperationCsReq::ENCLLODMEJH,
            ClockParkHandleWaitOperationCsReq::mut_ENCLLODMEJH,
            ClockParkHandleWaitOperationCsReq::set_ENCLLODMEJH,
        ));
        fields.push(::protobuf::reflect::rt::v2::make_oneof_message_has_get_mut_set_accessor::<_, super::MBHPHEIJPMG::MBHPHEIJPMG>(
            "MHEGJKOFOLB",
            ClockParkHandleWaitOperationCsReq::has_MHEGJKOFOLB,
            ClockParkHandleWaitOperationCsReq::MHEGJKOFOLB,
            ClockParkHandleWaitOperationCsReq::mut_MHEGJKOFOLB,
            ClockParkHandleWaitOperationCsReq::set_MHEGJKOFOLB,
        ));
        fields.push(::protobuf::reflect::rt::v2::make_oneof_message_has_get_mut_set_accessor::<_, super::KFJJAGAICDE::KFJJAGAICDE>(
            "KEKMKBFHEED",
            ClockParkHandleWaitOperationCsReq::has_KEKMKBFHEED,
            ClockParkHandleWaitOperationCsReq::KEKMKBFHEED,
            ClockParkHandleWaitOperationCsReq::mut_KEKMKBFHEED,
            ClockParkHandleWaitOperationCsReq::set_KEKMKBFHEED,
        ));
        fields.push(::protobuf::reflect::rt::v2::make_oneof_message_has_get_mut_set_accessor::<_, super::DBIACBDCPHE::DBIACBDCPHE>(
            "FHBOCHDINAB",
            ClockParkHandleWaitOperationCsReq::has_FHBOCHDINAB,
            ClockParkHandleWaitOperationCsReq::FHBOCHDINAB,
            ClockParkHandleWaitOperationCsReq::mut_FHBOCHDINAB,
            ClockParkHandleWaitOperationCsReq::set_FHBOCHDINAB,
        ));
        fields.push(::protobuf::reflect::rt::v2::make_oneof_copy_has_get_set_simpler_accessors::<_, _>(
            "FMPDFFAGKFO",
            ClockParkHandleWaitOperationCsReq::has_FMPDFFAGKFO,
            ClockParkHandleWaitOperationCsReq::FMPDFFAGKFO,
            ClockParkHandleWaitOperationCsReq::set_FMPDFFAGKFO,
        ));
        oneofs.push(clock_park_handle_wait_operation_cs_req::AOPHFJENLGB::generated_oneof_descriptor_data());
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<ClockParkHandleWaitOperationCsReq>(
            "ClockParkHandleWaitOperationCsReq",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for ClockParkHandleWaitOperationCsReq {
    const NAME: &'static str = "ClockParkHandleWaitOperationCsReq";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                64 => {
                    self.LKGNGHCPJAG = is.read_uint32()?;
                },
                96 => {
                    self.HIAGEINLAHL = is.read_uint32()?;
                },
                106 => {
                    self.AOPHFJENLGB = ::std::option::Option::Some(clock_park_handle_wait_operation_cs_req::AOPHFJENLGB::DBHAFLBPODA(is.read_message()?));
                },
                74 => {
                    self.AOPHFJENLGB = ::std::option::Option::Some(clock_park_handle_wait_operation_cs_req::AOPHFJENLGB::EMJEDJFEFHJ(is.read_message()?));
                },
                82 => {
                    self.AOPHFJENLGB = ::std::option::Option::Some(clock_park_handle_wait_operation_cs_req::AOPHFJENLGB::ONMPCHLDDGA(is.read_message()?));
                },
                42 => {
                    self.AOPHFJENLGB = ::std::option::Option::Some(clock_park_handle_wait_operation_cs_req::AOPHFJENLGB::ENCLLODMEJH(is.read_message()?));
                },
                114 => {
                    self.AOPHFJENLGB = ::std::option::Option::Some(clock_park_handle_wait_operation_cs_req::AOPHFJENLGB::MHEGJKOFOLB(is.read_message()?));
                },
                26 => {
                    self.AOPHFJENLGB = ::std::option::Option::Some(clock_park_handle_wait_operation_cs_req::AOPHFJENLGB::KEKMKBFHEED(is.read_message()?));
                },
                34 => {
                    self.AOPHFJENLGB = ::std::option::Option::Some(clock_park_handle_wait_operation_cs_req::AOPHFJENLGB::FHBOCHDINAB(is.read_message()?));
                },
                16 => {
                    self.AOPHFJENLGB = ::std::option::Option::Some(clock_park_handle_wait_operation_cs_req::AOPHFJENLGB::FMPDFFAGKFO(is.read_uint32()?));
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
        if self.LKGNGHCPJAG != 0 {
            my_size += ::protobuf::rt::uint32_size(8, self.LKGNGHCPJAG);
        }
        if self.HIAGEINLAHL != 0 {
            my_size += ::protobuf::rt::uint32_size(12, self.HIAGEINLAHL);
        }
        if let ::std::option::Option::Some(ref v) = self.AOPHFJENLGB {
            match v {
                &clock_park_handle_wait_operation_cs_req::AOPHFJENLGB::DBHAFLBPODA(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
                },
                &clock_park_handle_wait_operation_cs_req::AOPHFJENLGB::EMJEDJFEFHJ(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
                },
                &clock_park_handle_wait_operation_cs_req::AOPHFJENLGB::ONMPCHLDDGA(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
                },
                &clock_park_handle_wait_operation_cs_req::AOPHFJENLGB::ENCLLODMEJH(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
                },
                &clock_park_handle_wait_operation_cs_req::AOPHFJENLGB::MHEGJKOFOLB(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
                },
                &clock_park_handle_wait_operation_cs_req::AOPHFJENLGB::KEKMKBFHEED(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
                },
                &clock_park_handle_wait_operation_cs_req::AOPHFJENLGB::FHBOCHDINAB(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
                },
                &clock_park_handle_wait_operation_cs_req::AOPHFJENLGB::FMPDFFAGKFO(v) => {
                    my_size += ::protobuf::rt::uint32_size(2, v);
                },
            };
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.LKGNGHCPJAG != 0 {
            os.write_uint32(8, self.LKGNGHCPJAG)?;
        }
        if self.HIAGEINLAHL != 0 {
            os.write_uint32(12, self.HIAGEINLAHL)?;
        }
        if let ::std::option::Option::Some(ref v) = self.AOPHFJENLGB {
            match v {
                &clock_park_handle_wait_operation_cs_req::AOPHFJENLGB::DBHAFLBPODA(ref v) => {
                    ::protobuf::rt::write_message_field_with_cached_size(13, v, os)?;
                },
                &clock_park_handle_wait_operation_cs_req::AOPHFJENLGB::EMJEDJFEFHJ(ref v) => {
                    ::protobuf::rt::write_message_field_with_cached_size(9, v, os)?;
                },
                &clock_park_handle_wait_operation_cs_req::AOPHFJENLGB::ONMPCHLDDGA(ref v) => {
                    ::protobuf::rt::write_message_field_with_cached_size(10, v, os)?;
                },
                &clock_park_handle_wait_operation_cs_req::AOPHFJENLGB::ENCLLODMEJH(ref v) => {
                    ::protobuf::rt::write_message_field_with_cached_size(5, v, os)?;
                },
                &clock_park_handle_wait_operation_cs_req::AOPHFJENLGB::MHEGJKOFOLB(ref v) => {
                    ::protobuf::rt::write_message_field_with_cached_size(14, v, os)?;
                },
                &clock_park_handle_wait_operation_cs_req::AOPHFJENLGB::KEKMKBFHEED(ref v) => {
                    ::protobuf::rt::write_message_field_with_cached_size(3, v, os)?;
                },
                &clock_park_handle_wait_operation_cs_req::AOPHFJENLGB::FHBOCHDINAB(ref v) => {
                    ::protobuf::rt::write_message_field_with_cached_size(4, v, os)?;
                },
                &clock_park_handle_wait_operation_cs_req::AOPHFJENLGB::FMPDFFAGKFO(v) => {
                    os.write_uint32(2, v)?;
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

    fn new() -> ClockParkHandleWaitOperationCsReq {
        ClockParkHandleWaitOperationCsReq::new()
    }

    fn clear(&mut self) {
        self.LKGNGHCPJAG = 0;
        self.HIAGEINLAHL = 0;
        self.AOPHFJENLGB = ::std::option::Option::None;
        self.AOPHFJENLGB = ::std::option::Option::None;
        self.AOPHFJENLGB = ::std::option::Option::None;
        self.AOPHFJENLGB = ::std::option::Option::None;
        self.AOPHFJENLGB = ::std::option::Option::None;
        self.AOPHFJENLGB = ::std::option::Option::None;
        self.AOPHFJENLGB = ::std::option::Option::None;
        self.AOPHFJENLGB = ::std::option::Option::None;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static ClockParkHandleWaitOperationCsReq {
        static instance: ClockParkHandleWaitOperationCsReq = ClockParkHandleWaitOperationCsReq {
            LKGNGHCPJAG: 0,
            HIAGEINLAHL: 0,
            AOPHFJENLGB: ::std::option::Option::None,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for ClockParkHandleWaitOperationCsReq {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("ClockParkHandleWaitOperationCsReq").unwrap()).clone()
    }
}

impl ::std::fmt::Display for ClockParkHandleWaitOperationCsReq {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ClockParkHandleWaitOperationCsReq {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

/// Nested message and enums of message `ClockParkHandleWaitOperationCsReq`
pub mod clock_park_handle_wait_operation_cs_req {

    #[derive(Clone,PartialEq,Debug)]
    #[non_exhaustive]
    // @@protoc_insertion_point(oneof:ClockParkHandleWaitOperationCsReq.AOPHFJENLGB)
    pub enum AOPHFJENLGB {
        // @@protoc_insertion_point(oneof_field:ClockParkHandleWaitOperationCsReq.DBHAFLBPODA)
        DBHAFLBPODA(super::super::LMCBNBOMNKK::LMCBNBOMNKK),
        // @@protoc_insertion_point(oneof_field:ClockParkHandleWaitOperationCsReq.EMJEDJFEFHJ)
        EMJEDJFEFHJ(super::super::CAHPBPEEPGB::CAHPBPEEPGB),
        // @@protoc_insertion_point(oneof_field:ClockParkHandleWaitOperationCsReq.ONMPCHLDDGA)
        ONMPCHLDDGA(super::super::AJBFJOEIMDM::AJBFJOEIMDM),
        // @@protoc_insertion_point(oneof_field:ClockParkHandleWaitOperationCsReq.ENCLLODMEJH)
        ENCLLODMEJH(super::super::ACJICCNKCLK::ACJICCNKCLK),
        // @@protoc_insertion_point(oneof_field:ClockParkHandleWaitOperationCsReq.MHEGJKOFOLB)
        MHEGJKOFOLB(super::super::MBHPHEIJPMG::MBHPHEIJPMG),
        // @@protoc_insertion_point(oneof_field:ClockParkHandleWaitOperationCsReq.KEKMKBFHEED)
        KEKMKBFHEED(super::super::KFJJAGAICDE::KFJJAGAICDE),
        // @@protoc_insertion_point(oneof_field:ClockParkHandleWaitOperationCsReq.FHBOCHDINAB)
        FHBOCHDINAB(super::super::DBIACBDCPHE::DBIACBDCPHE),
        // @@protoc_insertion_point(oneof_field:ClockParkHandleWaitOperationCsReq.FMPDFFAGKFO)
        FMPDFFAGKFO(u32),
    }

    impl ::protobuf::Oneof for AOPHFJENLGB {
    }

    impl ::protobuf::OneofFull for AOPHFJENLGB {
        fn descriptor() -> ::protobuf::reflect::OneofDescriptor {
            static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::OneofDescriptor> = ::protobuf::rt::Lazy::new();
            descriptor.get(|| <super::ClockParkHandleWaitOperationCsReq as ::protobuf::MessageFull>::descriptor().oneof_by_name("AOPHFJENLGB").unwrap()).clone()
        }
    }

    impl AOPHFJENLGB {
        pub(in super) fn generated_oneof_descriptor_data() -> ::protobuf::reflect::GeneratedOneofDescriptorData {
            ::protobuf::reflect::GeneratedOneofDescriptorData::new::<AOPHFJENLGB>("AOPHFJENLGB")
        }
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n'ClockParkHandleWaitOperationCsReq.proto\x1a\x11ACJICCNKCLK.proto\x1a\
    \x11AJBFJOEIMDM.proto\x1a\x11CAHPBPEEPGB.proto\x1a\x11DBIACBDCPHE.proto\
    \x1a\x11KFJJAGAICDE.proto\x1a\x11LMCBNBOMNKK.proto\x1a\x11MBHPHEIJPMG.pr\
    oto\"\xf8\x03\n!ClockParkHandleWaitOperationCsReq\x12\x20\n\x0bLKGNGHCPJ\
    AG\x18\x08\x20\x01(\rR\x0bLKGNGHCPJAG\x12\x20\n\x0bHIAGEINLAHL\x18\x0c\
    \x20\x01(\rR\x0bHIAGEINLAHL\x120\n\x0bDBHAFLBPODA\x18\r\x20\x01(\x0b2\
    \x0c.LMCBNBOMNKKH\0R\x0bDBHAFLBPODA\x120\n\x0bEMJEDJFEFHJ\x18\t\x20\x01(\
    \x0b2\x0c.CAHPBPEEPGBH\0R\x0bEMJEDJFEFHJ\x120\n\x0bONMPCHLDDGA\x18\n\x20\
    \x01(\x0b2\x0c.AJBFJOEIMDMH\0R\x0bONMPCHLDDGA\x120\n\x0bENCLLODMEJH\x18\
    \x05\x20\x01(\x0b2\x0c.ACJICCNKCLKH\0R\x0bENCLLODMEJH\x120\n\x0bMHEGJKOF\
    OLB\x18\x0e\x20\x01(\x0b2\x0c.MBHPHEIJPMGH\0R\x0bMHEGJKOFOLB\x120\n\x0bK\
    EKMKBFHEED\x18\x03\x20\x01(\x0b2\x0c.KFJJAGAICDEH\0R\x0bKEKMKBFHEED\x120\
    \n\x0bFHBOCHDINAB\x18\x04\x20\x01(\x0b2\x0c.DBIACBDCPHEH\0R\x0bFHBOCHDIN\
    AB\x12\"\n\x0bFMPDFFAGKFO\x18\x02\x20\x01(\rH\0R\x0bFMPDFFAGKFOB\r\n\x0b\
    AOPHFJENLGBb\x06proto3\
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
            let mut deps = ::std::vec::Vec::with_capacity(7);
            deps.push(super::ACJICCNKCLK::file_descriptor().clone());
            deps.push(super::AJBFJOEIMDM::file_descriptor().clone());
            deps.push(super::CAHPBPEEPGB::file_descriptor().clone());
            deps.push(super::DBIACBDCPHE::file_descriptor().clone());
            deps.push(super::KFJJAGAICDE::file_descriptor().clone());
            deps.push(super::LMCBNBOMNKK::file_descriptor().clone());
            deps.push(super::MBHPHEIJPMG::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(ClockParkHandleWaitOperationCsReq::generated_message_descriptor_data());
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
