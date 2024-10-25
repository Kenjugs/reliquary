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

//! Generated file from `CmdDailyActiveType.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

#[derive(Clone,Copy,PartialEq,Eq,Debug,Hash)]
// @@protoc_insertion_point(enum:CmdDailyActiveType)
pub enum CmdDailyActiveType {
    // @@protoc_insertion_point(enum_value:CmdDailyActiveType.CmdDailyActiveTypeNone)
    CmdDailyActiveTypeNone = 0,
    // @@protoc_insertion_point(enum_value:CmdDailyActiveType.CmdDailyActiveInfoNotify)
    CmdDailyActiveInfoNotify = 3379,
    // @@protoc_insertion_point(enum_value:CmdDailyActiveType.CmdGetDailyActiveInfoCsReq)
    CmdGetDailyActiveInfoCsReq = 3383,
    // @@protoc_insertion_point(enum_value:CmdDailyActiveType.CmdTakeApRewardScRsp)
    CmdTakeApRewardScRsp = 3371,
    // @@protoc_insertion_point(enum_value:CmdDailyActiveType.CmdTakeApRewardCsReq)
    CmdTakeApRewardCsReq = 3398,
    // @@protoc_insertion_point(enum_value:CmdDailyActiveType.CmdTakeAllApRewardScRsp)
    CmdTakeAllApRewardScRsp = 3333,
    // @@protoc_insertion_point(enum_value:CmdDailyActiveType.CmdTakeAllApRewardCsReq)
    CmdTakeAllApRewardCsReq = 3377,
    // @@protoc_insertion_point(enum_value:CmdDailyActiveType.CmdGetDailyActiveInfoScRsp)
    CmdGetDailyActiveInfoScRsp = 3342,
}

impl ::protobuf::Enum for CmdDailyActiveType {
    const NAME: &'static str = "CmdDailyActiveType";

    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<CmdDailyActiveType> {
        match value {
            0 => ::std::option::Option::Some(CmdDailyActiveType::CmdDailyActiveTypeNone),
            3379 => ::std::option::Option::Some(CmdDailyActiveType::CmdDailyActiveInfoNotify),
            3383 => ::std::option::Option::Some(CmdDailyActiveType::CmdGetDailyActiveInfoCsReq),
            3371 => ::std::option::Option::Some(CmdDailyActiveType::CmdTakeApRewardScRsp),
            3398 => ::std::option::Option::Some(CmdDailyActiveType::CmdTakeApRewardCsReq),
            3333 => ::std::option::Option::Some(CmdDailyActiveType::CmdTakeAllApRewardScRsp),
            3377 => ::std::option::Option::Some(CmdDailyActiveType::CmdTakeAllApRewardCsReq),
            3342 => ::std::option::Option::Some(CmdDailyActiveType::CmdGetDailyActiveInfoScRsp),
            _ => ::std::option::Option::None
        }
    }

    fn from_str(str: &str) -> ::std::option::Option<CmdDailyActiveType> {
        match str {
            "CmdDailyActiveTypeNone" => ::std::option::Option::Some(CmdDailyActiveType::CmdDailyActiveTypeNone),
            "CmdDailyActiveInfoNotify" => ::std::option::Option::Some(CmdDailyActiveType::CmdDailyActiveInfoNotify),
            "CmdGetDailyActiveInfoCsReq" => ::std::option::Option::Some(CmdDailyActiveType::CmdGetDailyActiveInfoCsReq),
            "CmdTakeApRewardScRsp" => ::std::option::Option::Some(CmdDailyActiveType::CmdTakeApRewardScRsp),
            "CmdTakeApRewardCsReq" => ::std::option::Option::Some(CmdDailyActiveType::CmdTakeApRewardCsReq),
            "CmdTakeAllApRewardScRsp" => ::std::option::Option::Some(CmdDailyActiveType::CmdTakeAllApRewardScRsp),
            "CmdTakeAllApRewardCsReq" => ::std::option::Option::Some(CmdDailyActiveType::CmdTakeAllApRewardCsReq),
            "CmdGetDailyActiveInfoScRsp" => ::std::option::Option::Some(CmdDailyActiveType::CmdGetDailyActiveInfoScRsp),
            _ => ::std::option::Option::None
        }
    }

    const VALUES: &'static [CmdDailyActiveType] = &[
        CmdDailyActiveType::CmdDailyActiveTypeNone,
        CmdDailyActiveType::CmdDailyActiveInfoNotify,
        CmdDailyActiveType::CmdGetDailyActiveInfoCsReq,
        CmdDailyActiveType::CmdTakeApRewardScRsp,
        CmdDailyActiveType::CmdTakeApRewardCsReq,
        CmdDailyActiveType::CmdTakeAllApRewardScRsp,
        CmdDailyActiveType::CmdTakeAllApRewardCsReq,
        CmdDailyActiveType::CmdGetDailyActiveInfoScRsp,
    ];
}

impl ::protobuf::EnumFull for CmdDailyActiveType {
    fn enum_descriptor() -> ::protobuf::reflect::EnumDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().enum_by_package_relative_name("CmdDailyActiveType").unwrap()).clone()
    }

    fn descriptor(&self) -> ::protobuf::reflect::EnumValueDescriptor {
        let index = match self {
            CmdDailyActiveType::CmdDailyActiveTypeNone => 0,
            CmdDailyActiveType::CmdDailyActiveInfoNotify => 1,
            CmdDailyActiveType::CmdGetDailyActiveInfoCsReq => 2,
            CmdDailyActiveType::CmdTakeApRewardScRsp => 3,
            CmdDailyActiveType::CmdTakeApRewardCsReq => 4,
            CmdDailyActiveType::CmdTakeAllApRewardScRsp => 5,
            CmdDailyActiveType::CmdTakeAllApRewardCsReq => 6,
            CmdDailyActiveType::CmdGetDailyActiveInfoScRsp => 7,
        };
        Self::enum_descriptor().value_by_index(index)
    }
}

impl ::std::default::Default for CmdDailyActiveType {
    fn default() -> Self {
        CmdDailyActiveType::CmdDailyActiveTypeNone
    }
}

impl CmdDailyActiveType {
    fn generated_enum_descriptor_data() -> ::protobuf::reflect::GeneratedEnumDescriptorData {
        ::protobuf::reflect::GeneratedEnumDescriptorData::new::<CmdDailyActiveType>("CmdDailyActiveType")
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x18CmdDailyActiveType.proto*\x83\x02\n\x12CmdDailyActiveType\x12\x1a\
    \n\x16CmdDailyActiveTypeNone\x10\0\x12\x1d\n\x18CmdDailyActiveInfoNotify\
    \x10\xb3\x1a\x12\x1f\n\x1aCmdGetDailyActiveInfoCsReq\x10\xb7\x1a\x12\x19\
    \n\x14CmdTakeApRewardScRsp\x10\xab\x1a\x12\x19\n\x14CmdTakeApRewardCsReq\
    \x10\xc6\x1a\x12\x1c\n\x17CmdTakeAllApRewardScRsp\x10\x85\x1a\x12\x1c\n\
    \x17CmdTakeAllApRewardCsReq\x10\xb1\x1a\x12\x1f\n\x1aCmdGetDailyActiveIn\
    foScRsp\x10\x8e\x1ab\x06proto3\
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
            enums.push(CmdDailyActiveType::generated_enum_descriptor_data());
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