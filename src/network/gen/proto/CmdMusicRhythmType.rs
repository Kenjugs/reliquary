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

//! Generated file from `CmdMusicRhythmType.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

#[derive(Clone,Copy,PartialEq,Eq,Debug,Hash)]
// @@protoc_insertion_point(enum:CmdMusicRhythmType)
pub enum CmdMusicRhythmType {
    // @@protoc_insertion_point(enum_value:CmdMusicRhythmType.CmdMusicRhythmTypeNone)
    CmdMusicRhythmTypeNone = 0,
    // @@protoc_insertion_point(enum_value:CmdMusicRhythmType.CmdMusicRhythmFinishLevelScRsp)
    CmdMusicRhythmFinishLevelScRsp = 7587,
    // @@protoc_insertion_point(enum_value:CmdMusicRhythmType.CmdMusicRhythmStartLevelCsReq)
    CmdMusicRhythmStartLevelCsReq = 7599,
    // @@protoc_insertion_point(enum_value:CmdMusicRhythmType.CmdMusicRhythmUnlockTrackScNotify)
    CmdMusicRhythmUnlockTrackScNotify = 7598,
    // @@protoc_insertion_point(enum_value:CmdMusicRhythmType.CmdMusicRhythmUnlockSongNotify)
    CmdMusicRhythmUnlockSongNotify = 7573,
    // @@protoc_insertion_point(enum_value:CmdMusicRhythmType.CmdMusicRhythmDataCsReq)
    CmdMusicRhythmDataCsReq = 7591,
    // @@protoc_insertion_point(enum_value:CmdMusicRhythmType.CmdMusicRhythmDataScRsp)
    CmdMusicRhythmDataScRsp = 7577,
    // @@protoc_insertion_point(enum_value:CmdMusicRhythmType.CmdMusicRhythmSaveSongConfigDataCsReq)
    CmdMusicRhythmSaveSongConfigDataCsReq = 7584,
    // @@protoc_insertion_point(enum_value:CmdMusicRhythmType.CmdMusicRhythmStartLevelScRsp)
    CmdMusicRhythmStartLevelScRsp = 7571,
    // @@protoc_insertion_point(enum_value:CmdMusicRhythmType.CmdMusicRhythmSaveSongConfigDataScRsp)
    CmdMusicRhythmSaveSongConfigDataScRsp = 7585,
    // @@protoc_insertion_point(enum_value:CmdMusicRhythmType.CmdMusicRhythmUnlockSongSfxScNotify)
    CmdMusicRhythmUnlockSongSfxScNotify = 7580,
    // @@protoc_insertion_point(enum_value:CmdMusicRhythmType.CmdMusicRhythmFinishLevelCsReq)
    CmdMusicRhythmFinishLevelCsReq = 7578,
    // @@protoc_insertion_point(enum_value:CmdMusicRhythmType.CmdMusicRhythmMaxDifficultyLevelsUnlockNotify)
    CmdMusicRhythmMaxDifficultyLevelsUnlockNotify = 7575,
}

impl ::protobuf::Enum for CmdMusicRhythmType {
    const NAME: &'static str = "CmdMusicRhythmType";

    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<CmdMusicRhythmType> {
        match value {
            0 => ::std::option::Option::Some(CmdMusicRhythmType::CmdMusicRhythmTypeNone),
            7587 => ::std::option::Option::Some(CmdMusicRhythmType::CmdMusicRhythmFinishLevelScRsp),
            7599 => ::std::option::Option::Some(CmdMusicRhythmType::CmdMusicRhythmStartLevelCsReq),
            7598 => ::std::option::Option::Some(CmdMusicRhythmType::CmdMusicRhythmUnlockTrackScNotify),
            7573 => ::std::option::Option::Some(CmdMusicRhythmType::CmdMusicRhythmUnlockSongNotify),
            7591 => ::std::option::Option::Some(CmdMusicRhythmType::CmdMusicRhythmDataCsReq),
            7577 => ::std::option::Option::Some(CmdMusicRhythmType::CmdMusicRhythmDataScRsp),
            7584 => ::std::option::Option::Some(CmdMusicRhythmType::CmdMusicRhythmSaveSongConfigDataCsReq),
            7571 => ::std::option::Option::Some(CmdMusicRhythmType::CmdMusicRhythmStartLevelScRsp),
            7585 => ::std::option::Option::Some(CmdMusicRhythmType::CmdMusicRhythmSaveSongConfigDataScRsp),
            7580 => ::std::option::Option::Some(CmdMusicRhythmType::CmdMusicRhythmUnlockSongSfxScNotify),
            7578 => ::std::option::Option::Some(CmdMusicRhythmType::CmdMusicRhythmFinishLevelCsReq),
            7575 => ::std::option::Option::Some(CmdMusicRhythmType::CmdMusicRhythmMaxDifficultyLevelsUnlockNotify),
            _ => ::std::option::Option::None
        }
    }

