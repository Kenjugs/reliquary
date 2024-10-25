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

//! Generated file from `CancelCacheNotifyCsReq.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:CancelCacheNotifyCsReq)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct CancelCacheNotifyCsReq {
    // message fields
    // @@protoc_insertion_point(field:CancelCacheNotifyCsReq.GGEPGHPENFG)
    pub GGEPGHPENFG: ::std::vec::Vec<u32>,
    // @@protoc_insertion_point(field:CancelCacheNotifyCsReq.HJHCCJGMPBC)
    pub HJHCCJGMPBC: ::std::vec::Vec<::std::string::String>,
    // @@protoc_insertion_point(field:CancelCacheNotifyCsReq.slot)
    pub slot: ::protobuf::EnumOrUnknown<super::ACNLEMFFGLJ::ACNLEMFFGLJ>,
    // special fields
    // @@protoc_insertion_point(special_field:CancelCacheNotifyCsReq.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a CancelCacheNotifyCsReq {
    fn default() -> &'a CancelCacheNotifyCsReq {
        <CancelCacheNotifyCsReq as ::protobuf::Message>::default_instance()
    }
}

impl CancelCacheNotifyCsReq {
    pub fn new() -> CancelCacheNotifyCsReq {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(3);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "GGEPGHPENFG",
            |m: &CancelCacheNotifyCsReq| { &m.GGEPGHPENFG },
            |m: &mut CancelCacheNotifyCsReq| { &mut m.GGEPGHPENFG },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "HJHCCJGMPBC",
            |m: &CancelCacheNotifyCsReq| { &m.HJHCCJGMPBC },
            |m: &mut CancelCacheNotifyCsReq| { &mut m.HJHCCJGMPBC },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "slot",
            |m: &CancelCacheNotifyCsReq| { &m.slot },
            |m: &mut CancelCacheNotifyCsReq| { &mut m.slot },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<CancelCacheNotifyCsReq>(
            "CancelCacheNotifyCsReq",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for CancelCacheNotifyCsReq {
    const NAME: &'static str = "CancelCacheNotifyCsReq";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                34 => {
                    is.read_repeated_packed_uint32_into(&mut self.GGEPGHPENFG)?;
                },
                32 => {
                    self.GGEPGHPENFG.push(is.read_uint32()?);
                },
                74 => {
                    self.HJHCCJGMPBC.push(is.read_string()?);
                },
                56 => {
                    self.slot = is.read_enum_or_unknown()?;
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
        for value in &self.GGEPGHPENFG {
            my_size += ::protobuf::rt::uint32_size(4, *value);
        };
        for value in &self.HJHCCJGMPBC {
            my_size += ::protobuf::rt::string_size(9, &value);
        };
        if self.slot != ::protobuf::EnumOrUnknown::new(super::ACNLEMFFGLJ::ACNLEMFFGLJ::CACHE_NOTIFY_TYPE_NONE) {
            my_size += ::protobuf::rt::int32_size(7, self.slot.value());
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        for v in &self.GGEPGHPENFG {
            os.write_uint32(4, *v)?;
        };
        for v in &self.HJHCCJGMPBC {
            os.write_string(9, &v)?;
        };
        if self.slot != ::protobuf::EnumOrUnknown::new(super::ACNLEMFFGLJ::ACNLEMFFGLJ::CACHE_NOTIFY_TYPE_NONE) {
            os.write_enum(7, ::protobuf::EnumOrUnknown::value(&self.slot))?;
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

    fn new() -> CancelCacheNotifyCsReq {
        CancelCacheNotifyCsReq::new()
    }

    fn clear(&mut self) {
        self.GGEPGHPENFG.clear();
        self.HJHCCJGMPBC.clear();
        self.slot = ::protobuf::EnumOrUnknown::new(super::ACNLEMFFGLJ::ACNLEMFFGLJ::CACHE_NOTIFY_TYPE_NONE);
        self.special_fields.clear();
    }

    fn default_instance() -> &'static CancelCacheNotifyCsReq {
        static instance: CancelCacheNotifyCsReq = CancelCacheNotifyCsReq {
            GGEPGHPENFG: ::std::vec::Vec::new(),
            HJHCCJGMPBC: ::std::vec::Vec::new(),
            slot: ::protobuf::EnumOrUnknown::from_i32(0),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for CancelCacheNotifyCsReq {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("CancelCacheNotifyCsReq").unwrap()).clone()
    }
}

impl ::std::fmt::Display for CancelCacheNotifyCsReq {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CancelCacheNotifyCsReq {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x1cCancelCacheNotifyCsReq.proto\x1a\x11ACNLEMFFGLJ.proto\"~\n\x16Canc\
    elCacheNotifyCsReq\x12\x20\n\x0bGGEPGHPENFG\x18\x04\x20\x03(\rR\x0bGGEPG\
    HPENFG\x12\x20\n\x0bHJHCCJGMPBC\x18\t\x20\x03(\tR\x0bHJHCCJGMPBC\x12\x20\
    \n\x04slot\x18\x07\x20\x01(\x0e2\x0c.ACNLEMFFGLJR\x04slotb\x06proto3\
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
            let mut deps = ::std::vec::Vec::with_capacity(1);
            deps.push(super::ACNLEMFFGLJ::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(CancelCacheNotifyCsReq::generated_message_descriptor_data());
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