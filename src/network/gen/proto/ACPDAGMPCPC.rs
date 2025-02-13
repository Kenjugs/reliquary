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

//! Generated file from `ACPDAGMPCPC.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:ACPDAGMPCPC)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct ACPDAGMPCPC {
    // message fields
    // @@protoc_insertion_point(field:ACPDAGMPCPC.NEJJEFEOJOJ)
    pub NEJJEFEOJOJ: ::protobuf::EnumOrUnknown<super::BPOEMOLJCCE::BPOEMOLJCCE>,
    // @@protoc_insertion_point(field:ACPDAGMPCPC.level)
    pub level: u32,
    // @@protoc_insertion_point(field:ACPDAGMPCPC.EIFICNPAOIO)
    pub EIFICNPAOIO: u32,
    // @@protoc_insertion_point(field:ACPDAGMPCPC.uid)
    pub uid: u32,
    // @@protoc_insertion_point(field:ACPDAGMPCPC.KDLEHAPNACL)
    pub KDLEHAPNACL: ::std::string::String,
    // @@protoc_insertion_point(field:ACPDAGMPCPC.HFHGPLMEFGJ)
    pub HFHGPLMEFGJ: ::std::string::String,
    // @@protoc_insertion_point(field:ACPDAGMPCPC.MHCKOEADGEL)
    pub MHCKOEADGEL: u32,
    // @@protoc_insertion_point(field:ACPDAGMPCPC.DBPCMPCLOCI)
    pub DBPCMPCLOCI: ::std::string::String,
    // special fields
    // @@protoc_insertion_point(special_field:ACPDAGMPCPC.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a ACPDAGMPCPC {
    fn default() -> &'a ACPDAGMPCPC {
        <ACPDAGMPCPC as ::protobuf::Message>::default_instance()
    }
}

