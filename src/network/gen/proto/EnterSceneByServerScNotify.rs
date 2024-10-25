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

//! Generated file from `EnterSceneByServerScNotify.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:EnterSceneByServerScNotify)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct EnterSceneByServerScNotify {
    // message fields
    // @@protoc_insertion_point(field:EnterSceneByServerScNotify.EMDECAJPAPM)
    pub EMDECAJPAPM: ::protobuf::MessageField<super::FHGPCKGFGAO::FHGPCKGFGAO>,
    // @@protoc_insertion_point(field:EnterSceneByServerScNotify.COCFMLGGMKE)
    pub COCFMLGGMKE: ::protobuf::MessageField<super::FJPJJEIJLLP::FJPJJEIJLLP>,
    // @@protoc_insertion_point(field:EnterSceneByServerScNotify.DGDDHBLKMLI)
    pub DGDDHBLKMLI: ::protobuf::EnumOrUnknown<super::EENBNADCEML::EENBNADCEML>,
    // special fields
    // @@protoc_insertion_point(special_field:EnterSceneByServerScNotify.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a EnterSceneByServerScNotify {
    fn default() -> &'a EnterSceneByServerScNotify {
        <EnterSceneByServerScNotify as ::protobuf::Message>::default_instance()
    }
}

impl EnterSceneByServerScNotify {
    pub fn new() -> EnterSceneByServerScNotify {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(3);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::FHGPCKGFGAO::FHGPCKGFGAO>(
            "EMDECAJPAPM",
            |m: &EnterSceneByServerScNotify| { &m.EMDECAJPAPM },
            |m: &mut EnterSceneByServerScNotify| { &mut m.EMDECAJPAPM },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::FJPJJEIJLLP::FJPJJEIJLLP>(
            "COCFMLGGMKE",
            |m: &EnterSceneByServerScNotify| { &m.COCFMLGGMKE },
            |m: &mut EnterSceneByServerScNotify| { &mut m.COCFMLGGMKE },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "DGDDHBLKMLI",
            |m: &EnterSceneByServerScNotify| { &m.DGDDHBLKMLI },
            |m: &mut EnterSceneByServerScNotify| { &mut m.DGDDHBLKMLI },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<EnterSceneByServerScNotify>(
            "EnterSceneByServerScNotify",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for EnterSceneByServerScNotify {
    const NAME: &'static str = "EnterSceneByServerScNotify";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                106 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.EMDECAJPAPM)?;
                },
                74 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.COCFMLGGMKE)?;
                },
                96 => {
                    self.DGDDHBLKMLI = is.read_enum_or_unknown()?;
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
        if let Some(v) = self.EMDECAJPAPM.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if let Some(v) = self.COCFMLGGMKE.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if self.DGDDHBLKMLI != ::protobuf::EnumOrUnknown::new(super::EENBNADCEML::EENBNADCEML::ENTER_SCENE_REASON_NONE) {
            my_size += ::protobuf::rt::int32_size(12, self.DGDDHBLKMLI.value());
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if let Some(v) = self.EMDECAJPAPM.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(13, v, os)?;
        }
        if let Some(v) = self.COCFMLGGMKE.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(9, v, os)?;
        }
        if self.DGDDHBLKMLI != ::protobuf::EnumOrUnknown::new(super::EENBNADCEML::EENBNADCEML::ENTER_SCENE_REASON_NONE) {
            os.write_enum(12, ::protobuf::EnumOrUnknown::value(&self.DGDDHBLKMLI))?;
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

    fn new() -> EnterSceneByServerScNotify {
        EnterSceneByServerScNotify::new()
    }

    fn clear(&mut self) {
        self.EMDECAJPAPM.clear();
        self.COCFMLGGMKE.clear();
        self.DGDDHBLKMLI = ::protobuf::EnumOrUnknown::new(super::EENBNADCEML::EENBNADCEML::ENTER_SCENE_REASON_NONE);
        self.special_fields.clear();
    }

    fn default_instance() -> &'static EnterSceneByServerScNotify {
        static instance: EnterSceneByServerScNotify = EnterSceneByServerScNotify {
            EMDECAJPAPM: ::protobuf::MessageField::none(),
            COCFMLGGMKE: ::protobuf::MessageField::none(),
            DGDDHBLKMLI: ::protobuf::EnumOrUnknown::from_i32(0),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for EnterSceneByServerScNotify {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("EnterSceneByServerScNotify").unwrap()).clone()
    }
}

impl ::std::fmt::Display for EnterSceneByServerScNotify {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for EnterSceneByServerScNotify {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x20EnterSceneByServerScNotify.proto\x1a\x11EENBNADCEML.proto\x1a\x11F\
    HGPCKGFGAO.proto\x1a\x11FJPJJEIJLLP.proto\"\xac\x01\n\x1aEnterSceneBySer\
    verScNotify\x12.\n\x0bEMDECAJPAPM\x18\r\x20\x01(\x0b2\x0c.FHGPCKGFGAOR\
    \x0bEMDECAJPAPM\x12.\n\x0bCOCFMLGGMKE\x18\t\x20\x01(\x0b2\x0c.FJPJJEIJLL\
    PR\x0bCOCFMLGGMKE\x12.\n\x0bDGDDHBLKMLI\x18\x0c\x20\x01(\x0e2\x0c.EENBNA\
    DCEMLR\x0bDGDDHBLKMLIb\x06proto3\
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
            let mut deps = ::std::vec::Vec::with_capacity(3);
            deps.push(super::EENBNADCEML::file_descriptor().clone());
            deps.push(super::FHGPCKGFGAO::file_descriptor().clone());
            deps.push(super::FJPJJEIJLLP::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(EnterSceneByServerScNotify::generated_message_descriptor_data());
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
