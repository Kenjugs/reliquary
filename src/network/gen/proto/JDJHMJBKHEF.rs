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

//! Generated file from `JDJHMJBKHEF.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:JDJHMJBKHEF)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct JDJHMJBKHEF {
    // message fields
    // @@protoc_insertion_point(field:JDJHMJBKHEF.KHHMFIPMFMP)
    pub KHHMFIPMFMP: u32,
    // @@protoc_insertion_point(field:JDJHMJBKHEF.FFDMJAPGFOA)
    pub FFDMJAPGFOA: u32,
    // @@protoc_insertion_point(field:JDJHMJBKHEF.EOAIJOLGFNN)
    pub EOAIJOLGFNN: f64,
    // @@protoc_insertion_point(field:JDJHMJBKHEF.BLEINMLIFDD)
    pub BLEINMLIFDD: u32,
    // @@protoc_insertion_point(field:JDJHMJBKHEF.KKCHCLOLPMB)
    pub KKCHCLOLPMB: u32,
    // special fields
    // @@protoc_insertion_point(special_field:JDJHMJBKHEF.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a JDJHMJBKHEF {
    fn default() -> &'a JDJHMJBKHEF {
        <JDJHMJBKHEF as ::protobuf::Message>::default_instance()
    }
}

impl JDJHMJBKHEF {
    pub fn new() -> JDJHMJBKHEF {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(5);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "KHHMFIPMFMP",
            |m: &JDJHMJBKHEF| { &m.KHHMFIPMFMP },
            |m: &mut JDJHMJBKHEF| { &mut m.KHHMFIPMFMP },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "FFDMJAPGFOA",
            |m: &JDJHMJBKHEF| { &m.FFDMJAPGFOA },
            |m: &mut JDJHMJBKHEF| { &mut m.FFDMJAPGFOA },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "EOAIJOLGFNN",
            |m: &JDJHMJBKHEF| { &m.EOAIJOLGFNN },
            |m: &mut JDJHMJBKHEF| { &mut m.EOAIJOLGFNN },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "BLEINMLIFDD",
            |m: &JDJHMJBKHEF| { &m.BLEINMLIFDD },
            |m: &mut JDJHMJBKHEF| { &mut m.BLEINMLIFDD },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "KKCHCLOLPMB",
            |m: &JDJHMJBKHEF| { &m.KKCHCLOLPMB },
            |m: &mut JDJHMJBKHEF| { &mut m.KKCHCLOLPMB },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<JDJHMJBKHEF>(
            "JDJHMJBKHEF",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for JDJHMJBKHEF {
    const NAME: &'static str = "JDJHMJBKHEF";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                8 => {
                    self.KHHMFIPMFMP = is.read_uint32()?;
                },
                16 => {
                    self.FFDMJAPGFOA = is.read_uint32()?;
                },
                25 => {
                    self.EOAIJOLGFNN = is.read_double()?;
                },
                32 => {
                    self.BLEINMLIFDD = is.read_uint32()?;
                },
                40 => {
                    self.KKCHCLOLPMB = is.read_uint32()?;
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
        if self.KHHMFIPMFMP != 0 {
            my_size += ::protobuf::rt::uint32_size(1, self.KHHMFIPMFMP);
        }
        if self.FFDMJAPGFOA != 0 {
            my_size += ::protobuf::rt::uint32_size(2, self.FFDMJAPGFOA);
        }
        if self.EOAIJOLGFNN != 0. {
            my_size += 1 + 8;
        }
        if self.BLEINMLIFDD != 0 {
            my_size += ::protobuf::rt::uint32_size(4, self.BLEINMLIFDD);
        }
        if self.KKCHCLOLPMB != 0 {
            my_size += ::protobuf::rt::uint32_size(5, self.KKCHCLOLPMB);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.KHHMFIPMFMP != 0 {
            os.write_uint32(1, self.KHHMFIPMFMP)?;
        }
        if self.FFDMJAPGFOA != 0 {
            os.write_uint32(2, self.FFDMJAPGFOA)?;
        }
        if self.EOAIJOLGFNN != 0. {
            os.write_double(3, self.EOAIJOLGFNN)?;
        }
        if self.BLEINMLIFDD != 0 {
            os.write_uint32(4, self.BLEINMLIFDD)?;
        }
        if self.KKCHCLOLPMB != 0 {
            os.write_uint32(5, self.KKCHCLOLPMB)?;
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

    fn new() -> JDJHMJBKHEF {
        JDJHMJBKHEF::new()
    }

    fn clear(&mut self) {
        self.KHHMFIPMFMP = 0;
        self.FFDMJAPGFOA = 0;
        self.EOAIJOLGFNN = 0.;
        self.BLEINMLIFDD = 0;
        self.KKCHCLOLPMB = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static JDJHMJBKHEF {
        static instance: JDJHMJBKHEF = JDJHMJBKHEF {
            KHHMFIPMFMP: 0,
            FFDMJAPGFOA: 0,
            EOAIJOLGFNN: 0.,
            BLEINMLIFDD: 0,
            KKCHCLOLPMB: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for JDJHMJBKHEF {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("JDJHMJBKHEF").unwrap()).clone()
    }
}

impl ::std::fmt::Display for JDJHMJBKHEF {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for JDJHMJBKHEF {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11JDJHMJBKHEF.proto\"\xb7\x01\n\x0bJDJHMJBKHEF\x12\x20\n\x0bKHHMFIPM\
    FMP\x18\x01\x20\x01(\rR\x0bKHHMFIPMFMP\x12\x20\n\x0bFFDMJAPGFOA\x18\x02\
    \x20\x01(\rR\x0bFFDMJAPGFOA\x12\x20\n\x0bEOAIJOLGFNN\x18\x03\x20\x01(\
    \x01R\x0bEOAIJOLGFNN\x12\x20\n\x0bBLEINMLIFDD\x18\x04\x20\x01(\rR\x0bBLE\
    INMLIFDD\x12\x20\n\x0bKKCHCLOLPMB\x18\x05\x20\x01(\rR\x0bKKCHCLOLPMBb\
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
            let mut deps = ::std::vec::Vec::with_capacity(0);
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(JDJHMJBKHEF::generated_message_descriptor_data());
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