    fn from_str(str: &str) -> ::std::option::Option<CmdMusicRhythmType> {
        match str {
            "CmdMusicRhythmTypeNone" => ::std::option::Option::Some(CmdMusicRhythmType::CmdMusicRhythmTypeNone),
            "CmdMusicRhythmFinishLevelScRsp" => ::std::option::Option::Some(CmdMusicRhythmType::CmdMusicRhythmFinishLevelScRsp),
            "CmdMusicRhythmStartLevelCsReq" => ::std::option::Option::Some(CmdMusicRhythmType::CmdMusicRhythmStartLevelCsReq),
            "CmdMusicRhythmUnlockTrackScNotify" => ::std::option::Option::Some(CmdMusicRhythmType::CmdMusicRhythmUnlockTrackScNotify),
            "CmdMusicRhythmUnlockSongNotify" => ::std::option::Option::Some(CmdMusicRhythmType::CmdMusicRhythmUnlockSongNotify),
            "CmdMusicRhythmDataCsReq" => ::std::option::Option::Some(CmdMusicRhythmType::CmdMusicRhythmDataCsReq),
            "CmdMusicRhythmDataScRsp" => ::std::option::Option::Some(CmdMusicRhythmType::CmdMusicRhythmDataScRsp),
            "CmdMusicRhythmSaveSongConfigDataCsReq" => ::std::option::Option::Some(CmdMusicRhythmType::CmdMusicRhythmSaveSongConfigDataCsReq),
            "CmdMusicRhythmStartLevelScRsp" => ::std::option::Option::Some(CmdMusicRhythmType::CmdMusicRhythmStartLevelScRsp),
            "CmdMusicRhythmSaveSongConfigDataScRsp" => ::std::option::Option::Some(CmdMusicRhythmType::CmdMusicRhythmSaveSongConfigDataScRsp),
            "CmdMusicRhythmUnlockSongSfxScNotify" => ::std::option::Option::Some(CmdMusicRhythmType::CmdMusicRhythmUnlockSongSfxScNotify),
            "CmdMusicRhythmFinishLevelCsReq" => ::std::option::Option::Some(CmdMusicRhythmType::CmdMusicRhythmFinishLevelCsReq),
            "CmdMusicRhythmMaxDifficultyLevelsUnlockNotify" => ::std::option::Option::Some(CmdMusicRhythmType::CmdMusicRhythmMaxDifficultyLevelsUnlockNotify),
            _ => ::std::option::Option::None
        }
    }

    const VALUES: &'static [CmdMusicRhythmType] = &[
        CmdMusicRhythmType::CmdMusicRhythmTypeNone,
        CmdMusicRhythmType::CmdMusicRhythmFinishLevelScRsp,
        CmdMusicRhythmType::CmdMusicRhythmStartLevelCsReq,
        CmdMusicRhythmType::CmdMusicRhythmUnlockTrackScNotify,
        CmdMusicRhythmType::CmdMusicRhythmUnlockSongNotify,
        CmdMusicRhythmType::CmdMusicRhythmDataCsReq,
        CmdMusicRhythmType::CmdMusicRhythmDataScRsp,
        CmdMusicRhythmType::CmdMusicRhythmSaveSongConfigDataCsReq,
        CmdMusicRhythmType::CmdMusicRhythmStartLevelScRsp,
        CmdMusicRhythmType::CmdMusicRhythmSaveSongConfigDataScRsp,
        CmdMusicRhythmType::CmdMusicRhythmUnlockSongSfxScNotify,
        CmdMusicRhythmType::CmdMusicRhythmFinishLevelCsReq,
        CmdMusicRhythmType::CmdMusicRhythmMaxDifficultyLevelsUnlockNotify,
    ];
}

