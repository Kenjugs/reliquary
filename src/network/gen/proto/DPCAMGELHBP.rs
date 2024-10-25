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

//! Generated file from `DPCAMGELHBP.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:DPCAMGELHBP)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct DPCAMGELHBP {
    // message fields
    // @@protoc_insertion_point(field:DPCAMGELHBP.HINIBDNFAON)
    pub HINIBDNFAON: ::std::vec::Vec<super::HAKAOKNJJDL::HAKAOKNJJDL>,
    // @@protoc_insertion_point(field:DPCAMGELHBP.LFHCJIBBMHB)
    pub LFHCJIBBMHB: ::std::vec::Vec<u32>,
    // special fields
    // @@protoc_insertion_point(special_field:DPCAMGELHBP.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a DPCAMGELHBP {
    fn default() -> &'a DPCAMGELHBP {
        <DPCAMGELHBP as ::protobuf::Message>::default_instance()
    }
}

impl DPCAMGELHBP {
    pub fn new() -> DPCAMGELHBP {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(2);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "HINIBDNFAON",
            |m: &DPCAMGELHBP| { &m.HINIBDNFAON },
            |m: &mut DPCAMGELHBP| { &mut m.HINIBDNFAON },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "LFHCJIBBMHB",
            |m: &DPCAMGELHBP| { &m.LFHCJIBBMHB },
            |m: &mut DPCAMGELHBP| { &mut m.LFHCJIBBMHB },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<DPCAMGELHBP>(
            "DPCAMGELHBP",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for DPCAMGELHBP {
    const NAME: &'static str = "DPCAMGELHBP";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                74 => {
                    self.HINIBDNFAON.push(is.read_message()?);
                },
                42 => {
                    is.read_repeated_packed_uint32_into(&mut self.LFHCJIBBMHB)?;
                },
                40 => {
                    self.LFHCJIBBMHB.push(is.read_uint32()?);
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
        for value in &self.HINIBDNFAON {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        for value in &self.LFHCJIBBMHB {
            my_size += ::protobuf::rt::uint32_size(5, *value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        for v in &self.HINIBDNFAON {
            ::protobuf::rt::write_message_field_with_cached_size(9, v, os)?;
        };
        for v in &self.LFHCJIBBMHB {
            os.write_uint32(5, *v)?;
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

    fn new() -> DPCAMGELHBP {
        DPCAMGELHBP::new()
    }

    fn clear(&mut self) {
        self.HINIBDNFAON.clear();
        self.LFHCJIBBMHB.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static DPCAMGELHBP {
        static instance: DPCAMGELHBP = DPCAMGELHBP {
            HINIBDNFAON: ::std::vec::Vec::new(),
            LFHCJIBBMHB: ::std::vec::Vec::new(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for DPCAMGELHBP {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("DPCAMGELHBP").unwrap()).clone()
    }
}

impl ::std::fmt::Display for DPCAMGELHBP {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for DPCAMGELHBP {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11DPCAMGELHBP.proto\x1a\x11HAKAOKNJJDL.proto\"_\n\x0bDPCAMGELHBP\x12\
    .\n\x0bHINIBDNFAON\x18\t\x20\x03(\x0b2\x0c.HAKAOKNJJDLR\x0bHINIBDNFAON\
    \x12\x20\n\x0bLFHCJIBBMHB\x18\x05\x20\x03(\rR\x0bLFHCJIBBMHBb\x06proto3\
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
            deps.push(super::HAKAOKNJJDL::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(DPCAMGELHBP::generated_message_descriptor_data());
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
