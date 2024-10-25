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

//! Generated file from `CmdAdventureType.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

#[derive(Clone,Copy,PartialEq,Eq,Debug,Hash)]
// @@protoc_insertion_point(enum:CmdAdventureType)
pub enum CmdAdventureType {
    // @@protoc_insertion_point(enum_value:CmdAdventureType.CmdAdventureTypeNone)
    CmdAdventureTypeNone = 0,
    // @@protoc_insertion_point(enum_value:CmdAdventureType.CmdGetFarmStageGachaInfoScRsp)
    CmdGetFarmStageGachaInfoScRsp = 1342,
    // @@protoc_insertion_point(enum_value:CmdAdventureType.CmdGetFarmStageGachaInfoCsReq)
    CmdGetFarmStageGachaInfoCsReq = 1383,
    // @@protoc_insertion_point(enum_value:CmdAdventureType.CmdEnterAdventureCsReq)
    CmdEnterAdventureCsReq = 1398,
    // @@protoc_insertion_point(enum_value:CmdAdventureType.CmdEnterAdventureScRsp)
    CmdEnterAdventureScRsp = 1371,
}

impl ::protobuf::Enum for CmdAdventureType {
    const NAME: &'static str = "CmdAdventureType";

    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<CmdAdventureType> {
        match value {
            0 => ::std::option::Option::Some(CmdAdventureType::CmdAdventureTypeNone),
            1342 => ::std::option::Option::Some(CmdAdventureType::CmdGetFarmStageGachaInfoScRsp),
            1383 => ::std::option::Option::Some(CmdAdventureType::CmdGetFarmStageGachaInfoCsReq),
            1398 => ::std::option::Option::Some(CmdAdventureType::CmdEnterAdventureCsReq),
            1371 => ::std::option::Option::Some(CmdAdventureType::CmdEnterAdventureScRsp),
            _ => ::std::option::Option::None
        }
    }

    fn from_str(str: &str) -> ::std::option::Option<CmdAdventureType> {
        match str {
            "CmdAdventureTypeNone" => ::std::option::Option::Some(CmdAdventureType::CmdAdventureTypeNone),
            "CmdGetFarmStageGachaInfoScRsp" => ::std::option::Option::Some(CmdAdventureType::CmdGetFarmStageGachaInfoScRsp),
            "CmdGetFarmStageGachaInfoCsReq" => ::std::option::Option::Some(CmdAdventureType::CmdGetFarmStageGachaInfoCsReq),
            "CmdEnterAdventureCsReq" => ::std::option::Option::Some(CmdAdventureType::CmdEnterAdventureCsReq),
            "CmdEnterAdventureScRsp" => ::std::option::Option::Some(CmdAdventureType::CmdEnterAdventureScRsp),
            _ => ::std::option::Option::None
        }
    }

    const VALUES: &'static [CmdAdventureType] = &[
        CmdAdventureType::CmdAdventureTypeNone,
        CmdAdventureType::CmdGetFarmStageGachaInfoScRsp,
        CmdAdventureType::CmdGetFarmStageGachaInfoCsReq,
        CmdAdventureType::CmdEnterAdventureCsReq,
        CmdAdventureType::CmdEnterAdventureScRsp,
    ];
}

impl ::protobuf::EnumFull for CmdAdventureType {
    fn enum_descriptor() -> ::protobuf::reflect::EnumDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().enum_by_package_relative_name("CmdAdventureType").unwrap()).clone()
    }

    fn descriptor(&self) -> ::protobuf::reflect::EnumValueDescriptor {
        let index = match self {
            CmdAdventureType::CmdAdventureTypeNone => 0,
            CmdAdventureType::CmdGetFarmStageGachaInfoScRsp => 1,
            CmdAdventureType::CmdGetFarmStageGachaInfoCsReq => 2,
            CmdAdventureType::CmdEnterAdventureCsReq => 3,
            CmdAdventureType::CmdEnterAdventureScRsp => 4,
        };
        Self::enum_descriptor().value_by_index(index)
    }
}

impl ::std::default::Default for CmdAdventureType {
    fn default() -> Self {
        CmdAdventureType::CmdAdventureTypeNone
    }
}

impl CmdAdventureType {
    fn generated_enum_descriptor_data() -> ::protobuf::reflect::GeneratedEnumDescriptorData {
        ::protobuf::reflect::GeneratedEnumDescriptorData::new::<CmdAdventureType>("CmdAdventureType")
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x16CmdAdventureType.proto*\xae\x01\n\x10CmdAdventureType\x12\x18\n\
    \x14CmdAdventureTypeNone\x10\0\x12\"\n\x1dCmdGetFarmStageGachaInfoScRsp\
    \x10\xbe\n\x12\"\n\x1dCmdGetFarmStageGachaInfoCsReq\x10\xe7\n\x12\x1b\n\
    \x16CmdEnterAdventureCsReq\x10\xf6\n\x12\x1b\n\x16CmdEnterAdventureScRsp\
    \x10\xdb\nb\x06proto3\
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
            enums.push(CmdAdventureType::generated_enum_descriptor_data());
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
