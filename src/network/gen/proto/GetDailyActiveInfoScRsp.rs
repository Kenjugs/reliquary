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

//! Generated file from `GetDailyActiveInfoScRsp.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:GetDailyActiveInfoScRsp)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct GetDailyActiveInfoScRsp {
    // message fields
    // @@protoc_insertion_point(field:GetDailyActiveInfoScRsp.FPNDPLMIGJP)
    pub FPNDPLMIGJP: u32,
    // @@protoc_insertion_point(field:GetDailyActiveInfoScRsp.EKLOBIDAGOD)
    pub EKLOBIDAGOD: ::std::vec::Vec<u32>,
    // @@protoc_insertion_point(field:GetDailyActiveInfoScRsp.retcode)
    pub retcode: u32,
    // @@protoc_insertion_point(field:GetDailyActiveInfoScRsp.KGCLFOABCIB)
    pub KGCLFOABCIB: ::std::vec::Vec<super::LEPJPJHLDBO::LEPJPJHLDBO>,
    // special fields
    // @@protoc_insertion_point(special_field:GetDailyActiveInfoScRsp.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a GetDailyActiveInfoScRsp {
    fn default() -> &'a GetDailyActiveInfoScRsp {
        <GetDailyActiveInfoScRsp as ::protobuf::Message>::default_instance()
    }
}

impl GetDailyActiveInfoScRsp {
    pub fn new() -> GetDailyActiveInfoScRsp {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(4);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "FPNDPLMIGJP",
            |m: &GetDailyActiveInfoScRsp| { &m.FPNDPLMIGJP },
            |m: &mut GetDailyActiveInfoScRsp| { &mut m.FPNDPLMIGJP },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "EKLOBIDAGOD",
            |m: &GetDailyActiveInfoScRsp| { &m.EKLOBIDAGOD },
            |m: &mut GetDailyActiveInfoScRsp| { &mut m.EKLOBIDAGOD },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "retcode",
            |m: &GetDailyActiveInfoScRsp| { &m.retcode },
            |m: &mut GetDailyActiveInfoScRsp| { &mut m.retcode },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "KGCLFOABCIB",
            |m: &GetDailyActiveInfoScRsp| { &m.KGCLFOABCIB },
            |m: &mut GetDailyActiveInfoScRsp| { &mut m.KGCLFOABCIB },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<GetDailyActiveInfoScRsp>(
            "GetDailyActiveInfoScRsp",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for GetDailyActiveInfoScRsp {
    const NAME: &'static str = "GetDailyActiveInfoScRsp";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                48 => {
                    self.FPNDPLMIGJP = is.read_uint32()?;
                },
                66 => {
                    is.read_repeated_packed_uint32_into(&mut self.EKLOBIDAGOD)?;
                },
                64 => {
                    self.EKLOBIDAGOD.push(is.read_uint32()?);
                },
                56 => {
                    self.retcode = is.read_uint32()?;
                },
                10 => {
                    self.KGCLFOABCIB.push(is.read_message()?);
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
        if self.FPNDPLMIGJP != 0 {
            my_size += ::protobuf::rt::uint32_size(6, self.FPNDPLMIGJP);
        }
        for value in &self.EKLOBIDAGOD {
            my_size += ::protobuf::rt::uint32_size(8, *value);
        };
        if self.retcode != 0 {
            my_size += ::protobuf::rt::uint32_size(7, self.retcode);
        }
        for value in &self.KGCLFOABCIB {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.FPNDPLMIGJP != 0 {
            os.write_uint32(6, self.FPNDPLMIGJP)?;
        }
        for v in &self.EKLOBIDAGOD {
            os.write_uint32(8, *v)?;
        };
        if self.retcode != 0 {
            os.write_uint32(7, self.retcode)?;
        }
        for v in &self.KGCLFOABCIB {
            ::protobuf::rt::write_message_field_with_cached_size(1, v, os)?;
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

    fn new() -> GetDailyActiveInfoScRsp {
        GetDailyActiveInfoScRsp::new()
    }

    fn clear(&mut self) {
        self.FPNDPLMIGJP = 0;
        self.EKLOBIDAGOD.clear();
        self.retcode = 0;
        self.KGCLFOABCIB.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static GetDailyActiveInfoScRsp {
        static instance: GetDailyActiveInfoScRsp = GetDailyActiveInfoScRsp {
            FPNDPLMIGJP: 0,
            EKLOBIDAGOD: ::std::vec::Vec::new(),
            retcode: 0,
            KGCLFOABCIB: ::std::vec::Vec::new(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for GetDailyActiveInfoScRsp {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("GetDailyActiveInfoScRsp").unwrap()).clone()
    }
}

impl ::std::fmt::Display for GetDailyActiveInfoScRsp {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for GetDailyActiveInfoScRsp {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x1dGetDailyActiveInfoScRsp.proto\x1a\x11LEPJPJHLDBO.proto\"\xa7\x01\n\
    \x17GetDailyActiveInfoScRsp\x12\x20\n\x0bFPNDPLMIGJP\x18\x06\x20\x01(\rR\
    \x0bFPNDPLMIGJP\x12\x20\n\x0bEKLOBIDAGOD\x18\x08\x20\x03(\rR\x0bEKLOBIDA\
    GOD\x12\x18\n\x07retcode\x18\x07\x20\x01(\rR\x07retcode\x12.\n\x0bKGCLFO\
    ABCIB\x18\x01\x20\x03(\x0b2\x0c.LEPJPJHLDBOR\x0bKGCLFOABCIBb\x06proto3\
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
            deps.push(super::LEPJPJHLDBO::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(GetDailyActiveInfoScRsp::generated_message_descriptor_data());
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
