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

//! Generated file from `PDCACCGOMJB.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:PDCACCGOMJB)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct PDCACCGOMJB {
    // message fields
    // @@protoc_insertion_point(field:PDCACCGOMJB.KLMOOMBPHIO)
    pub KLMOOMBPHIO: ::std::string::String,
    // @@protoc_insertion_point(field:PDCACCGOMJB.OEOKBLIOGCB)
    pub OEOKBLIOGCB: ::std::string::String,
    // @@protoc_insertion_point(field:PDCACCGOMJB.DCNFJNLJEDO)
    pub DCNFJNLJEDO: ::std::string::String,
    // @@protoc_insertion_point(field:PDCACCGOMJB.GBNOPNPEAOF)
    pub GBNOPNPEAOF: ::std::string::String,
    // @@protoc_insertion_point(field:PDCACCGOMJB.AMOIBMJGKNN)
    pub AMOIBMJGKNN: ::std::string::String,
    // @@protoc_insertion_point(field:PDCACCGOMJB.LLCMICAEFGM)
    pub LLCMICAEFGM: ::std::string::String,
    // @@protoc_insertion_point(field:PDCACCGOMJB.MAC)
    pub MAC: ::std::string::String,
    // special fields
    // @@protoc_insertion_point(special_field:PDCACCGOMJB.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a PDCACCGOMJB {
    fn default() -> &'a PDCACCGOMJB {
        <PDCACCGOMJB as ::protobuf::Message>::default_instance()
    }
}

