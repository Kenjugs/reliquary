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

//! Generated file from `JPCAGPHCLKB.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:JPCAGPHCLKB)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct JPCAGPHCLKB {
    // message fields
    // @@protoc_insertion_point(field:JPCAGPHCLKB.BJHAKIGNEEO)
    pub BJHAKIGNEEO: ::std::vec::Vec<u32>,
    // @@protoc_insertion_point(field:JPCAGPHCLKB.HADENLEHOOL)
    pub HADENLEHOOL: bool,
    // @@protoc_insertion_point(field:JPCAGPHCLKB.PCFGOMJNGDM)
    pub PCFGOMJNGDM: u32,
    // @@protoc_insertion_point(field:JPCAGPHCLKB.LCNEAKONLCB)
    pub LCNEAKONLCB: u32,
    // special fields
    // @@protoc_insertion_point(special_field:JPCAGPHCLKB.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a JPCAGPHCLKB {
    fn default() -> &'a JPCAGPHCLKB {
        <JPCAGPHCLKB as ::protobuf::Message>::default_instance()
    }
}

impl JPCAGPHCLKB {
    pub fn new() -> JPCAGPHCLKB {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(4);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "BJHAKIGNEEO",
            |m: &JPCAGPHCLKB| { &m.BJHAKIGNEEO },
            |m: &mut JPCAGPHCLKB| { &mut m.BJHAKIGNEEO },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "HADENLEHOOL",
            |m: &JPCAGPHCLKB| { &m.HADENLEHOOL },
            |m: &mut JPCAGPHCLKB| { &mut m.HADENLEHOOL },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "PCFGOMJNGDM",
            |m: &JPCAGPHCLKB| { &m.PCFGOMJNGDM },
            |m: &mut JPCAGPHCLKB| { &mut m.PCFGOMJNGDM },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "LCNEAKONLCB",
            |m: &JPCAGPHCLKB| { &m.LCNEAKONLCB },
            |m: &mut JPCAGPHCLKB| { &mut m.LCNEAKONLCB },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<JPCAGPHCLKB>(
            "JPCAGPHCLKB",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for JPCAGPHCLKB {
    const NAME: &'static str = "JPCAGPHCLKB";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                82 => {
                    is.read_repeated_packed_uint32_into(&mut self.BJHAKIGNEEO)?;
                },
                80 => {
                    self.BJHAKIGNEEO.push(is.read_uint32()?);
                },
                64 => {
                    self.HADENLEHOOL = is.read_bool()?;
                },
                88 => {
                    self.PCFGOMJNGDM = is.read_uint32()?;
                },
                120 => {
                    self.LCNEAKONLCB = is.read_uint32()?;
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
        for value in &self.BJHAKIGNEEO {
            my_size += ::protobuf::rt::uint32_size(10, *value);
        };
        if self.HADENLEHOOL != false {
            my_size += 1 + 1;
        }
        if self.PCFGOMJNGDM != 0 {
            my_size += ::protobuf::rt::uint32_size(11, self.PCFGOMJNGDM);
        }
        if self.LCNEAKONLCB != 0 {
            my_size += ::protobuf::rt::uint32_size(15, self.LCNEAKONLCB);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        for v in &self.BJHAKIGNEEO {
            os.write_uint32(10, *v)?;
        };
        if self.HADENLEHOOL != false {
            os.write_bool(8, self.HADENLEHOOL)?;
        }
        if self.PCFGOMJNGDM != 0 {
            os.write_uint32(11, self.PCFGOMJNGDM)?;
        }
        if self.LCNEAKONLCB != 0 {
            os.write_uint32(15, self.LCNEAKONLCB)?;
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

    fn new() -> JPCAGPHCLKB {
        JPCAGPHCLKB::new()
    }

    fn clear(&mut self) {
        self.BJHAKIGNEEO.clear();
        self.HADENLEHOOL = false;
        self.PCFGOMJNGDM = 0;
        self.LCNEAKONLCB = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static JPCAGPHCLKB {
        static instance: JPCAGPHCLKB = JPCAGPHCLKB {
            BJHAKIGNEEO: ::std::vec::Vec::new(),
            HADENLEHOOL: false,
            PCFGOMJNGDM: 0,
            LCNEAKONLCB: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for JPCAGPHCLKB {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("JPCAGPHCLKB").unwrap()).clone()
    }
}

impl ::std::fmt::Display for JPCAGPHCLKB {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for JPCAGPHCLKB {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11JPCAGPHCLKB.proto\"\x95\x01\n\x0bJPCAGPHCLKB\x12\x20\n\x0bBJHAKIGN\
    EEO\x18\n\x20\x03(\rR\x0bBJHAKIGNEEO\x12\x20\n\x0bHADENLEHOOL\x18\x08\
    \x20\x01(\x08R\x0bHADENLEHOOL\x12\x20\n\x0bPCFGOMJNGDM\x18\x0b\x20\x01(\
    \rR\x0bPCFGOMJNGDM\x12\x20\n\x0bLCNEAKONLCB\x18\x0f\x20\x01(\rR\x0bLCNEA\
    KONLCBb\x06proto3\
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
            messages.push(JPCAGPHCLKB::generated_message_descriptor_data());
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
