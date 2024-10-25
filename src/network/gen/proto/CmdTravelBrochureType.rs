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

//! Generated file from `CmdTravelBrochureType.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

#[derive(Clone,Copy,PartialEq,Eq,Debug,Hash)]
// @@protoc_insertion_point(enum:CmdTravelBrochureType)
pub enum CmdTravelBrochureType {
    // @@protoc_insertion_point(enum_value:CmdTravelBrochureType.CmdTravelBrochureNone)
    CmdTravelBrochureNone = 0,
    // @@protoc_insertion_point(enum_value:CmdTravelBrochureType.CmdTravelBrochureRemovePasterScRsp)
    CmdTravelBrochureRemovePasterScRsp = 6438,
    // @@protoc_insertion_point(enum_value:CmdTravelBrochureType.CmdTravelBrochureGetPasterScNotify)
    CmdTravelBrochureGetPasterScNotify = 6489,
    // @@protoc_insertion_point(enum_value:CmdTravelBrochureType.CmdTravelBrochureSetPageDescStatusScRsp)
    CmdTravelBrochureSetPageDescStatusScRsp = 6420,
    // @@protoc_insertion_point(enum_value:CmdTravelBrochureType.CmdTravelBrochureUpdatePasterPosCsReq)
    CmdTravelBrochureUpdatePasterPosCsReq = 6478,
    // @@protoc_insertion_point(enum_value:CmdTravelBrochureType.CmdTravelBrochureGetDataScRsp)
    CmdTravelBrochureGetDataScRsp = 6471,
    // @@protoc_insertion_point(enum_value:CmdTravelBrochureType.CmdTravelBrochureSelectMessageScRsp)
    CmdTravelBrochureSelectMessageScRsp = 6477,
    // @@protoc_insertion_point(enum_value:CmdTravelBrochureType.CmdTravelBrochureGetDataCsReq)
    CmdTravelBrochureGetDataCsReq = 6498,
    // @@protoc_insertion_point(enum_value:CmdTravelBrochureType.CmdTravelBrochureSetCustomValueCsReq)
    CmdTravelBrochureSetCustomValueCsReq = 6466,
    // @@protoc_insertion_point(enum_value:CmdTravelBrochureType.CmdTravelBrochurePageResetCsReq)
    CmdTravelBrochurePageResetCsReq = 6450,
    // @@protoc_insertion_point(enum_value:CmdTravelBrochureType.CmdTravelBrochureApplyPasterCsReq)
    CmdTravelBrochureApplyPasterCsReq = 6433,
    // @@protoc_insertion_point(enum_value:CmdTravelBrochureType.CmdTravelBrochureApplyPasterListCsReq)
    CmdTravelBrochureApplyPasterListCsReq = 6404,
    // @@protoc_insertion_point(enum_value:CmdTravelBrochureType.CmdTravelBrochureApplyPasterScRsp)
    CmdTravelBrochureApplyPasterScRsp = 6412,
    // @@protoc_insertion_point(enum_value:CmdTravelBrochureType.CmdTravelBrochureSelectMessageCsReq)
    CmdTravelBrochureSelectMessageCsReq = 6479,
    // @@protoc_insertion_point(enum_value:CmdTravelBrochureType.CmdTravelBrochureApplyPasterListScRsp)
    CmdTravelBrochureApplyPasterListScRsp = 6460,
    // @@protoc_insertion_point(enum_value:CmdTravelBrochureType.CmdTravelBrochureRemovePasterCsReq)
    CmdTravelBrochureRemovePasterCsReq = 6428,
    // @@protoc_insertion_point(enum_value:CmdTravelBrochureType.CmdTravelBrochureSetPageDescStatusCsReq)
    CmdTravelBrochureSetPageDescStatusCsReq = 6445,
    // @@protoc_insertion_point(enum_value:CmdTravelBrochureType.CmdTravelBrochureUpdatePasterPosScRsp)
    CmdTravelBrochureUpdatePasterPosScRsp = 6456,
    // @@protoc_insertion_point(enum_value:CmdTravelBrochureType.CmdTravelBrochureSetCustomValueScRsp)
    CmdTravelBrochureSetCustomValueScRsp = 6405,
    // @@protoc_insertion_point(enum_value:CmdTravelBrochureType.CmdTravelBrochurePageResetScRsp)
    CmdTravelBrochurePageResetScRsp = 6431,
    // @@protoc_insertion_point(enum_value:CmdTravelBrochureType.CmdTravelBrochurePageUnlockScNotify)
    CmdTravelBrochurePageUnlockScNotify = 6483,
}

