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

//! Generated file from `NPFBNEDBOPE.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:NPFBNEDBOPE)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct NPFBNEDBOPE {
    // message fields
    // @@protoc_insertion_point(field:NPFBNEDBOPE.OCFMLFJEHHF)
    pub OCFMLFJEHHF: ::std::vec::Vec<u32>,
    // @@protoc_insertion_point(field:NPFBNEDBOPE.JGGBHMKOADA)
    pub JGGBHMKOADA: ::std::vec::Vec<u32>,
    // @@protoc_insertion_point(field:NPFBNEDBOPE.BIJCOIGGOJP)
    pub BIJCOIGGOJP: ::std::vec::Vec<super::LKJEHEOKIIF::LKJEHEOKIIF>,
    // @@protoc_insertion_point(field:NPFBNEDBOPE.ANJMGEHLOHE)
    pub ANJMGEHLOHE: ::std::vec::Vec<u32>,
    // @@protoc_insertion_point(field:NPFBNEDBOPE.MIPNLPEAODE)
    pub MIPNLPEAODE: ::std::vec::Vec<super::CFMNHAJOBHE::CFMNHAJOBHE>,
    // special fields
    // @@protoc_insertion_point(special_field:NPFBNEDBOPE.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a NPFBNEDBOPE {
    fn default() -> &'a NPFBNEDBOPE {
        <NPFBNEDBOPE as ::protobuf::Message>::default_instance()
    }
}

impl NPFBNEDBOPE {
    pub fn new() -> NPFBNEDBOPE {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(5);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "OCFMLFJEHHF",
            |m: &NPFBNEDBOPE| { &m.OCFMLFJEHHF },
            |m: &mut NPFBNEDBOPE| { &mut m.OCFMLFJEHHF },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "JGGBHMKOADA",
            |m: &NPFBNEDBOPE| { &m.JGGBHMKOADA },
            |m: &mut NPFBNEDBOPE| { &mut m.JGGBHMKOADA },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "BIJCOIGGOJP",
            |m: &NPFBNEDBOPE| { &m.BIJCOIGGOJP },
            |m: &mut NPFBNEDBOPE| { &mut m.BIJCOIGGOJP },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "ANJMGEHLOHE",
            |m: &NPFBNEDBOPE| { &m.ANJMGEHLOHE },
            |m: &mut NPFBNEDBOPE| { &mut m.ANJMGEHLOHE },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "MIPNLPEAODE",
            |m: &NPFBNEDBOPE| { &m.MIPNLPEAODE },
            |m: &mut NPFBNEDBOPE| { &mut m.MIPNLPEAODE },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<NPFBNEDBOPE>(
            "NPFBNEDBOPE",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for NPFBNEDBOPE {
    const NAME: &'static str = "NPFBNEDBOPE";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                82 => {
                    is.read_repeated_packed_uint32_into(&mut self.OCFMLFJEHHF)?;
                },
                80 => {
                    self.OCFMLFJEHHF.push(is.read_uint32()?);
                },
                74 => {
                    is.read_repeated_packed_uint32_into(&mut self.JGGBHMKOADA)?;
                },
                72 => {
                    self.JGGBHMKOADA.push(is.read_uint32()?);
                },
                18 => {
                    self.BIJCOIGGOJP.push(is.read_message()?);
                },
                58 => {
                    is.read_repeated_packed_uint32_into(&mut self.ANJMGEHLOHE)?;
                },
                56 => {
                    self.ANJMGEHLOHE.push(is.read_uint32()?);
                },
                106 => {
                    self.MIPNLPEAODE.push(is.read_message()?);
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
        for value in &self.OCFMLFJEHHF {
            my_size += ::protobuf::rt::uint32_size(10, *value);
        };
        for value in &self.JGGBHMKOADA {
            my_size += ::protobuf::rt::uint32_size(9, *value);
        };
        for value in &self.BIJCOIGGOJP {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        for value in &self.ANJMGEHLOHE {
            my_size += ::protobuf::rt::uint32_size(7, *value);
        };
        for value in &self.MIPNLPEAODE {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        for v in &self.OCFMLFJEHHF {
            os.write_uint32(10, *v)?;
        };
        for v in &self.JGGBHMKOADA {
            os.write_uint32(9, *v)?;
        };
        for v in &self.BIJCOIGGOJP {
            ::protobuf::rt::write_message_field_with_cached_size(2, v, os)?;
        };
        for v in &self.ANJMGEHLOHE {
            os.write_uint32(7, *v)?;
        };
        for v in &self.MIPNLPEAODE {
            ::protobuf::rt::write_message_field_with_cached_size(13, v, os)?;
        };
        os.write_unknown_fields(self.special_fields.unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn special_fields(&self) -> &::protobuf::SpecialFields {
        &self.special_fields
    }

    fn mut_special_fields(&mut self) -> &mut ::protobuf::SpecialFields {
        &mut self.special_fields
    }

    fn new() -> NPFBNEDBOPE {
        NPFBNEDBOPE::new()
    }

    fn clear(&mut self) {
        self.OCFMLFJEHHF.clear();
        self.JGGBHMKOADA.clear();
        self.BIJCOIGGOJP.clear();
        self.ANJMGEHLOHE.clear();
        self.MIPNLPEAODE.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static NPFBNEDBOPE {
        static instance: NPFBNEDBOPE = NPFBNEDBOPE {
            OCFMLFJEHHF: ::std::vec::Vec::new(),
            JGGBHMKOADA: ::std::vec::Vec::new(),
            BIJCOIGGOJP: ::std::vec::Vec::new(),
            ANJMGEHLOHE: ::std::vec::Vec::new(),
            MIPNLPEAODE: ::std::vec::Vec::new(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for NPFBNEDBOPE {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("NPFBNEDBOPE").unwrap()).clone()
    }
}

impl ::std::fmt::Display for NPFBNEDBOPE {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for NPFBNEDBOPE {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11NPFBNEDBOPE.proto\x1a\x11CFMNHAJOBHE.proto\x1a\x11LKJEHEOKIIF.prot\
    o\"\xd3\x01\n\x0bNPFBNEDBOPE\x12\x20\n\x0bOCFMLFJEHHF\x18\n\x20\x03(\rR\
    \x0bOCFMLFJEHHF\x12\x20\n\x0bJGGBHMKOADA\x18\t\x20\x03(\rR\x0bJGGBHMKOAD\
    A\x12.\n\x0bBIJCOIGGOJP\x18\x02\x20\x03(\x0b2\x0c.LKJEHEOKIIFR\x0bBIJCOI\
    GGOJP\x12\x20\n\x0bANJMGEHLOHE\x18\x07\x20\x03(\rR\x0bANJMGEHLOHE\x12.\n\
    \x0bMIPNLPEAODE\x18\r\x20\x03(\x0b2\x0c.CFMNHAJOBHER\x0bMIPNLPEAODEb\x06\
    proto3\
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
            deps.push(super::CFMNHAJOBHE::file_descriptor().clone());
            deps.push(super::LKJEHEOKIIF::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(NPFBNEDBOPE::generated_message_descriptor_data());
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
