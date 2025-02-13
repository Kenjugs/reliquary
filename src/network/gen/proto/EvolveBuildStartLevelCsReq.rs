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

//! Generated file from `EvolveBuildStartLevelCsReq.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:EvolveBuildStartLevelCsReq)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct EvolveBuildStartLevelCsReq {
    // message fields
    // @@protoc_insertion_point(field:EvolveBuildStartLevelCsReq.DGEJMCKIFJB)
    pub DGEJMCKIFJB: u32,
    // @@protoc_insertion_point(field:EvolveBuildStartLevelCsReq.avatar_list)
    pub avatar_list: ::std::vec::Vec<super::EvolveBuildAvatar::EvolveBuildAvatar>,
    // @@protoc_insertion_point(field:EvolveBuildStartLevelCsReq.JJLFIBAAJNJ)
    pub JJLFIBAAJNJ: ::protobuf::MessageField<super::NGIKGHKMAHA::NGIKGHKMAHA>,
    // special fields
    // @@protoc_insertion_point(special_field:EvolveBuildStartLevelCsReq.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a EvolveBuildStartLevelCsReq {
    fn default() -> &'a EvolveBuildStartLevelCsReq {
        <EvolveBuildStartLevelCsReq as ::protobuf::Message>::default_instance()
    }
}

impl EvolveBuildStartLevelCsReq {
    pub fn new() -> EvolveBuildStartLevelCsReq {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(3);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "DGEJMCKIFJB",
            |m: &EvolveBuildStartLevelCsReq| { &m.DGEJMCKIFJB },
            |m: &mut EvolveBuildStartLevelCsReq| { &mut m.DGEJMCKIFJB },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "avatar_list",
            |m: &EvolveBuildStartLevelCsReq| { &m.avatar_list },
            |m: &mut EvolveBuildStartLevelCsReq| { &mut m.avatar_list },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::NGIKGHKMAHA::NGIKGHKMAHA>(
            "JJLFIBAAJNJ",
            |m: &EvolveBuildStartLevelCsReq| { &m.JJLFIBAAJNJ },
            |m: &mut EvolveBuildStartLevelCsReq| { &mut m.JJLFIBAAJNJ },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<EvolveBuildStartLevelCsReq>(
            "EvolveBuildStartLevelCsReq",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for EvolveBuildStartLevelCsReq {
    const NAME: &'static str = "EvolveBuildStartLevelCsReq";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                32 => {
                    self.DGEJMCKIFJB = is.read_uint32()?;
                },
                106 => {
                    self.avatar_list.push(is.read_message()?);
                },
                98 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.JJLFIBAAJNJ)?;
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
        if self.DGEJMCKIFJB != 0 {
            my_size += ::protobuf::rt::uint32_size(4, self.DGEJMCKIFJB);
        }
        for value in &self.avatar_list {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        if let Some(v) = self.JJLFIBAAJNJ.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.DGEJMCKIFJB != 0 {
            os.write_uint32(4, self.DGEJMCKIFJB)?;
        }
        for v in &self.avatar_list {
            ::protobuf::rt::write_message_field_with_cached_size(13, v, os)?;
        };
        if let Some(v) = self.JJLFIBAAJNJ.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(12, v, os)?;
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

    fn new() -> EvolveBuildStartLevelCsReq {
        EvolveBuildStartLevelCsReq::new()
    }

    fn clear(&mut self) {
        self.DGEJMCKIFJB = 0;
        self.avatar_list.clear();
        self.JJLFIBAAJNJ.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static EvolveBuildStartLevelCsReq {
        static instance: EvolveBuildStartLevelCsReq = EvolveBuildStartLevelCsReq {
            DGEJMCKIFJB: 0,
            avatar_list: ::std::vec::Vec::new(),
            JJLFIBAAJNJ: ::protobuf::MessageField::none(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for EvolveBuildStartLevelCsReq {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("EvolveBuildStartLevelCsReq").unwrap()).clone()
    }
}

impl ::std::fmt::Display for EvolveBuildStartLevelCsReq {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for EvolveBuildStartLevelCsReq {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x20EvolveBuildStartLevelCsReq.proto\x1a\x17EvolveBuildAvatar.proto\
    \x1a\x11NGIKGHKMAHA.proto\"\xa3\x01\n\x1aEvolveBuildStartLevelCsReq\x12\
    \x20\n\x0bDGEJMCKIFJB\x18\x04\x20\x01(\rR\x0bDGEJMCKIFJB\x123\n\x0bavata\
    r_list\x18\r\x20\x03(\x0b2\x12.EvolveBuildAvatarR\navatarList\x12.\n\x0b\
    JJLFIBAAJNJ\x18\x0c\x20\x01(\x0b2\x0c.NGIKGHKMAHAR\x0bJJLFIBAAJNJb\x06pr\
    oto3\
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
            let mut deps = ::std::vec::Vec::with_capacity(2);
            deps.push(super::EvolveBuildAvatar::file_descriptor().clone());
            deps.push(super::NGIKGHKMAHA::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(EvolveBuildStartLevelCsReq::generated_message_descriptor_data());
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