impl ::protobuf::Enum for CmdTravelBrochureType {
    const NAME: &'static str = "CmdTravelBrochureType";

    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<CmdTravelBrochureType> {
        match value {
            0 => ::std::option::Option::Some(CmdTravelBrochureType::CmdTravelBrochureNone),
            6438 => ::std::option::Option::Some(CmdTravelBrochureType::CmdTravelBrochureRemovePasterScRsp),
            6489 => ::std::option::Option::Some(CmdTravelBrochureType::CmdTravelBrochureGetPasterScNotify),
            6420 => ::std::option::Option::Some(CmdTravelBrochureType::CmdTravelBrochureSetPageDescStatusScRsp),
            6478 => ::std::option::Option::Some(CmdTravelBrochureType::CmdTravelBrochureUpdatePasterPosCsReq),
            6471 => ::std::option::Option::Some(CmdTravelBrochureType::CmdTravelBrochureGetDataScRsp),
            6477 => ::std::option::Option::Some(CmdTravelBrochureType::CmdTravelBrochureSelectMessageScRsp),
            6498 => ::std::option::Option::Some(CmdTravelBrochureType::CmdTravelBrochureGetDataCsReq),
            6466 => ::std::option::Option::Some(CmdTravelBrochureType::CmdTravelBrochureSetCustomValueCsReq),
            6450 => ::std::option::Option::Some(CmdTravelBrochureType::CmdTravelBrochurePageResetCsReq),
            6433 => ::std::option::Option::Some(CmdTravelBrochureType::CmdTravelBrochureApplyPasterCsReq),
            6404 => ::std::option::Option::Some(CmdTravelBrochureType::CmdTravelBrochureApplyPasterListCsReq),
            6412 => ::std::option::Option::Some(CmdTravelBrochureType::CmdTravelBrochureApplyPasterScRsp),
            6479 => ::std::option::Option::Some(CmdTravelBrochureType::CmdTravelBrochureSelectMessageCsReq),
            6460 => ::std::option::Option::Some(CmdTravelBrochureType::CmdTravelBrochureApplyPasterListScRsp),
            6428 => ::std::option::Option::Some(CmdTravelBrochureType::CmdTravelBrochureRemovePasterCsReq),
            6445 => ::std::option::Option::Some(CmdTravelBrochureType::CmdTravelBrochureSetPageDescStatusCsReq),
            6456 => ::std::option::Option::Some(CmdTravelBrochureType::CmdTravelBrochureUpdatePasterPosScRsp),
            6405 => ::std::option::Option::Some(CmdTravelBrochureType::CmdTravelBrochureSetCustomValueScRsp),
            6431 => ::std::option::Option::Some(CmdTravelBrochureType::CmdTravelBrochurePageResetScRsp),
            6483 => ::std::option::Option::Some(CmdTravelBrochureType::CmdTravelBrochurePageUnlockScNotify),
            _ => ::std::option::Option::None
        }
    }

