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

//! Generated file from `CmdPetType.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

#[derive(Clone,Copy,PartialEq,Eq,Debug,Hash)]
// @@protoc_insertion_point(enum:CmdPetType)
pub enum CmdPetType {
    // @@protoc_insertion_point(enum_value:CmdPetType.CmdPetTypeNone)
    CmdPetTypeNone = 0,
    // @@protoc_insertion_point(enum_value:CmdPetType.CmdRecallPetCsReq)
    CmdRecallPetCsReq = 7644,
    // @@protoc_insertion_point(enum_value:CmdPetType.CmdCurPetChangedScNotify)
    CmdCurPetChangedScNotify = 7624,
    // @@protoc_insertion_point(enum_value:CmdPetType.CmdSummonPetCsReq)
    CmdSummonPetCsReq = 7642,
    // @@protoc_insertion_point(enum_value:CmdPetType.CmdSummonPetScRsp)
    CmdSummonPetScRsp = 7602,
    // @@protoc_insertion_point(enum_value:CmdPetType.CmdGetPetDataScRsp)
    CmdGetPetDataScRsp = 7637,
    // @@protoc_insertion_point(enum_value:CmdPetType.CmdRecallPetScRsp)
    CmdRecallPetScRsp = 7610,
    // @@protoc_insertion_point(enum_value:CmdPetType.CmdGetPetDataCsReq)
    CmdGetPetDataCsReq = 7629,
}

impl ::protobuf::Enum for CmdPetType {
    const NAME: &'static str = "CmdPetType";

    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<CmdPetType> {
        match value {
            0 => ::std::option::Option::Some(CmdPetType::CmdPetTypeNone),
            7644 => ::std::option::Option::Some(CmdPetType::CmdRecallPetCsReq),
            7624 => ::std::option::Option::Some(CmdPetType::CmdCurPetChangedScNotify),
            7642 => ::std::option::Option::Some(CmdPetType::CmdSummonPetCsReq),
            7602 => ::std::option::Option::Some(CmdPetType::CmdSummonPetScRsp),
            7637 => ::std::option::Option::Some(CmdPetType::CmdGetPetDataScRsp),
            7610 => ::std::option::Option::Some(CmdPetType::CmdRecallPetScRsp),
            7629 => ::std::option::Option::Some(CmdPetType::CmdGetPetDataCsReq),
            _ => ::std::option::Option::None
        }
    }

    fn from_str(str: &str) -> ::std::option::Option<CmdPetType> {
        match str {
            "CmdPetTypeNone" => ::std::option::Option::Some(CmdPetType::CmdPetTypeNone),
            "CmdRecallPetCsReq" => ::std::option::Option::Some(CmdPetType::CmdRecallPetCsReq),
            "CmdCurPetChangedScNotify" => ::std::option::Option::Some(CmdPetType::CmdCurPetChangedScNotify),
            "CmdSummonPetCsReq" => ::std::option::Option::Some(CmdPetType::CmdSummonPetCsReq),
            "CmdSummonPetScRsp" => ::std::option::Option::Some(CmdPetType::CmdSummonPetScRsp),
            "CmdGetPetDataScRsp" => ::std::option::Option::Some(CmdPetType::CmdGetPetDataScRsp),
            "CmdRecallPetScRsp" => ::std::option::Option::Some(CmdPetType::CmdRecallPetScRsp),
            "CmdGetPetDataCsReq" => ::std::option::Option::Some(CmdPetType::CmdGetPetDataCsReq),
            _ => ::std::option::Option::None
        }
    }

    const VALUES: &'static [CmdPetType] = &[
        CmdPetType::CmdPetTypeNone,
        CmdPetType::CmdRecallPetCsReq,
        CmdPetType::CmdCurPetChangedScNotify,
        CmdPetType::CmdSummonPetCsReq,
        CmdPetType::CmdSummonPetScRsp,
        CmdPetType::CmdGetPetDataScRsp,
        CmdPetType::CmdRecallPetScRsp,
        CmdPetType::CmdGetPetDataCsReq,
    ];
}

impl ::protobuf::EnumFull for CmdPetType {
    fn enum_descriptor() -> ::protobuf::reflect::EnumDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().enum_by_package_relative_name("CmdPetType").unwrap()).clone()
    }

    fn descriptor(&self) -> ::protobuf::reflect::EnumValueDescriptor {
        let index = match self {
            CmdPetType::CmdPetTypeNone => 0,
            CmdPetType::CmdRecallPetCsReq => 1,
            CmdPetType::CmdCurPetChangedScNotify => 2,
            CmdPetType::CmdSummonPetCsReq => 3,
            CmdPetType::CmdSummonPetScRsp => 4,
            CmdPetType::CmdGetPetDataScRsp => 5,
            CmdPetType::CmdRecallPetScRsp => 6,
            CmdPetType::CmdGetPetDataCsReq => 7,
        };
        Self::enum_descriptor().value_by_index(index)
    }
}

impl ::std::default::Default for CmdPetType {
    fn default() -> Self {
        CmdPetType::CmdPetTypeNone
    }
}

impl CmdPetType {
    fn generated_enum_descriptor_data() -> ::protobuf::reflect::GeneratedEnumDescriptorData {
        ::protobuf::reflect::GeneratedEnumDescriptorData::new::<CmdPetType>("CmdPetType")
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x10CmdPetType.proto*\xd1\x01\n\nCmdPetType\x12\x12\n\x0eCmdPetTypeNon\
    e\x10\0\x12\x16\n\x11CmdRecallPetCsReq\x10\xdc;\x12\x1d\n\x18CmdCurPetCh\
    angedScNotify\x10\xc8;\x12\x16\n\x11CmdSummonPetCsReq\x10\xda;\x12\x16\n\
    \x11CmdSummonPetScRsp\x10\xb2;\x12\x17\n\x12CmdGetPetDataScRsp\x10\xd5;\
    \x12\x16\n\x11CmdRecallPetScRsp\x10\xba;\x12\x17\n\x12CmdGetPetDataCsReq\
    \x10\xcd;b\x06proto3\
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
            enums.push(CmdPetType::generated_enum_descriptor_data());
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