impl ACPDAGMPCPC {
    pub fn new() -> ACPDAGMPCPC {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(8);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "NEJJEFEOJOJ",
            |m: &ACPDAGMPCPC| { &m.NEJJEFEOJOJ },
            |m: &mut ACPDAGMPCPC| { &mut m.NEJJEFEOJOJ },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "level",
            |m: &ACPDAGMPCPC| { &m.level },
            |m: &mut ACPDAGMPCPC| { &mut m.level },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "EIFICNPAOIO",
            |m: &ACPDAGMPCPC| { &m.EIFICNPAOIO },
            |m: &mut ACPDAGMPCPC| { &mut m.EIFICNPAOIO },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "uid",
            |m: &ACPDAGMPCPC| { &m.uid },
            |m: &mut ACPDAGMPCPC| { &mut m.uid },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "KDLEHAPNACL",
            |m: &ACPDAGMPCPC| { &m.KDLEHAPNACL },
            |m: &mut ACPDAGMPCPC| { &mut m.KDLEHAPNACL },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "HFHGPLMEFGJ",
            |m: &ACPDAGMPCPC| { &m.HFHGPLMEFGJ },
            |m: &mut ACPDAGMPCPC| { &mut m.HFHGPLMEFGJ },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "MHCKOEADGEL",
            |m: &ACPDAGMPCPC| { &m.MHCKOEADGEL },
            |m: &mut ACPDAGMPCPC| { &mut m.MHCKOEADGEL },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "DBPCMPCLOCI",
            |m: &ACPDAGMPCPC| { &m.DBPCMPCLOCI },
            |m: &mut ACPDAGMPCPC| { &mut m.DBPCMPCLOCI },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<ACPDAGMPCPC>(
            "ACPDAGMPCPC",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for ACPDAGMPCPC {
    const NAME: &'static str = "ACPDAGMPCPC";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                56 => {
                    self.NEJJEFEOJOJ = is.read_enum_or_unknown()?;
                },
                80 => {
                    self.level = is.read_uint32()?;
                },
                88 => {
                    self.EIFICNPAOIO = is.read_uint32()?;
                },
                120 => {
                    self.uid = is.read_uint32()?;
                },
                26 => {
                    self.KDLEHAPNACL = is.read_string()?;
                },
                106 => {
                    self.HFHGPLMEFGJ = is.read_string()?;
                },
                72 => {
                    self.MHCKOEADGEL = is.read_uint32()?;
                },
                18 => {
                    self.DBPCMPCLOCI = is.read_string()?;
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
        if self.NEJJEFEOJOJ != ::protobuf::EnumOrUnknown::new(super::BPOEMOLJCCE::BPOEMOLJCCE::EDITOR) {
            my_size += ::protobuf::rt::int32_size(7, self.NEJJEFEOJOJ.value());
        }
        if self.level != 0 {
            my_size += ::protobuf::rt::uint32_size(10, self.level);
        }
        if self.EIFICNPAOIO != 0 {
            my_size += ::protobuf::rt::uint32_size(11, self.EIFICNPAOIO);
        }
        if self.uid != 0 {
            my_size += ::protobuf::rt::uint32_size(15, self.uid);
        }
        if !self.KDLEHAPNACL.is_empty() {
            my_size += ::protobuf::rt::string_size(3, &self.KDLEHAPNACL);
        }
        if !self.HFHGPLMEFGJ.is_empty() {
            my_size += ::protobuf::rt::string_size(13, &self.HFHGPLMEFGJ);
        }
        if self.MHCKOEADGEL != 0 {
            my_size += ::protobuf::rt::uint32_size(9, self.MHCKOEADGEL);
        }
        if !self.DBPCMPCLOCI.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.DBPCMPCLOCI);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.NEJJEFEOJOJ != ::protobuf::EnumOrUnknown::new(super::BPOEMOLJCCE::BPOEMOLJCCE::EDITOR) {
            os.write_enum(7, ::protobuf::EnumOrUnknown::value(&self.NEJJEFEOJOJ))?;
        }
        if self.level != 0 {
            os.write_uint32(10, self.level)?;
        }
        if self.EIFICNPAOIO != 0 {
            os.write_uint32(11, self.EIFICNPAOIO)?;
        }
        if self.uid != 0 {
            os.write_uint32(15, self.uid)?;
        }
        if !self.KDLEHAPNACL.is_empty() {
            os.write_string(3, &self.KDLEHAPNACL)?;
        }
        if !self.HFHGPLMEFGJ.is_empty() {
            os.write_string(13, &self.HFHGPLMEFGJ)?;
        }
        if self.MHCKOEADGEL != 0 {
            os.write_uint32(9, self.MHCKOEADGEL)?;
        }
        if !self.DBPCMPCLOCI.is_empty() {
            os.write_string(2, &self.DBPCMPCLOCI)?;
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

    fn new() -> ACPDAGMPCPC {
        ACPDAGMPCPC::new()
    }

    fn clear(&mut self) {
        self.NEJJEFEOJOJ = ::protobuf::EnumOrUnknown::new(super::BPOEMOLJCCE::BPOEMOLJCCE::EDITOR);
        self.level = 0;
        self.EIFICNPAOIO = 0;
        self.uid = 0;
        self.KDLEHAPNACL.clear();
        self.HFHGPLMEFGJ.clear();
        self.MHCKOEADGEL = 0;
        self.DBPCMPCLOCI.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static ACPDAGMPCPC {
        static instance: ACPDAGMPCPC = ACPDAGMPCPC {
            NEJJEFEOJOJ: ::protobuf::EnumOrUnknown::from_i32(0),
            level: 0,
            EIFICNPAOIO: 0,
            uid: 0,
            KDLEHAPNACL: ::std::string::String::new(),
            HFHGPLMEFGJ: ::std::string::String::new(),
            MHCKOEADGEL: 0,
            DBPCMPCLOCI: ::std::string::String::new(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for ACPDAGMPCPC {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("ACPDAGMPCPC").unwrap()).clone()
    }
}

impl ::std::fmt::Display for ACPDAGMPCPC {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ACPDAGMPCPC {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11ACPDAGMPCPC.proto\x1a\x11BPOEMOLJCCE.proto\"\x8f\x02\n\x0bACPDAGMP\
    CPC\x12.\n\x0bNEJJEFEOJOJ\x18\x07\x20\x01(\x0e2\x0c.BPOEMOLJCCER\x0bNEJJ\
    EFEOJOJ\x12\x14\n\x05level\x18\n\x20\x01(\rR\x05level\x12\x20\n\x0bEIFIC\
    NPAOIO\x18\x0b\x20\x01(\rR\x0bEIFICNPAOIO\x12\x10\n\x03uid\x18\x0f\x20\
    \x01(\rR\x03uid\x12\x20\n\x0bKDLEHAPNACL\x18\x03\x20\x01(\tR\x0bKDLEHAPN\
    ACL\x12\x20\n\x0bHFHGPLMEFGJ\x18\r\x20\x01(\tR\x0bHFHGPLMEFGJ\x12\x20\n\
    \x0bMHCKOEADGEL\x18\t\x20\x01(\rR\x0bMHCKOEADGEL\x12\x20\n\x0bDBPCMPCLOC\
    I\x18\x02\x20\x01(\tR\x0bDBPCMPCLOCIb\x06proto3\
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
            deps.push(super::BPOEMOLJCCE::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(ACPDAGMPCPC::generated_message_descriptor_data());
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