impl PDCACCGOMJB {
    pub fn new() -> PDCACCGOMJB {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(7);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "KLMOOMBPHIO",
            |m: &PDCACCGOMJB| { &m.KLMOOMBPHIO },
            |m: &mut PDCACCGOMJB| { &mut m.KLMOOMBPHIO },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "OEOKBLIOGCB",
            |m: &PDCACCGOMJB| { &m.OEOKBLIOGCB },
            |m: &mut PDCACCGOMJB| { &mut m.OEOKBLIOGCB },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "DCNFJNLJEDO",
            |m: &PDCACCGOMJB| { &m.DCNFJNLJEDO },
            |m: &mut PDCACCGOMJB| { &mut m.DCNFJNLJEDO },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "GBNOPNPEAOF",
            |m: &PDCACCGOMJB| { &m.GBNOPNPEAOF },
            |m: &mut PDCACCGOMJB| { &mut m.GBNOPNPEAOF },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "AMOIBMJGKNN",
            |m: &PDCACCGOMJB| { &m.AMOIBMJGKNN },
            |m: &mut PDCACCGOMJB| { &mut m.AMOIBMJGKNN },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "LLCMICAEFGM",
            |m: &PDCACCGOMJB| { &m.LLCMICAEFGM },
            |m: &mut PDCACCGOMJB| { &mut m.LLCMICAEFGM },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "MAC",
            |m: &PDCACCGOMJB| { &m.MAC },
            |m: &mut PDCACCGOMJB| { &mut m.MAC },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<PDCACCGOMJB>(
            "PDCACCGOMJB",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for PDCACCGOMJB {
    const NAME: &'static str = "PDCACCGOMJB";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                10 => {
                    self.KLMOOMBPHIO = is.read_string()?;
                },
                18 => {
                    self.OEOKBLIOGCB = is.read_string()?;
                },
                26 => {
                    self.DCNFJNLJEDO = is.read_string()?;
                },
                34 => {
                    self.GBNOPNPEAOF = is.read_string()?;
                },
                42 => {
                    self.AMOIBMJGKNN = is.read_string()?;
                },
                50 => {
                    self.LLCMICAEFGM = is.read_string()?;
                },
                58 => {
                    self.MAC = is.read_string()?;
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
        if !self.KLMOOMBPHIO.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.KLMOOMBPHIO);
        }
        if !self.OEOKBLIOGCB.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.OEOKBLIOGCB);
        }
        if !self.DCNFJNLJEDO.is_empty() {
            my_size += ::protobuf::rt::string_size(3, &self.DCNFJNLJEDO);
        }
        if !self.GBNOPNPEAOF.is_empty() {
            my_size += ::protobuf::rt::string_size(4, &self.GBNOPNPEAOF);
        }
        if !self.AMOIBMJGKNN.is_empty() {
            my_size += ::protobuf::rt::string_size(5, &self.AMOIBMJGKNN);
        }
        if !self.LLCMICAEFGM.is_empty() {
            my_size += ::protobuf::rt::string_size(6, &self.LLCMICAEFGM);
        }
        if !self.MAC.is_empty() {
            my_size += ::protobuf::rt::string_size(7, &self.MAC);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if !self.KLMOOMBPHIO.is_empty() {
            os.write_string(1, &self.KLMOOMBPHIO)?;
        }
        if !self.OEOKBLIOGCB.is_empty() {
            os.write_string(2, &self.OEOKBLIOGCB)?;
        }
        if !self.DCNFJNLJEDO.is_empty() {
            os.write_string(3, &self.DCNFJNLJEDO)?;
        }
        if !self.GBNOPNPEAOF.is_empty() {
            os.write_string(4, &self.GBNOPNPEAOF)?;
        }
        if !self.AMOIBMJGKNN.is_empty() {
            os.write_string(5, &self.AMOIBMJGKNN)?;
        }
        if !self.LLCMICAEFGM.is_empty() {
            os.write_string(6, &self.LLCMICAEFGM)?;
        }
        if !self.MAC.is_empty() {
            os.write_string(7, &self.MAC)?;
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

    fn new() -> PDCACCGOMJB {
        PDCACCGOMJB::new()
    }

    fn clear(&mut self) {
        self.KLMOOMBPHIO.clear();
        self.OEOKBLIOGCB.clear();
        self.DCNFJNLJEDO.clear();
        self.GBNOPNPEAOF.clear();
        self.AMOIBMJGKNN.clear();
        self.LLCMICAEFGM.clear();
        self.MAC.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static PDCACCGOMJB {
        static instance: PDCACCGOMJB = PDCACCGOMJB {
            KLMOOMBPHIO: ::std::string::String::new(),
            OEOKBLIOGCB: ::std::string::String::new(),
            DCNFJNLJEDO: ::std::string::String::new(),
            GBNOPNPEAOF: ::std::string::String::new(),
            AMOIBMJGKNN: ::std::string::String::new(),
            LLCMICAEFGM: ::std::string::String::new(),
            MAC: ::std::string::String::new(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for PDCACCGOMJB {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("PDCACCGOMJB").unwrap()).clone()
    }
}

impl ::std::fmt::Display for PDCACCGOMJB {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for PDCACCGOMJB {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11PDCACCGOMJB.proto\"\xeb\x01\n\x0bPDCACCGOMJB\x12\x20\n\x0bKLMOOMBP\
    HIO\x18\x01\x20\x01(\tR\x0bKLMOOMBPHIO\x12\x20\n\x0bOEOKBLIOGCB\x18\x02\
    \x20\x01(\tR\x0bOEOKBLIOGCB\x12\x20\n\x0bDCNFJNLJEDO\x18\x03\x20\x01(\tR\
    \x0bDCNFJNLJEDO\x12\x20\n\x0bGBNOPNPEAOF\x18\x04\x20\x01(\tR\x0bGBNOPNPE\
    AOF\x12\x20\n\x0bAMOIBMJGKNN\x18\x05\x20\x01(\tR\x0bAMOIBMJGKNN\x12\x20\
    \n\x0bLLCMICAEFGM\x18\x06\x20\x01(\tR\x0bLLCMICAEFGM\x12\x10\n\x03MAC\
    \x18\x07\x20\x01(\tR\x03MACb\x06proto3\
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
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(PDCACCGOMJB::generated_message_descriptor_data());
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