    fn from_str(str: &str) -> ::std::option::Option<CmdTravelBrochureType> {
        match str {
            "CmdTravelBrochureNone" => ::std::option::Option::Some(CmdTravelBrochureType::CmdTravelBrochureNone),
            "CmdTravelBrochureRemovePasterScRsp" => ::std::option::Option::Some(CmdTravelBrochureType::CmdTravelBrochureRemovePasterScRsp),
            "CmdTravelBrochureGetPasterScNotify" => ::std::option::Option::Some(CmdTravelBrochureType::CmdTravelBrochureGetPasterScNotify),
            "CmdTravelBrochureSetPageDescStatusScRsp" => ::std::option::Option::Some(CmdTravelBrochureType::CmdTravelBrochureSetPageDescStatusScRsp),
            "CmdTravelBrochureUpdatePasterPosCsReq" => ::std::option::Option::Some(CmdTravelBrochureType::CmdTravelBrochureUpdatePasterPosCsReq),
            "CmdTravelBrochureGetDataScRsp" => ::std::option::Option::Some(CmdTravelBrochureType::CmdTravelBrochureGetDataScRsp),
            "CmdTravelBrochureSelectMessageScRsp" => ::std::option::Option::Some(CmdTravelBrochureType::CmdTravelBrochureSelectMessageScRsp),
            "CmdTravelBrochureGetDataCsReq" => ::std::option::Option::Some(CmdTravelBrochureType::CmdTravelBrochureGetDataCsReq),
            "CmdTravelBrochureSetCustomValueCsReq" => ::std::option::Option::Some(CmdTravelBrochureType::CmdTravelBrochureSetCustomValueCsReq),
            "CmdTravelBrochurePageResetCsReq" => ::std::option::Option::Some(CmdTravelBrochureType::CmdTravelBrochurePageResetCsReq),
            "CmdTravelBrochureApplyPasterCsReq" => ::std::option::Option::Some(CmdTravelBrochureType::CmdTravelBrochureApplyPasterCsReq),
            "CmdTravelBrochureApplyPasterListCsReq" => ::std::option::Option::Some(CmdTravelBrochureType::CmdTravelBrochureApplyPasterListCsReq),
            "CmdTravelBrochureApplyPasterScRsp" => ::std::option::Option::Some(CmdTravelBrochureType::CmdTravelBrochureApplyPasterScRsp),
            "CmdTravelBrochureSelectMessageCsReq" => ::std::option::Option::Some(CmdTravelBrochureType::CmdTravelBrochureSelectMessageCsReq),
            "CmdTravelBrochureApplyPasterListScRsp" => ::std::option::Option::Some(CmdTravelBrochureType::CmdTravelBrochureApplyPasterListScRsp),
            "CmdTravelBrochureRemovePasterCsReq" => ::std::option::Option::Some(CmdTravelBrochureType::CmdTravelBrochureRemovePasterCsReq),
            "CmdTravelBrochureSetPageDescStatusCsReq" => ::std::option::Option::Some(CmdTravelBrochureType::CmdTravelBrochureSetPageDescStatusCsReq),
            "CmdTravelBrochureUpdatePasterPosScRsp" => ::std::option::Option::Some(CmdTravelBrochureType::CmdTravelBrochureUpdatePasterPosScRsp),
            "CmdTravelBrochureSetCustomValueScRsp" => ::std::option::Option::Some(CmdTravelBrochureType::CmdTravelBrochureSetCustomValueScRsp),
            "CmdTravelBrochurePageResetScRsp" => ::std::option::Option::Some(CmdTravelBrochureType::CmdTravelBrochurePageResetScRsp),
            "CmdTravelBrochurePageUnlockScNotify" => ::std::option::Option::Some(CmdTravelBrochureType::CmdTravelBrochurePageUnlockScNotify),
            _ => ::std::option::Option::None
        }
    }

    const VALUES: &'static [CmdTravelBrochureType] = &[
        CmdTravelBrochureType::CmdTravelBrochureNone,
        CmdTravelBrochureType::CmdTravelBrochureRemovePasterScRsp,
        CmdTravelBrochureType::CmdTravelBrochureGetPasterScNotify,
        CmdTravelBrochureType::CmdTravelBrochureSetPageDescStatusScRsp,
        CmdTravelBrochureType::CmdTravelBrochureUpdatePasterPosCsReq,
        CmdTravelBrochureType::CmdTravelBrochureGetDataScRsp,
        CmdTravelBrochureType::CmdTravelBrochureSelectMessageScRsp,
        CmdTravelBrochureType::CmdTravelBrochureGetDataCsReq,
        CmdTravelBrochureType::CmdTravelBrochureSetCustomValueCsReq,
        CmdTravelBrochureType::CmdTravelBrochurePageResetCsReq,
        CmdTravelBrochureType::CmdTravelBrochureApplyPasterCsReq,
        CmdTravelBrochureType::CmdTravelBrochureApplyPasterListCsReq,
        CmdTravelBrochureType::CmdTravelBrochureApplyPasterScRsp,
        CmdTravelBrochureType::CmdTravelBrochureSelectMessageCsReq,
        CmdTravelBrochureType::CmdTravelBrochureApplyPasterListScRsp,
        CmdTravelBrochureType::CmdTravelBrochureRemovePasterCsReq,
        CmdTravelBrochureType::CmdTravelBrochureSetPageDescStatusCsReq,
        CmdTravelBrochureType::CmdTravelBrochureUpdatePasterPosScRsp,
        CmdTravelBrochureType::CmdTravelBrochureSetCustomValueScRsp,
        CmdTravelBrochureType::CmdTravelBrochurePageResetScRsp,
        CmdTravelBrochureType::CmdTravelBrochurePageUnlockScNotify,
    ];
}

