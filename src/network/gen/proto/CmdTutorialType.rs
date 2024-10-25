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

//! Generated file from `CmdTutorialType.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

#[derive(Clone,Copy,PartialEq,Eq,Debug,Hash)]
// @@protoc_insertion_point(enum:CmdTutorialType)
pub enum CmdTutorialType {
    // @@protoc_insertion_point(enum_value:CmdTutorialType.CmdTutorialTypeNone)
    CmdTutorialTypeNone = 0,
    // @@protoc_insertion_point(enum_value:CmdTutorialType.CmdFinishTutorialScRsp)
    CmdFinishTutorialScRsp = 1638,
    // @@protoc_insertion_point(enum_value:CmdTutorialType.CmdFinishTutorialCsReq)
    CmdFinishTutorialCsReq = 1628,
    // @@protoc_insertion_point(enum_value:CmdTutorialType.CmdGetTutorialCsReq)
    CmdGetTutorialCsReq = 1698,
    // @@protoc_insertion_point(enum_value:CmdTutorialType.CmdUnlockTutorialScRsp)
    CmdUnlockTutorialScRsp = 1677,
    // @@protoc_insertion_point(enum_value:CmdTutorialType.CmdGetTutorialGuideCsReq)
    CmdGetTutorialGuideCsReq = 1683,
    // @@protoc_insertion_point(enum_value:CmdTutorialType.CmdUnlockTutorialCsReq)
    CmdUnlockTutorialCsReq = 1679,
    // @@protoc_insertion_point(enum_value:CmdTutorialType.CmdFinishTutorialGuideScRsp)
    CmdFinishTutorialGuideScRsp = 1656,
    // @@protoc_insertion_point(enum_value:CmdTutorialType.CmdUnlockTutorialGuideScRsp)
    CmdUnlockTutorialGuideScRsp = 1612,
    // @@protoc_insertion_point(enum_value:CmdTutorialType.CmdGetTutorialGuideScRsp)
    CmdGetTutorialGuideScRsp = 1642,
    // @@protoc_insertion_point(enum_value:CmdTutorialType.CmdFinishTutorialGuideCsReq)
    CmdFinishTutorialGuideCsReq = 1678,
    // @@protoc_insertion_point(enum_value:CmdTutorialType.CmdGetTutorialScRsp)
    CmdGetTutorialScRsp = 1671,
    // @@protoc_insertion_point(enum_value:CmdTutorialType.CmdUnlockTutorialGuideCsReq)
    CmdUnlockTutorialGuideCsReq = 1633,
}

impl ::protobuf::Enum for CmdTutorialType {
    const NAME: &'static str = "CmdTutorialType";

    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<CmdTutorialType> {
        match value {
            0 => ::std::option::Option::Some(CmdTutorialType::CmdTutorialTypeNone),
            1638 => ::std::option::Option::Some(CmdTutorialType::CmdFinishTutorialScRsp),
            1628 => ::std::option::Option::Some(CmdTutorialType::CmdFinishTutorialCsReq),
            1698 => ::std::option::Option::Some(CmdTutorialType::CmdGetTutorialCsReq),
            1677 => ::std::option::Option::Some(CmdTutorialType::CmdUnlockTutorialScRsp),
            1683 => ::std::option::Option::Some(CmdTutorialType::CmdGetTutorialGuideCsReq),
            1679 => ::std::option::Option::Some(CmdTutorialType::CmdUnlockTutorialCsReq),
            1656 => ::std::option::Option::Some(CmdTutorialType::CmdFinishTutorialGuideScRsp),
            1612 => ::std::option::Option::Some(CmdTutorialType::CmdUnlockTutorialGuideScRsp),
            1642 => ::std::option::Option::Some(CmdTutorialType::CmdGetTutorialGuideScRsp),
            1678 => ::std::option::Option::Some(CmdTutorialType::CmdFinishTutorialGuideCsReq),
            1671 => ::std::option::Option::Some(CmdTutorialType::CmdGetTutorialScRsp),
            1633 => ::std::option::Option::Some(CmdTutorialType::CmdUnlockTutorialGuideCsReq),
            _ => ::std::option::Option::None
        }
    }

    fn from_str(str: &str) -> ::std::option::Option<CmdTutorialType> {
        match str {
            "CmdTutorialTypeNone" => ::std::option::Option::Some(CmdTutorialType::CmdTutorialTypeNone),
            "CmdFinishTutorialScRsp" => ::std::option::Option::Some(CmdTutorialType::CmdFinishTutorialScRsp),
            "CmdFinishTutorialCsReq" => ::std::option::Option::Some(CmdTutorialType::CmdFinishTutorialCsReq),
            "CmdGetTutorialCsReq" => ::std::option::Option::Some(CmdTutorialType::CmdGetTutorialCsReq),
            "CmdUnlockTutorialScRsp" => ::std::option::Option::Some(CmdTutorialType::CmdUnlockTutorialScRsp),
            "CmdGetTutorialGuideCsReq" => ::std::option::Option::Some(CmdTutorialType::CmdGetTutorialGuideCsReq),
            "CmdUnlockTutorialCsReq" => ::std::option::Option::Some(CmdTutorialType::CmdUnlockTutorialCsReq),
            "CmdFinishTutorialGuideScRsp" => ::std::option::Option::Some(CmdTutorialType::CmdFinishTutorialGuideScRsp),
            "CmdUnlockTutorialGuideScRsp" => ::std::option::Option::Some(CmdTutorialType::CmdUnlockTutorialGuideScRsp),
            "CmdGetTutorialGuideScRsp" => ::std::option::Option::Some(CmdTutorialType::CmdGetTutorialGuideScRsp),
            "CmdFinishTutorialGuideCsReq" => ::std::option::Option::Some(CmdTutorialType::CmdFinishTutorialGuideCsReq),
            "CmdGetTutorialScRsp" => ::std::option::Option::Some(CmdTutorialType::CmdGetTutorialScRsp),
            "CmdUnlockTutorialGuideCsReq" => ::std::option::Option::Some(CmdTutorialType::CmdUnlockTutorialGuideCsReq),
            _ => ::std::option::Option::None
        }
    }

