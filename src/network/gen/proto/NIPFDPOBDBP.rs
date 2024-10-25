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

//! Generated file from `NIPFDPOBDBP.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:NIPFDPOBDBP)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct NIPFDPOBDBP {
    // message fields
    // @@protoc_insertion_point(field:NIPFDPOBDBP.EFDPHFELDEG)
    pub EFDPHFELDEG: u32,
    // @@protoc_insertion_point(field:NIPFDPOBDBP.KFJCGMAFANN)
    pub KFJCGMAFANN: ::std::vec::Vec<u32>,
    // @@protoc_insertion_point(field:NIPFDPOBDBP.BINLNAGFFEP)
    pub BINLNAGFFEP: u32,
    // @@protoc_insertion_point(field:NIPFDPOBDBP.DAAIBKIKBEJ)
    pub DAAIBKIKBEJ: u32,
    // special fields
    // @@protoc_insertion_point(special_field:NIPFDPOBDBP.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a NIPFDPOBDBP {
    fn default() -> &'a NIPFDPOBDBP {
        <NIPFDPOBDBP as ::protobuf::Message>::default_instance()
    }
}

impl NIPFDPOBDBP {
    pub fn new() -> NIPFDPOBDBP {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(4);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "EFDPHFELDEG",
            |m: &NIPFDPOBDBP| { &m.EFDPHFELDEG },
            |m: &mut NIPFDPOBDBP| { &mut m.EFDPHFELDEG },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "KFJCGMAFANN",
            |m: &NIPFDPOBDBP| { &m.KFJCGMAFANN },
            |m: &mut NIPFDPOBDBP| { &mut m.KFJCGMAFANN },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "BINLNAGFFEP",
            |m: &NIPFDPOBDBP| { &m.BINLNAGFFEP },
            |m: &mut NIPFDPOBDBP| { &mut m.BINLNAGFFEP },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "DAAIBKIKBEJ",
            |m: &NIPFDPOBDBP| { &m.DAAIBKIKBEJ },
            |m: &mut NIPFDPOBDBP| { &mut m.DAAIBKIKBEJ },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<NIPFDPOBDBP>(
            "NIPFDPOBDBP",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for NIPFDPOBDBP {
    const NAME: &'static str = "NIPFDPOBDBP";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                8 => {
                    self.EFDPHFELDEG = is.read_uint32()?;
                },
                90 => {
                    is.read_repeated_packed_uint32_into(&mut self.KFJCGMAFANN)?;
                },
                88 => {
                    self.KFJCGMAFANN.push(is.read_uint32()?);
                },
                32 => {
                    self.BINLNAGFFEP = is.read_uint32()?;
                },
                72 => {
                    self.DAAIBKIKBEJ = is.read_uint32()?;
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
        if self.EFDPHFELDEG != 0 {
            my_size += ::protobuf::rt::uint32_size(1, self.EFDPHFELDEG);
        }
        for value in &self.KFJCGMAFANN {
            my_size += ::protobuf::rt::uint32_size(11, *value);
        };
        if self.BINLNAGFFEP != 0 {
            my_size += ::protobuf::rt::uint32_size(4, self.BINLNAGFFEP);
        }
        if self.DAAIBKIKBEJ != 0 {
            my_size += ::protobuf::rt::uint32_size(9, self.DAAIBKIKBEJ);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.EFDPHFELDEG != 0 {
            os.write_uint32(1, self.EFDPHFELDEG)?;
        }
        for v in &self.KFJCGMAFANN {
            os.write_uint32(11, *v)?;
        };
        if self.BINLNAGFFEP != 0 {
            os.write_uint32(4, self.BINLNAGFFEP)?;
        }
        if self.DAAIBKIKBEJ != 0 {
            os.write_uint32(9, self.DAAIBKIKBEJ)?;
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

    fn new() -> NIPFDPOBDBP {
        NIPFDPOBDBP::new()
    }

    fn clear(&mut self) {
        self.EFDPHFELDEG = 0;
        self.KFJCGMAFANN.clear();
        self.BINLNAGFFEP = 0;
        self.DAAIBKIKBEJ = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static NIPFDPOBDBP {
        static instance: NIPFDPOBDBP = NIPFDPOBDBP {
            EFDPHFELDEG: 0,
            KFJCGMAFANN: ::std::vec::Vec::new(),
            BINLNAGFFEP: 0,
            DAAIBKIKBEJ: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for NIPFDPOBDBP {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("NIPFDPOBDBP").unwrap()).clone()
    }
}

impl ::std::fmt::Display for NIPFDPOBDBP {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for NIPFDPOBDBP {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11NIPFDPOBDBP.proto\"\x95\x01\n\x0bNIPFDPOBDBP\x12\x20\n\x0bEFDPHFEL\
    DEG\x18\x01\x20\x01(\rR\x0bEFDPHFELDEG\x12\x20\n\x0bKFJCGMAFANN\x18\x0b\
    \x20\x03(\rR\x0bKFJCGMAFANN\x12\x20\n\x0bBINLNAGFFEP\x18\x04\x20\x01(\rR\
    \x0bBINLNAGFFEP\x12\x20\n\x0bDAAIBKIKBEJ\x18\t\x20\x01(\rR\x0bDAAIBKIKBE\
    Jb\x06proto3\
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
            messages.push(NIPFDPOBDBP::generated_message_descriptor_data());
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