impl ::protobuf::EnumFull for CmdTravelBrochureType {
    fn enum_descriptor() -> ::protobuf::reflect::EnumDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().enum_by_package_relative_name("CmdTravelBrochureType").unwrap()).clone()
    }

    fn descriptor(&self) -> ::protobuf::reflect::EnumValueDescriptor {
        let index = match self {
            CmdTravelBrochureType::CmdTravelBrochureNone => 0,
            CmdTravelBrochureType::CmdTravelBrochureRemovePasterScRsp => 1,
            CmdTravelBrochureType::CmdTravelBrochureGetPasterScNotify => 2,
            CmdTravelBrochureType::CmdTravelBrochureSetPageDescStatusScRsp => 3,
            CmdTravelBrochureType::CmdTravelBrochureUpdatePasterPosCsReq => 4,
            CmdTravelBrochureType::CmdTravelBrochureGetDataScRsp => 5,
            CmdTravelBrochureType::CmdTravelBrochureSelectMessageScRsp => 6,
            CmdTravelBrochureType::CmdTravelBrochureGetDataCsReq => 7,
            CmdTravelBrochureType::CmdTravelBrochureSetCustomValueCsReq => 8,
            CmdTravelBrochureType::CmdTravelBrochurePageResetCsReq => 9,
            CmdTravelBrochureType::CmdTravelBrochureApplyPasterCsReq => 10,
            CmdTravelBrochureType::CmdTravelBrochureApplyPasterListCsReq => 11,
            CmdTravelBrochureType::CmdTravelBrochureApplyPasterScRsp => 12,
            CmdTravelBrochureType::CmdTravelBrochureSelectMessageCsReq => 13,
            CmdTravelBrochureType::CmdTravelBrochureApplyPasterListScRsp => 14,
            CmdTravelBrochureType::CmdTravelBrochureRemovePasterCsReq => 15,
            CmdTravelBrochureType::CmdTravelBrochureSetPageDescStatusCsReq => 16,
            CmdTravelBrochureType::CmdTravelBrochureUpdatePasterPosScRsp => 17,
            CmdTravelBrochureType::CmdTravelBrochureSetCustomValueScRsp => 18,
            CmdTravelBrochureType::CmdTravelBrochurePageResetScRsp => 19,
            CmdTravelBrochureType::CmdTravelBrochurePageUnlockScNotify => 20,
        };
        Self::enum_descriptor().value_by_index(index)
    }
}

impl ::std::default::Default for CmdTravelBrochureType {
    fn default() -> Self {
        CmdTravelBrochureType::CmdTravelBrochureNone
    }
}

impl CmdTravelBrochureType {
    fn generated_enum_descriptor_data() -> ::protobuf::reflect::GeneratedEnumDescriptorData {
        ::protobuf::reflect::GeneratedEnumDescriptorData::new::<CmdTravelBrochureType>("CmdTravelBrochureType")
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x1bCmdTravelBrochureType.proto*\xf1\x06\n\x15CmdTravelBrochureType\
    \x12\x19\n\x15CmdTravelBrochureNone\x10\0\x12'\n\"CmdTravelBrochureRemov\
    ePasterScRsp\x10\xa62\x12'\n\"CmdTravelBrochureGetPasterScNotify\x10\xd9\
    2\x12,\n'CmdTravelBrochureSetPageDescStatusScRsp\x10\x942\x12*\n%CmdTrav\
    elBrochureUpdatePasterPosCsReq\x10\xce2\x12\"\n\x1dCmdTravelBrochureGetD\
    ataScRsp\x10\xc72\x12(\n#CmdTravelBrochureSelectMessageScRsp\x10\xcd2\
    \x12\"\n\x1dCmdTravelBrochureGetDataCsReq\x10\xe22\x12)\n$CmdTravelBroch\
    ureSetCustomValueCsReq\x10\xc22\x12$\n\x1fCmdTravelBrochurePageResetCsRe\
    q\x10\xb22\x12&\n!CmdTravelBrochureApplyPasterCsReq\x10\xa12\x12*\n%CmdT\
    ravelBrochureApplyPasterListCsReq\x10\x842\x12&\n!CmdTravelBrochureApply\
    PasterScRsp\x10\x8c2\x12(\n#CmdTravelBrochureSelectMessageCsReq\x10\xcf2\
    \x12*\n%CmdTravelBrochureApplyPasterListScRsp\x10\xbc2\x12'\n\"CmdTravel\
    BrochureRemovePasterCsReq\x10\x9c2\x12,\n'CmdTravelBrochureSetPageDescSt\
    atusCsReq\x10\xad2\x12*\n%CmdTravelBrochureUpdatePasterPosScRsp\x10\xb82\
    \x12)\n$CmdTravelBrochureSetCustomValueScRsp\x10\x852\x12$\n\x1fCmdTrave\
    lBrochurePageResetScRsp\x10\x9f2\x12(\n#CmdTravelBrochurePageUnlockScNot\
    ify\x10\xd32b\x06proto3\
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
            enums.push(CmdTravelBrochureType::generated_enum_descriptor_data());
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