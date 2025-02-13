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

//! Generated file from `DHLNCCAIHLF.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:DHLNCCAIHLF)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct DHLNCCAIHLF {
    // message fields
    // @@protoc_insertion_point(field:DHLNCCAIHLF.NNHOFJNHCPI)
    pub NNHOFJNHCPI: u32,
    // @@protoc_insertion_point(field:DHLNCCAIHLF.FBFJEOMBKEO)
    pub FBFJEOMBKEO: ::std::vec::Vec<u32>,
    // @@protoc_insertion_point(field:DHLNCCAIHLF.status)
    pub status: ::protobuf::EnumOrUnknown<super::MessageSectionStatus::MessageSectionStatus>,
    // @@protoc_insertion_point(field:DHLNCCAIHLF.DGFNOGJFILI)
    pub DGFNOGJFILI: ::std::vec::Vec<super::PAMILHACAGM::PAMILHACAGM>,
    // @@protoc_insertion_point(field:DHLNCCAIHLF.id)
    pub id: u32,
    // special fields
    // @@protoc_insertion_point(special_field:DHLNCCAIHLF.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a DHLNCCAIHLF {
    fn default() -> &'a DHLNCCAIHLF {
        <DHLNCCAIHLF as ::protobuf::Message>::default_instance()
    }
}

impl DHLNCCAIHLF {
    pub fn new() -> DHLNCCAIHLF {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(5);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "NNHOFJNHCPI",
            |m: &DHLNCCAIHLF| { &m.NNHOFJNHCPI },
            |m: &mut DHLNCCAIHLF| { &mut m.NNHOFJNHCPI },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "FBFJEOMBKEO",
            |m: &DHLNCCAIHLF| { &m.FBFJEOMBKEO },
            |m: &mut DHLNCCAIHLF| { &mut m.FBFJEOMBKEO },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "status",
            |m: &DHLNCCAIHLF| { &m.status },
            |m: &mut DHLNCCAIHLF| { &mut m.status },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "DGFNOGJFILI",
            |m: &DHLNCCAIHLF| { &m.DGFNOGJFILI },
            |m: &mut DHLNCCAIHLF| { &mut m.DGFNOGJFILI },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "id",
            |m: &DHLNCCAIHLF| { &m.id },
            |m: &mut DHLNCCAIHLF| { &mut m.id },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<DHLNCCAIHLF>(
            "DHLNCCAIHLF",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for DHLNCCAIHLF {
    const NAME: &'static str = "DHLNCCAIHLF";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                112 => {
                    self.NNHOFJNHCPI = is.read_uint32()?;
                },
                74 => {
                    is.read_repeated_packed_uint32_into(&mut self.FBFJEOMBKEO)?;
                },
                72 => {
                    self.FBFJEOMBKEO.push(is.read_uint32()?);
                },
                80 => {
                    self.status = is.read_enum_or_unknown()?;
                },
                122 => {
                    self.DGFNOGJFILI.push(is.read_message()?);
                },
                24 => {
                    self.id = is.read_uint32()?;
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
        if self.NNHOFJNHCPI != 0 {
            my_size += ::protobuf::rt::uint32_size(14, self.NNHOFJNHCPI);
        }
        for value in &self.FBFJEOMBKEO {
            my_size += ::protobuf::rt::uint32_size(9, *value);
        };
        if self.status != ::protobuf::EnumOrUnknown::new(super::MessageSectionStatus::MessageSectionStatus::MESSAGE_SECTION_NONE) {
            my_size += ::protobuf::rt::int32_size(10, self.status.value());
        }
        for value in &self.DGFNOGJFILI {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        if self.id != 0 {
            my_size += ::protobuf::rt::uint32_size(3, self.id);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.NNHOFJNHCPI != 0 {
            os.write_uint32(14, self.NNHOFJNHCPI)?;
        }
        for v in &self.FBFJEOMBKEO {
            os.write_uint32(9, *v)?;
        };
        if self.status != ::protobuf::EnumOrUnknown::new(super::MessageSectionStatus::MessageSectionStatus::MESSAGE_SECTION_NONE) {
            os.write_enum(10, ::protobuf::EnumOrUnknown::value(&self.status))?;
        }
        for v in &self.DGFNOGJFILI {
            ::protobuf::rt::write_message_field_with_cached_size(15, v, os)?;
        };
        if self.id != 0 {
            os.write_uint32(3, self.id)?;
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

    fn new() -> DHLNCCAIHLF {
        DHLNCCAIHLF::new()
    }

    fn clear(&mut self) {
        self.NNHOFJNHCPI = 0;
        self.FBFJEOMBKEO.clear();
        self.status = ::protobuf::EnumOrUnknown::new(super::MessageSectionStatus::MessageSectionStatus::MESSAGE_SECTION_NONE);
        self.DGFNOGJFILI.clear();
        self.id = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static DHLNCCAIHLF {
        static instance: DHLNCCAIHLF = DHLNCCAIHLF {
            NNHOFJNHCPI: 0,
            FBFJEOMBKEO: ::std::vec::Vec::new(),
            status: ::protobuf::EnumOrUnknown::from_i32(0),
            DGFNOGJFILI: ::std::vec::Vec::new(),
            id: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for DHLNCCAIHLF {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("DHLNCCAIHLF").unwrap()).clone()
    }
}

impl ::std::fmt::Display for DHLNCCAIHLF {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for DHLNCCAIHLF {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11DHLNCCAIHLF.proto\x1a\x1aMessageSectionStatus.proto\x1a\x11PAMILHA\
    CAGM.proto\"\xc0\x01\n\x0bDHLNCCAIHLF\x12\x20\n\x0bNNHOFJNHCPI\x18\x0e\
    \x20\x01(\rR\x0bNNHOFJNHCPI\x12\x20\n\x0bFBFJEOMBKEO\x18\t\x20\x03(\rR\
    \x0bFBFJEOMBKEO\x12-\n\x06status\x18\n\x20\x01(\x0e2\x15.MessageSectionS\
    tatusR\x06status\x12.\n\x0bDGFNOGJFILI\x18\x0f\x20\x03(\x0b2\x0c.PAMILHA\
    CAGMR\x0bDGFNOGJFILI\x12\x0e\n\x02id\x18\x03\x20\x01(\rR\x02idb\x06proto\
    3\
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
            deps.push(super::MessageSectionStatus::file_descriptor().clone());
            deps.push(super::PAMILHACAGM::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(DHLNCCAIHLF::generated_message_descriptor_data());
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
