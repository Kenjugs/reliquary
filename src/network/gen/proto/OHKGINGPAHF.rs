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

//! Generated file from `OHKGINGPAHF.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:OHKGINGPAHF)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct OHKGINGPAHF {
    // message fields
    // @@protoc_insertion_point(field:OHKGINGPAHF.PLNPPJMHGFE)
    pub PLNPPJMHGFE: ::std::vec::Vec<super::CEEPCDDKLKA::CEEPCDDKLKA>,
    // @@protoc_insertion_point(field:OHKGINGPAHF.KFCIJHFJDCA)
    pub KFCIJHFJDCA: u32,
    // @@protoc_insertion_point(field:OHKGINGPAHF.DOKHJDCPPIB)
    pub DOKHJDCPPIB: ::std::vec::Vec<super::CEEPCDDKLKA::CEEPCDDKLKA>,
    // special fields
    // @@protoc_insertion_point(special_field:OHKGINGPAHF.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a OHKGINGPAHF {
    fn default() -> &'a OHKGINGPAHF {
        <OHKGINGPAHF as ::protobuf::Message>::default_instance()
    }
}

impl OHKGINGPAHF {
    pub fn new() -> OHKGINGPAHF {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(3);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "PLNPPJMHGFE",
            |m: &OHKGINGPAHF| { &m.PLNPPJMHGFE },
            |m: &mut OHKGINGPAHF| { &mut m.PLNPPJMHGFE },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "KFCIJHFJDCA",
            |m: &OHKGINGPAHF| { &m.KFCIJHFJDCA },
            |m: &mut OHKGINGPAHF| { &mut m.KFCIJHFJDCA },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "DOKHJDCPPIB",
            |m: &OHKGINGPAHF| { &m.DOKHJDCPPIB },
            |m: &mut OHKGINGPAHF| { &mut m.DOKHJDCPPIB },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<OHKGINGPAHF>(
            "OHKGINGPAHF",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for OHKGINGPAHF {
    const NAME: &'static str = "OHKGINGPAHF";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                66 => {
                    self.PLNPPJMHGFE.push(is.read_message()?);
                },
                80 => {
                    self.KFCIJHFJDCA = is.read_uint32()?;
                },
                34 => {
                    self.DOKHJDCPPIB.push(is.read_message()?);
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
        for value in &self.PLNPPJMHGFE {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        if self.KFCIJHFJDCA != 0 {
            my_size += ::protobuf::rt::uint32_size(10, self.KFCIJHFJDCA);
        }
        for value in &self.DOKHJDCPPIB {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        for v in &self.PLNPPJMHGFE {
            ::protobuf::rt::write_message_field_with_cached_size(8, v, os)?;
        };
        if self.KFCIJHFJDCA != 0 {
            os.write_uint32(10, self.KFCIJHFJDCA)?;
        }
        for v in &self.DOKHJDCPPIB {
            ::protobuf::rt::write_message_field_with_cached_size(4, v, os)?;
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

    fn new() -> OHKGINGPAHF {
        OHKGINGPAHF::new()
    }

    fn clear(&mut self) {
        self.PLNPPJMHGFE.clear();
        self.KFCIJHFJDCA = 0;
        self.DOKHJDCPPIB.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static OHKGINGPAHF {
        static instance: OHKGINGPAHF = OHKGINGPAHF {
            PLNPPJMHGFE: ::std::vec::Vec::new(),
            KFCIJHFJDCA: 0,
            DOKHJDCPPIB: ::std::vec::Vec::new(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for OHKGINGPAHF {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("OHKGINGPAHF").unwrap()).clone()
    }
}

impl ::std::fmt::Display for OHKGINGPAHF {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for OHKGINGPAHF {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11OHKGINGPAHF.proto\x1a\x11CEEPCDDKLKA.proto\"\x8f\x01\n\x0bOHKGINGP\
    AHF\x12.\n\x0bPLNPPJMHGFE\x18\x08\x20\x03(\x0b2\x0c.CEEPCDDKLKAR\x0bPLNP\
    PJMHGFE\x12\x20\n\x0bKFCIJHFJDCA\x18\n\x20\x01(\rR\x0bKFCIJHFJDCA\x12.\n\
    \x0bDOKHJDCPPIB\x18\x04\x20\x03(\x0b2\x0c.CEEPCDDKLKAR\x0bDOKHJDCPPIBb\
    \x06proto3\
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
            let mut deps = ::std::vec::Vec::with_capacity(1);
            deps.push(super::CEEPCDDKLKA::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(OHKGINGPAHF::generated_message_descriptor_data());
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
