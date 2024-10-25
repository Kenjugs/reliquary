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

//! Generated file from `DJPHMOJGKCO.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:DJPHMOJGKCO)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct DJPHMOJGKCO {
    // message fields
    // @@protoc_insertion_point(field:DJPHMOJGKCO.IBMPGMBHHPP)
    pub IBMPGMBHHPP: u32,
    // @@protoc_insertion_point(field:DJPHMOJGKCO.AEDDEMIGDGB)
    pub AEDDEMIGDGB: u32,
    // @@protoc_insertion_point(field:DJPHMOJGKCO.OJDNJFDJIMP)
    pub OJDNJFDJIMP: u32,
    // @@protoc_insertion_point(field:DJPHMOJGKCO.PCABIMPKMBN)
    pub PCABIMPKMBN: u32,
    // special fields
    // @@protoc_insertion_point(special_field:DJPHMOJGKCO.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a DJPHMOJGKCO {
    fn default() -> &'a DJPHMOJGKCO {
        <DJPHMOJGKCO as ::protobuf::Message>::default_instance()
    }
}

impl DJPHMOJGKCO {
    pub fn new() -> DJPHMOJGKCO {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(4);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "IBMPGMBHHPP",
            |m: &DJPHMOJGKCO| { &m.IBMPGMBHHPP },
            |m: &mut DJPHMOJGKCO| { &mut m.IBMPGMBHHPP },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "AEDDEMIGDGB",
            |m: &DJPHMOJGKCO| { &m.AEDDEMIGDGB },
            |m: &mut DJPHMOJGKCO| { &mut m.AEDDEMIGDGB },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "OJDNJFDJIMP",
            |m: &DJPHMOJGKCO| { &m.OJDNJFDJIMP },
            |m: &mut DJPHMOJGKCO| { &mut m.OJDNJFDJIMP },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "PCABIMPKMBN",
            |m: &DJPHMOJGKCO| { &m.PCABIMPKMBN },
            |m: &mut DJPHMOJGKCO| { &mut m.PCABIMPKMBN },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<DJPHMOJGKCO>(
            "DJPHMOJGKCO",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for DJPHMOJGKCO {
    const NAME: &'static str = "DJPHMOJGKCO";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                112 => {
                    self.IBMPGMBHHPP = is.read_uint32()?;
                },
                8 => {
                    self.AEDDEMIGDGB = is.read_uint32()?;
                },
                96 => {
                    self.OJDNJFDJIMP = is.read_uint32()?;
                },
                72 => {
                    self.PCABIMPKMBN = is.read_uint32()?;
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
        if self.IBMPGMBHHPP != 0 {
            my_size += ::protobuf::rt::uint32_size(14, self.IBMPGMBHHPP);
        }
        if self.AEDDEMIGDGB != 0 {
            my_size += ::protobuf::rt::uint32_size(1, self.AEDDEMIGDGB);
        }
        if self.OJDNJFDJIMP != 0 {
            my_size += ::protobuf::rt::uint32_size(12, self.OJDNJFDJIMP);
        }
        if self.PCABIMPKMBN != 0 {
            my_size += ::protobuf::rt::uint32_size(9, self.PCABIMPKMBN);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.IBMPGMBHHPP != 0 {
            os.write_uint32(14, self.IBMPGMBHHPP)?;
        }
        if self.AEDDEMIGDGB != 0 {
            os.write_uint32(1, self.AEDDEMIGDGB)?;
        }
        if self.OJDNJFDJIMP != 0 {
            os.write_uint32(12, self.OJDNJFDJIMP)?;
        }
        if self.PCABIMPKMBN != 0 {
            os.write_uint32(9, self.PCABIMPKMBN)?;
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

    fn new() -> DJPHMOJGKCO {
        DJPHMOJGKCO::new()
    }

    fn clear(&mut self) {
        self.IBMPGMBHHPP = 0;
        self.AEDDEMIGDGB = 0;
        self.OJDNJFDJIMP = 0;
        self.PCABIMPKMBN = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static DJPHMOJGKCO {
        static instance: DJPHMOJGKCO = DJPHMOJGKCO {
            IBMPGMBHHPP: 0,
            AEDDEMIGDGB: 0,
            OJDNJFDJIMP: 0,
            PCABIMPKMBN: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for DJPHMOJGKCO {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("DJPHMOJGKCO").unwrap()).clone()
    }
}

impl ::std::fmt::Display for DJPHMOJGKCO {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for DJPHMOJGKCO {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11DJPHMOJGKCO.proto\"\x95\x01\n\x0bDJPHMOJGKCO\x12\x20\n\x0bIBMPGMBH\
    HPP\x18\x0e\x20\x01(\rR\x0bIBMPGMBHHPP\x12\x20\n\x0bAEDDEMIGDGB\x18\x01\
    \x20\x01(\rR\x0bAEDDEMIGDGB\x12\x20\n\x0bOJDNJFDJIMP\x18\x0c\x20\x01(\rR\
    \x0bOJDNJFDJIMP\x12\x20\n\x0bPCABIMPKMBN\x18\t\x20\x01(\rR\x0bPCABIMPKMB\
    Nb\x06proto3\
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
            messages.push(DJPHMOJGKCO::generated_message_descriptor_data());
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
