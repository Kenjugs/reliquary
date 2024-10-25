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

//! Generated file from `AEBKGEAGJCJ.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:AEBKGEAGJCJ)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct AEBKGEAGJCJ {
    // message fields
    // @@protoc_insertion_point(field:AEBKGEAGJCJ.HJFIDJAHMJG)
    pub HJFIDJAHMJG: ::std::vec::Vec<super::HGGCNNJOEPH::HGGCNNJOEPH>,
    // @@protoc_insertion_point(field:AEBKGEAGJCJ.FILMNFDLMNI)
    pub FILMNFDLMNI: ::std::vec::Vec<super::OEDMPCBKCJD::OEDMPCBKCJD>,
    // @@protoc_insertion_point(field:AEBKGEAGJCJ.PJIMDPOODDJ)
    pub PJIMDPOODDJ: ::std::vec::Vec<u32>,
    // @@protoc_insertion_point(field:AEBKGEAGJCJ.PDBOKHNHPNA)
    pub PDBOKHNHPNA: ::std::vec::Vec<super::LogisticsScore::LogisticsScore>,
    // special fields
    // @@protoc_insertion_point(special_field:AEBKGEAGJCJ.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a AEBKGEAGJCJ {
    fn default() -> &'a AEBKGEAGJCJ {
        <AEBKGEAGJCJ as ::protobuf::Message>::default_instance()
    }
}

impl AEBKGEAGJCJ {
    pub fn new() -> AEBKGEAGJCJ {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(4);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "HJFIDJAHMJG",
            |m: &AEBKGEAGJCJ| { &m.HJFIDJAHMJG },
            |m: &mut AEBKGEAGJCJ| { &mut m.HJFIDJAHMJG },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "FILMNFDLMNI",
            |m: &AEBKGEAGJCJ| { &m.FILMNFDLMNI },
            |m: &mut AEBKGEAGJCJ| { &mut m.FILMNFDLMNI },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "PJIMDPOODDJ",
            |m: &AEBKGEAGJCJ| { &m.PJIMDPOODDJ },
            |m: &mut AEBKGEAGJCJ| { &mut m.PJIMDPOODDJ },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "PDBOKHNHPNA",
            |m: &AEBKGEAGJCJ| { &m.PDBOKHNHPNA },
            |m: &mut AEBKGEAGJCJ| { &mut m.PDBOKHNHPNA },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<AEBKGEAGJCJ>(
            "AEBKGEAGJCJ",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for AEBKGEAGJCJ {
    const NAME: &'static str = "AEBKGEAGJCJ";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                114 => {
                    self.HJFIDJAHMJG.push(is.read_message()?);
                },
                74 => {
                    self.FILMNFDLMNI.push(is.read_message()?);
                },
                106 => {
                    is.read_repeated_packed_uint32_into(&mut self.PJIMDPOODDJ)?;
                },
                104 => {
                    self.PJIMDPOODDJ.push(is.read_uint32()?);
                },
                90 => {
                    self.PDBOKHNHPNA.push(is.read_message()?);
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
        for value in &self.HJFIDJAHMJG {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        for value in &self.FILMNFDLMNI {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        for value in &self.PJIMDPOODDJ {
            my_size += ::protobuf::rt::uint32_size(13, *value);
        };
        for value in &self.PDBOKHNHPNA {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        for v in &self.HJFIDJAHMJG {
            ::protobuf::rt::write_message_field_with_cached_size(14, v, os)?;
        };
        for v in &self.FILMNFDLMNI {
            ::protobuf::rt::write_message_field_with_cached_size(9, v, os)?;
        };
        for v in &self.PJIMDPOODDJ {
            os.write_uint32(13, *v)?;
        };
        for v in &self.PDBOKHNHPNA {
            ::protobuf::rt::write_message_field_with_cached_size(11, v, os)?;
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

    fn new() -> AEBKGEAGJCJ {
        AEBKGEAGJCJ::new()
    }

    fn clear(&mut self) {
        self.HJFIDJAHMJG.clear();
        self.FILMNFDLMNI.clear();
        self.PJIMDPOODDJ.clear();
        self.PDBOKHNHPNA.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static AEBKGEAGJCJ {
        static instance: AEBKGEAGJCJ = AEBKGEAGJCJ {
            HJFIDJAHMJG: ::std::vec::Vec::new(),
            FILMNFDLMNI: ::std::vec::Vec::new(),
            PJIMDPOODDJ: ::std::vec::Vec::new(),
            PDBOKHNHPNA: ::std::vec::Vec::new(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for AEBKGEAGJCJ {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("AEBKGEAGJCJ").unwrap()).clone()
    }
}

impl ::std::fmt::Display for AEBKGEAGJCJ {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for AEBKGEAGJCJ {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11AEBKGEAGJCJ.proto\x1a\x11HGGCNNJOEPH.proto\x1a\x14LogisticsScore.p\
    roto\x1a\x11OEDMPCBKCJD.proto\"\xc2\x01\n\x0bAEBKGEAGJCJ\x12.\n\x0bHJFID\
    JAHMJG\x18\x0e\x20\x03(\x0b2\x0c.HGGCNNJOEPHR\x0bHJFIDJAHMJG\x12.\n\x0bF\
    ILMNFDLMNI\x18\t\x20\x03(\x0b2\x0c.OEDMPCBKCJDR\x0bFILMNFDLMNI\x12\x20\n\
    \x0bPJIMDPOODDJ\x18\r\x20\x03(\rR\x0bPJIMDPOODDJ\x121\n\x0bPDBOKHNHPNA\
    \x18\x0b\x20\x03(\x0b2\x0f.LogisticsScoreR\x0bPDBOKHNHPNAb\x06proto3\
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
            deps.push(super::HGGCNNJOEPH::file_descriptor().clone());
            deps.push(super::LogisticsScore::file_descriptor().clone());
            deps.push(super::OEDMPCBKCJD::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(AEBKGEAGJCJ::generated_message_descriptor_data());
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