impl ::protobuf::EnumFull for CmdMusicRhythmType {
    fn enum_descriptor() -> ::protobuf::reflect::EnumDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().enum_by_package_relative_name("CmdMusicRhythmType").unwrap()).clone()
    }

    fn descriptor(&self) -> ::protobuf::reflect::EnumValueDescriptor {
        let index = match self {
            CmdMusicRhythmType::CmdMusicRhythmTypeNone => 0,
            CmdMusicRhythmType::CmdMusicRhythmFinishLevelScRsp => 1,
            CmdMusicRhythmType::CmdMusicRhythmStartLevelCsReq => 2,
            CmdMusicRhythmType::CmdMusicRhythmUnlockTrackScNotify => 3,
            CmdMusicRhythmType::CmdMusicRhythmUnlockSongNotify => 4,
            CmdMusicRhythmType::CmdMusicRhythmDataCsReq => 5,
            CmdMusicRhythmType::CmdMusicRhythmDataScRsp => 6,
            CmdMusicRhythmType::CmdMusicRhythmSaveSongConfigDataCsReq => 7,
            CmdMusicRhythmType::CmdMusicRhythmStartLevelScRsp => 8,
            CmdMusicRhythmType::CmdMusicRhythmSaveSongConfigDataScRsp => 9,
            CmdMusicRhythmType::CmdMusicRhythmUnlockSongSfxScNotify => 10,
            CmdMusicRhythmType::CmdMusicRhythmFinishLevelCsReq => 11,
            CmdMusicRhythmType::CmdMusicRhythmMaxDifficultyLevelsUnlockNotify => 12,
        };
        Self::enum_descriptor().value_by_index(index)
    }
}

impl ::std::default::Default for CmdMusicRhythmType {
    fn default() -> Self {
        CmdMusicRhythmType::CmdMusicRhythmTypeNone
    }
}

impl CmdMusicRhythmType {
    fn generated_enum_descriptor_data() -> ::protobuf::reflect::GeneratedEnumDescriptorData {
        ::protobuf::reflect::GeneratedEnumDescriptorData::new::<CmdMusicRhythmType>("CmdMusicRhythmType")
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x18CmdMusicRhythmType.proto*\x81\x04\n\x12CmdMusicRhythmType\x12\x1a\
    \n\x16CmdMusicRhythmTypeNone\x10\0\x12#\n\x1eCmdMusicRhythmFinishLevelSc\
    Rsp\x10\xa3;\x12\"\n\x1dCmdMusicRhythmStartLevelCsReq\x10\xaf;\x12&\n!Cm\
    dMusicRhythmUnlockTrackScNotify\x10\xae;\x12#\n\x1eCmdMusicRhythmUnlockS\
    ongNotify\x10\x95;\x12\x1c\n\x17CmdMusicRhythmDataCsReq\x10\xa7;\x12\x1c\
    \n\x17CmdMusicRhythmDataScRsp\x10\x99;\x12*\n%CmdMusicRhythmSaveSongConf\
    igDataCsReq\x10\xa0;\x12\"\n\x1dCmdMusicRhythmStartLevelScRsp\x10\x93;\
    \x12*\n%CmdMusicRhythmSaveSongConfigDataScRsp\x10\xa1;\x12(\n#CmdMusicRh\
    ythmUnlockSongSfxScNotify\x10\x9c;\x12#\n\x1eCmdMusicRhythmFinishLevelCs\
    Req\x10\x9a;\x122\n-CmdMusicRhythmMaxDifficultyLevelsUnlockNotify\x10\
    \x97;b\x06proto3\
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
            enums.push(CmdMusicRhythmType::generated_enum_descriptor_data());
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