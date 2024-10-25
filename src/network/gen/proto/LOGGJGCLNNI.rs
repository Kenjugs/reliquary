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

//! Generated file from `LOGGJGCLNNI.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:LOGGJGCLNNI)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct LOGGJGCLNNI {
    // message fields
    // @@protoc_insertion_point(field:LOGGJGCLNNI.id)
    pub id: u32,
    // @@protoc_insertion_point(field:LOGGJGCLNNI.CMBBJPPMHPM)
    pub CMBBJPPMHPM: u32,
    // @@protoc_insertion_point(field:LOGGJGCLNNI.promotion)
    pub promotion: u32,
    // @@protoc_insertion_point(field:LOGGJGCLNNI.AAMLBFINHON)
    pub AAMLBFINHON: ::std::vec::Vec<u32>,
    // @@protoc_insertion_point(field:LOGGJGCLNNI.FMIPGNJJABN)
    pub FMIPGNJJABN: ::protobuf::EnumOrUnknown<super::EDKMNBHJOAM::EDKMNBHJOAM>,
    // @@protoc_insertion_point(field:LOGGJGCLNNI.LHLDCFOMIIK)
    pub LHLDCFOMIIK: ::protobuf::MessageField<super::MBEGINNAINL::MBEGINNAINL>,
    // special fields
    // @@protoc_insertion_point(special_field:LOGGJGCLNNI.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a LOGGJGCLNNI {
    fn default() -> &'a LOGGJGCLNNI {
        <LOGGJGCLNNI as ::protobuf::Message>::default_instance()
    }
}

