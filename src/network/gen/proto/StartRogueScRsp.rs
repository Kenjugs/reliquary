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

//! Generated file from `StartRogueScRsp.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:StartRogueScRsp)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct StartRogueScRsp {
    // message fields
    // @@protoc_insertion_point(field:StartRogueScRsp.COCFMLGGMKE)
    pub COCFMLGGMKE: ::protobuf::MessageField<super::FJPJJEIJLLP::FJPJJEIJLLP>,
    // @@protoc_insertion_point(field:StartRogueScRsp.NPFLJLEEFFI)
    pub NPFLJLEEFFI: ::protobuf::MessageField<super::MGCIMEKACFE::MGCIMEKACFE>,
    // @@protoc_insertion_point(field:StartRogueScRsp.retcode)
    pub retcode: u32,
    // @@protoc_insertion_point(field:StartRogueScRsp.EMDECAJPAPM)
    pub EMDECAJPAPM: ::protobuf::MessageField<super::FHGPCKGFGAO::FHGPCKGFGAO>,
    // @@protoc_insertion_point(field:StartRogueScRsp.GOHKAOJCIDM)
    pub GOHKAOJCIDM: ::protobuf::MessageField<super::BMIEOHPLAKP::BMIEOHPLAKP>,
    // special fields
    // @@protoc_insertion_point(special_field:StartRogueScRsp.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a StartRogueScRsp {
    fn default() -> &'a StartRogueScRsp {
        <StartRogueScRsp as ::protobuf::Message>::default_instance()
    }
}

impl StartRogueScRsp {
    pub fn new() -> StartRogueScRsp {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(5);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::FJPJJEIJLLP::FJPJJEIJLLP>(
            "COCFMLGGMKE",
            |m: &StartRogueScRsp| { &m.COCFMLGGMKE },
            |m: &mut StartRogueScRsp| { &mut m.COCFMLGGMKE },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::MGCIMEKACFE::MGCIMEKACFE>(
            "NPFLJLEEFFI",
            |m: &StartRogueScRsp| { &m.NPFLJLEEFFI },
            |m: &mut StartRogueScRsp| { &mut m.NPFLJLEEFFI },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "retcode",
            |m: &StartRogueScRsp| { &m.retcode },
            |m: &mut StartRogueScRsp| { &mut m.retcode },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::FHGPCKGFGAO::FHGPCKGFGAO>(
            "EMDECAJPAPM",
            |m: &StartRogueScRsp| { &m.EMDECAJPAPM },
            |m: &mut StartRogueScRsp| { &mut m.EMDECAJPAPM },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::BMIEOHPLAKP::BMIEOHPLAKP>(
            "GOHKAOJCIDM",
            |m: &StartRogueScRsp| { &m.GOHKAOJCIDM },
            |m: &mut StartRogueScRsp| { &mut m.GOHKAOJCIDM },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<StartRogueScRsp>(
            "StartRogueScRsp",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for StartRogueScRsp {
    const NAME: &'static str = "StartRogueScRsp";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                90 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.COCFMLGGMKE)?;
                },
                114 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.NPFLJLEEFFI)?;
                },
                32 => {
                    self.retcode = is.read_uint32()?;
                },
                50 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.EMDECAJPAPM)?;
                },
                122 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.GOHKAOJCIDM)?;
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
        if let Some(v) = self.COCFMLGGMKE.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if let Some(v) = self.NPFLJLEEFFI.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if self.retcode != 0 {
            my_size += ::protobuf::rt::uint32_size(4, self.retcode);
        }
        if let Some(v) = self.EMDECAJPAPM.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if let Some(v) = self.GOHKAOJCIDM.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if let Some(v) = self.COCFMLGGMKE.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(11, v, os)?;
        }
        if let Some(v) = self.NPFLJLEEFFI.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(14, v, os)?;
        }
        if self.retcode != 0 {
            os.write_uint32(4, self.retcode)?;
        }
        if let Some(v) = self.EMDECAJPAPM.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(6, v, os)?;
        }
        if let Some(v) = self.GOHKAOJCIDM.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(15, v, os)?;
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

    fn new() -> StartRogueScRsp {
        StartRogueScRsp::new()
    }

    fn clear(&mut self) {
        self.COCFMLGGMKE.clear();
        self.NPFLJLEEFFI.clear();
        self.retcode = 0;
        self.EMDECAJPAPM.clear();
        self.GOHKAOJCIDM.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static StartRogueScRsp {
        static instance: StartRogueScRsp = StartRogueScRsp {
            COCFMLGGMKE: ::protobuf::MessageField::none(),
            NPFLJLEEFFI: ::protobuf::MessageField::none(),
            retcode: 0,
            EMDECAJPAPM: ::protobuf::MessageField::none(),
            GOHKAOJCIDM: ::protobuf::MessageField::none(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for StartRogueScRsp {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("StartRogueScRsp").unwrap()).clone()
    }
}

impl ::std::fmt::Display for StartRogueScRsp {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for StartRogueScRsp {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x15StartRogueScRsp.proto\x1a\x11BMIEOHPLAKP.proto\x1a\x11FHGPCKGFGAO.\
    proto\x1a\x11FJPJJEIJLLP.proto\x1a\x11MGCIMEKACFE.proto\"\xeb\x01\n\x0fS\
    tartRogueScRsp\x12.\n\x0bCOCFMLGGMKE\x18\x0b\x20\x01(\x0b2\x0c.FJPJJEIJL\
    LPR\x0bCOCFMLGGMKE\x12.\n\x0bNPFLJLEEFFI\x18\x0e\x20\x01(\x0b2\x0c.MGCIM\
    EKACFER\x0bNPFLJLEEFFI\x12\x18\n\x07retcode\x18\x04\x20\x01(\rR\x07retco\
    de\x12.\n\x0bEMDECAJPAPM\x18\x06\x20\x01(\x0b2\x0c.FHGPCKGFGAOR\x0bEMDEC\
    AJPAPM\x12.\n\x0bGOHKAOJCIDM\x18\x0f\x20\x01(\x0b2\x0c.BMIEOHPLAKPR\x0bG\
    OHKAOJCIDMb\x06proto3\
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
            let mut deps = ::std::vec::Vec::with_capacity(4);
            deps.push(super::BMIEOHPLAKP::file_descriptor().clone());
            deps.push(super::FHGPCKGFGAO::file_descriptor().clone());
            deps.push(super::FJPJJEIJLLP::file_descriptor().clone());
            deps.push(super::MGCIMEKACFE::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(StartRogueScRsp::generated_message_descriptor_data());
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