    const VALUES: &'static [CmdTutorialType] = &[
        CmdTutorialType::CmdTutorialTypeNone,
        CmdTutorialType::CmdFinishTutorialScRsp,
        CmdTutorialType::CmdFinishTutorialCsReq,
        CmdTutorialType::CmdGetTutorialCsReq,
        CmdTutorialType::CmdUnlockTutorialScRsp,
        CmdTutorialType::CmdGetTutorialGuideCsReq,
        CmdTutorialType::CmdUnlockTutorialCsReq,
        CmdTutorialType::CmdFinishTutorialGuideScRsp,
        CmdTutorialType::CmdUnlockTutorialGuideScRsp,
        CmdTutorialType::CmdGetTutorialGuideScRsp,
        CmdTutorialType::CmdFinishTutorialGuideCsReq,
        CmdTutorialType::CmdGetTutorialScRsp,
        CmdTutorialType::CmdUnlockTutorialGuideCsReq,
    ];
}

impl ::protobuf::EnumFull for CmdTutorialType {
    fn enum_descriptor() -> ::protobuf::reflect::EnumDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().enum_by_package_relative_name("CmdTutorialType").unwrap()).clone()
    }

    fn descriptor(&self) -> ::protobuf::reflect::EnumValueDescriptor {
        let index = match self {
            CmdTutorialType::CmdTutorialTypeNone => 0,
            CmdTutorialType::CmdFinishTutorialScRsp => 1,
            CmdTutorialType::CmdFinishTutorialCsReq => 2,
            CmdTutorialType::CmdGetTutorialCsReq => 3,
            CmdTutorialType::CmdUnlockTutorialScRsp => 4,
            CmdTutorialType::CmdGetTutorialGuideCsReq => 5,
            CmdTutorialType::CmdUnlockTutorialCsReq => 6,
            CmdTutorialType::CmdFinishTutorialGuideScRsp => 7,
            CmdTutorialType::CmdUnlockTutorialGuideScRsp => 8,
            CmdTutorialType::CmdGetTutorialGuideScRsp => 9,
            CmdTutorialType::CmdFinishTutorialGuideCsReq => 10,
            CmdTutorialType::CmdGetTutorialScRsp => 11,
            CmdTutorialType::CmdUnlockTutorialGuideCsReq => 12,
        };
        Self::enum_descriptor().value_by_index(index)
    }
}

impl ::std::default::Default for CmdTutorialType {
    fn default() -> Self {
        CmdTutorialType::CmdTutorialTypeNone
    }
}

impl CmdTutorialType {
    fn generated_enum_descriptor_data() -> ::protobuf::reflect::GeneratedEnumDescriptorData {
        ::protobuf::reflect::GeneratedEnumDescriptorData::new::<CmdTutorialType>("CmdTutorialType")
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x15CmdTutorialType.proto*\x98\x03\n\x0fCmdTutorialType\x12\x17\n\x13C\
    mdTutorialTypeNone\x10\0\x12\x1b\n\x16CmdFinishTutorialScRsp\x10\xe6\x0c\
    \x12\x1b\n\x16CmdFinishTutorialCsReq\x10\xdc\x0c\x12\x18\n\x13CmdGetTuto\
    rialCsReq\x10\xa2\r\x12\x1b\n\x16CmdUnlockTutorialScRsp\x10\x8d\r\x12\
    \x1d\n\x18CmdGetTutorialGuideCsReq\x10\x93\r\x12\x1b\n\x16CmdUnlockTutor\
    ialCsReq\x10\x8f\r\x12\x20\n\x1bCmdFinishTutorialGuideScRsp\x10\xf8\x0c\
    \x12\x20\n\x1bCmdUnlockTutorialGuideScRsp\x10\xcc\x0c\x12\x1d\n\x18CmdGe\
    tTutorialGuideScRsp\x10\xea\x0c\x12\x20\n\x1bCmdFinishTutorialGuideCsReq\
    \x10\x8e\r\x12\x18\n\x13CmdGetTutorialScRsp\x10\x87\r\x12\x20\n\x1bCmdUn\
    lockTutorialGuideCsReq\x10\xe1\x0cb\x06proto3\
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
            enums.push(CmdTutorialType::generated_enum_descriptor_data());
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