impl LOGGJGCLNNI {
    pub fn new() -> LOGGJGCLNNI {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(6);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "id",
            |m: &LOGGJGCLNNI| { &m.id },
            |m: &mut LOGGJGCLNNI| { &mut m.id },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "CMBBJPPMHPM",
            |m: &LOGGJGCLNNI| { &m.CMBBJPPMHPM },
            |m: &mut LOGGJGCLNNI| { &mut m.CMBBJPPMHPM },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "promotion",
            |m: &LOGGJGCLNNI| { &m.promotion },
            |m: &mut LOGGJGCLNNI| { &mut m.promotion },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "AAMLBFINHON",
            |m: &LOGGJGCLNNI| { &m.AAMLBFINHON },
            |m: &mut LOGGJGCLNNI| { &mut m.AAMLBFINHON },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "FMIPGNJJABN",
            |m: &LOGGJGCLNNI| { &m.FMIPGNJJABN },
            |m: &mut LOGGJGCLNNI| { &mut m.FMIPGNJJABN },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::MBEGINNAINL::MBEGINNAINL>(
            "LHLDCFOMIIK",
            |m: &LOGGJGCLNNI| { &m.LHLDCFOMIIK },
            |m: &mut LOGGJGCLNNI| { &mut m.LHLDCFOMIIK },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<LOGGJGCLNNI>(
            "LOGGJGCLNNI",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for LOGGJGCLNNI {
    const NAME: &'static str = "LOGGJGCLNNI";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                8 => {
                    self.id = is.read_uint32()?;
                },
                16 => {
                    self.CMBBJPPMHPM = is.read_uint32()?;
                },
                24 => {
                    self.promotion = is.read_uint32()?;
                },
                34 => {
                    is.read_repeated_packed_uint32_into(&mut self.AAMLBFINHON)?;
                },
                32 => {
                    self.AAMLBFINHON.push(is.read_uint32()?);
                },
                40 => {
                    self.FMIPGNJJABN = is.read_enum_or_unknown()?;
                },
                50 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.LHLDCFOMIIK)?;
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
        if self.id != 0 {
            my_size += ::protobuf::rt::uint32_size(1, self.id);
        }
        if self.CMBBJPPMHPM != 0 {
            my_size += ::protobuf::rt::uint32_size(2, self.CMBBJPPMHPM);
        }
        if self.promotion != 0 {
            my_size += ::protobuf::rt::uint32_size(3, self.promotion);
        }
        for value in &self.AAMLBFINHON {
            my_size += ::protobuf::rt::uint32_size(4, *value);
        };
        if self.FMIPGNJJABN != ::protobuf::EnumOrUnknown::new(super::EDKMNBHJOAM::EDKMNBHJOAM::AETHERDIVIDE_SPIRIT_LINEUP_NONE) {
            my_size += ::protobuf::rt::int32_size(5, self.FMIPGNJJABN.value());
        }
        if let Some(v) = self.LHLDCFOMIIK.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.id != 0 {
            os.write_uint32(1, self.id)?;
        }
        if self.CMBBJPPMHPM != 0 {
            os.write_uint32(2, self.CMBBJPPMHPM)?;
        }
        if self.promotion != 0 {
            os.write_uint32(3, self.promotion)?;
        }
        for v in &self.AAMLBFINHON {
            os.write_uint32(4, *v)?;
        };
        if self.FMIPGNJJABN != ::protobuf::EnumOrUnknown::new(super::EDKMNBHJOAM::EDKMNBHJOAM::AETHERDIVIDE_SPIRIT_LINEUP_NONE) {
            os.write_enum(5, ::protobuf::EnumOrUnknown::value(&self.FMIPGNJJABN))?;
        }
        if let Some(v) = self.LHLDCFOMIIK.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(6, v, os)?;
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

    fn new() -> LOGGJGCLNNI {
        LOGGJGCLNNI::new()
    }

    fn clear(&mut self) {
        self.id = 0;
        self.CMBBJPPMHPM = 0;
        self.promotion = 0;
        self.AAMLBFINHON.clear();
        self.FMIPGNJJABN = ::protobuf::EnumOrUnknown::new(super::EDKMNBHJOAM::EDKMNBHJOAM::AETHERDIVIDE_SPIRIT_LINEUP_NONE);
        self.LHLDCFOMIIK.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static LOGGJGCLNNI {
        static instance: LOGGJGCLNNI = LOGGJGCLNNI {
            id: 0,
            CMBBJPPMHPM: 0,
            promotion: 0,
            AAMLBFINHON: ::std::vec::Vec::new(),
            FMIPGNJJABN: ::protobuf::EnumOrUnknown::from_i32(0),
            LHLDCFOMIIK: ::protobuf::MessageField::none(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for LOGGJGCLNNI {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("LOGGJGCLNNI").unwrap()).clone()
    }
}

impl ::std::fmt::Display for LOGGJGCLNNI {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for LOGGJGCLNNI {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11LOGGJGCLNNI.proto\x1a\x11EDKMNBHJOAM.proto\x1a\x11MBEGINNAINL.prot\
    o\"\xdf\x01\n\x0bLOGGJGCLNNI\x12\x0e\n\x02id\x18\x01\x20\x01(\rR\x02id\
    \x12\x20\n\x0bCMBBJPPMHPM\x18\x02\x20\x01(\rR\x0bCMBBJPPMHPM\x12\x1c\n\t\
    promotion\x18\x03\x20\x01(\rR\tpromotion\x12\x20\n\x0bAAMLBFINHON\x18\
    \x04\x20\x03(\rR\x0bAAMLBFINHON\x12.\n\x0bFMIPGNJJABN\x18\x05\x20\x01(\
    \x0e2\x0c.EDKMNBHJOAMR\x0bFMIPGNJJABN\x12.\n\x0bLHLDCFOMIIK\x18\x06\x20\
    \x01(\x0b2\x0c.MBEGINNAINLR\x0bLHLDCFOMIIKb\x06proto3\
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
            deps.push(super::EDKMNBHJOAM::file_descriptor().clone());
            deps.push(super::MBEGINNAINL::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(LOGGJGCLNNI::generated_message_descriptor_data());
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
