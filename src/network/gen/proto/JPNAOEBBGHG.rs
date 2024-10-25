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

//! Generated file from `JPNAOEBBGHG.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

#[derive(Clone,Copy,PartialEq,Eq,Debug,Hash)]
// @@protoc_insertion_point(enum:JPNAOEBBGHG)
pub enum JPNAOEBBGHG {
    // @@protoc_insertion_point(enum_value:JPNAOEBBGHG.SWORD_TRAINING_STATUS_TYPE_NONE)
    SWORD_TRAINING_STATUS_TYPE_NONE = 0,
    // @@protoc_insertion_point(enum_value:JPNAOEBBGHG.SWORD_TRAINING_STATUS_TYPE_POWER)
    SWORD_TRAINING_STATUS_TYPE_POWER = 1,
    // @@protoc_insertion_point(enum_value:JPNAOEBBGHG.SWORD_TRAINING_STATUS_TYPE_AGILITY)
    SWORD_TRAINING_STATUS_TYPE_AGILITY = 2,
    // @@protoc_insertion_point(enum_value:JPNAOEBBGHG.SWORD_TRAINING_STATUS_TYPE_TOUGHNESS)
    SWORD_TRAINING_STATUS_TYPE_TOUGHNESS = 3,
    // @@protoc_insertion_point(enum_value:JPNAOEBBGHG.SWORD_TRAINING_STATUS_TYPE_PERCEPTION)
    SWORD_TRAINING_STATUS_TYPE_PERCEPTION = 4,
    // @@protoc_insertion_point(enum_value:JPNAOEBBGHG._SWORD_TRAINING_STATUS_TYPE_MAX)
    _SWORD_TRAINING_STATUS_TYPE_MAX = 5,
}

impl ::protobuf::Enum for JPNAOEBBGHG {
    const NAME: &'static str = "JPNAOEBBGHG";

    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<JPNAOEBBGHG> {
        match value {
            0 => ::std::option::Option::Some(JPNAOEBBGHG::SWORD_TRAINING_STATUS_TYPE_NONE),
            1 => ::std::option::Option::Some(JPNAOEBBGHG::SWORD_TRAINING_STATUS_TYPE_POWER),
            2 => ::std::option::Option::Some(JPNAOEBBGHG::SWORD_TRAINING_STATUS_TYPE_AGILITY),
            3 => ::std::option::Option::Some(JPNAOEBBGHG::SWORD_TRAINING_STATUS_TYPE_TOUGHNESS),
            4 => ::std::option::Option::Some(JPNAOEBBGHG::SWORD_TRAINING_STATUS_TYPE_PERCEPTION),
            5 => ::std::option::Option::Some(JPNAOEBBGHG::_SWORD_TRAINING_STATUS_TYPE_MAX),
            _ => ::std::option::Option::None
        }
    }

    fn from_str(str: &str) -> ::std::option::Option<JPNAOEBBGHG> {
        match str {
            "SWORD_TRAINING_STATUS_TYPE_NONE" => ::std::option::Option::Some(JPNAOEBBGHG::SWORD_TRAINING_STATUS_TYPE_NONE),
            "SWORD_TRAINING_STATUS_TYPE_POWER" => ::std::option::Option::Some(JPNAOEBBGHG::SWORD_TRAINING_STATUS_TYPE_POWER),
            "SWORD_TRAINING_STATUS_TYPE_AGILITY" => ::std::option::Option::Some(JPNAOEBBGHG::SWORD_TRAINING_STATUS_TYPE_AGILITY),
            "SWORD_TRAINING_STATUS_TYPE_TOUGHNESS" => ::std::option::Option::Some(JPNAOEBBGHG::SWORD_TRAINING_STATUS_TYPE_TOUGHNESS),
            "SWORD_TRAINING_STATUS_TYPE_PERCEPTION" => ::std::option::Option::Some(JPNAOEBBGHG::SWORD_TRAINING_STATUS_TYPE_PERCEPTION),
            "_SWORD_TRAINING_STATUS_TYPE_MAX" => ::std::option::Option::Some(JPNAOEBBGHG::_SWORD_TRAINING_STATUS_TYPE_MAX),
            _ => ::std::option::Option::None
        }
    }

    const VALUES: &'static [JPNAOEBBGHG] = &[
        JPNAOEBBGHG::SWORD_TRAINING_STATUS_TYPE_NONE,
        JPNAOEBBGHG::SWORD_TRAINING_STATUS_TYPE_POWER,
        JPNAOEBBGHG::SWORD_TRAINING_STATUS_TYPE_AGILITY,
        JPNAOEBBGHG::SWORD_TRAINING_STATUS_TYPE_TOUGHNESS,
        JPNAOEBBGHG::SWORD_TRAINING_STATUS_TYPE_PERCEPTION,
        JPNAOEBBGHG::_SWORD_TRAINING_STATUS_TYPE_MAX,
    ];
}

impl ::protobuf::EnumFull for JPNAOEBBGHG {
    fn enum_descriptor() -> ::protobuf::reflect::EnumDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().enum_by_package_relative_name("JPNAOEBBGHG").unwrap()).clone()
    }

    fn descriptor(&self) -> ::protobuf::reflect::EnumValueDescriptor {
        let index = *self as usize;
        Self::enum_descriptor().value_by_index(index)
    }
}

impl ::std::default::Default for JPNAOEBBGHG {
    fn default() -> Self {
        JPNAOEBBGHG::SWORD_TRAINING_STATUS_TYPE_NONE
    }
}

impl JPNAOEBBGHG {
    fn generated_enum_descriptor_data() -> ::protobuf::reflect::GeneratedEnumDescriptorData {
        ::protobuf::reflect::GeneratedEnumDescriptorData::new::<JPNAOEBBGHG>("JPNAOEBBGHG")
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11JPNAOEBBGHG.proto*\xfa\x01\n\x0bJPNAOEBBGHG\x12#\n\x1fSWORD_TRAINI\
    NG_STATUS_TYPE_NONE\x10\0\x12$\n\x20SWORD_TRAINING_STATUS_TYPE_POWER\x10\
    \x01\x12&\n\"SWORD_TRAINING_STATUS_TYPE_AGILITY\x10\x02\x12(\n$SWORD_TRA\
    INING_STATUS_TYPE_TOUGHNESS\x10\x03\x12)\n%SWORD_TRAINING_STATUS_TYPE_PE\
    RCEPTION\x10\x04\x12#\n\x1f_SWORD_TRAINING_STATUS_TYPE_MAX\x10\x05b\x06p\
    roto3\
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
            let mut deps = ::std::vec::Vec::with_capacity(0);
            let mut messages = ::std::vec::Vec::with_capacity(0);
            let mut enums = ::std::vec::Vec::with_capacity(1);
            enums.push(JPNAOEBBGHG::generated_enum_descriptor_data());
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
