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

//! Generated file from `NDACGJLONGF.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:NDACGJLONGF)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct NDACGJLONGF {
    // message fields
    // @@protoc_insertion_point(field:NDACGJLONGF.IOKADLFHEHK)
    pub IOKADLFHEHK: ::protobuf::EnumOrUnknown<super::ExtraLineupType::ExtraLineupType>,
    // @@protoc_insertion_point(field:NDACGJLONGF.DBKHFAEKNKL)
    pub DBKHFAEKNKL: u32,
    // @@protoc_insertion_point(field:NDACGJLONGF.CMIPCBOJJIC)
    pub CMIPCBOJJIC: u32,
    // @@protoc_insertion_point(field:NDACGJLONGF.KFHANKAEJFJ)
    pub KFHANKAEJFJ: u32,
    // @@protoc_insertion_point(field:NDACGJLONGF.LBBGEEFLGNO)
    pub LBBGEEFLGNO: ::std::vec::Vec<super::INBKDCICJCE::INBKDCICJCE>,
    // @@protoc_insertion_point(field:NDACGJLONGF.FILMAOEBILH)
    pub FILMAOEBILH: u32,
    // @@protoc_insertion_point(field:NDACGJLONGF.FHGIHLGILMG)
    pub FHGIHLGILMG: u32,
    // @@protoc_insertion_point(field:NDACGJLONGF.status)
    pub status: ::protobuf::EnumOrUnknown<super::EFDKGILOGKI::EFDKGILOGKI>,
    // @@protoc_insertion_point(field:NDACGJLONGF.HLDLDAPNILF)
    pub HLDLDAPNILF: ::protobuf::MessageField<super::FCHOCBIKFLL::FCHOCBIKFLL>,
    // special fields
    // @@protoc_insertion_point(special_field:NDACGJLONGF.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a NDACGJLONGF {
    fn default() -> &'a NDACGJLONGF {
        <NDACGJLONGF as ::protobuf::Message>::default_instance()
    }
}

impl NDACGJLONGF {
    pub fn new() -> NDACGJLONGF {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(9);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "IOKADLFHEHK",
            |m: &NDACGJLONGF| { &m.IOKADLFHEHK },
            |m: &mut NDACGJLONGF| { &mut m.IOKADLFHEHK },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "DBKHFAEKNKL",
            |m: &NDACGJLONGF| { &m.DBKHFAEKNKL },
            |m: &mut NDACGJLONGF| { &mut m.DBKHFAEKNKL },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "CMIPCBOJJIC",
            |m: &NDACGJLONGF| { &m.CMIPCBOJJIC },
            |m: &mut NDACGJLONGF| { &mut m.CMIPCBOJJIC },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "KFHANKAEJFJ",
            |m: &NDACGJLONGF| { &m.KFHANKAEJFJ },
            |m: &mut NDACGJLONGF| { &mut m.KFHANKAEJFJ },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "LBBGEEFLGNO",
            |m: &NDACGJLONGF| { &m.LBBGEEFLGNO },
            |m: &mut NDACGJLONGF| { &mut m.LBBGEEFLGNO },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "FILMAOEBILH",
            |m: &NDACGJLONGF| { &m.FILMAOEBILH },
            |m: &mut NDACGJLONGF| { &mut m.FILMAOEBILH },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "FHGIHLGILMG",
            |m: &NDACGJLONGF| { &m.FHGIHLGILMG },
            |m: &mut NDACGJLONGF| { &mut m.FHGIHLGILMG },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "status",
            |m: &NDACGJLONGF| { &m.status },
            |m: &mut NDACGJLONGF| { &mut m.status },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::FCHOCBIKFLL::FCHOCBIKFLL>(
            "HLDLDAPNILF",
            |m: &NDACGJLONGF| { &m.HLDLDAPNILF },
            |m: &mut NDACGJLONGF| { &mut m.HLDLDAPNILF },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<NDACGJLONGF>(
            "NDACGJLONGF",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for NDACGJLONGF {
    const NAME: &'static str = "NDACGJLONGF";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                72 => {
                    self.IOKADLFHEHK = is.read_enum_or_unknown()?;
                },
                48 => {
                    self.DBKHFAEKNKL = is.read_uint32()?;
                },
                8 => {
                    self.CMIPCBOJJIC = is.read_uint32()?;
                },
                104 => {
                    self.KFHANKAEJFJ = is.read_uint32()?;
                },
                42 => {
                    self.LBBGEEFLGNO.push(is.read_message()?);
                },
                96 => {
                    self.FILMAOEBILH = is.read_uint32()?;
                },
                112 => {
                    self.FHGIHLGILMG = is.read_uint32()?;
                },
                64 => {
                    self.status = is.read_enum_or_unknown()?;
                },
                18 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.HLDLDAPNILF)?;
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
        if self.IOKADLFHEHK != ::protobuf::EnumOrUnknown::new(super::ExtraLineupType::ExtraLineupType::LINEUP_NONE) {
            my_size += ::protobuf::rt::int32_size(9, self.IOKADLFHEHK.value());
        }
        if self.DBKHFAEKNKL != 0 {
            my_size += ::protobuf::rt::uint32_size(6, self.DBKHFAEKNKL);
        }
        if self.CMIPCBOJJIC != 0 {
            my_size += ::protobuf::rt::uint32_size(1, self.CMIPCBOJJIC);
        }
        if self.KFHANKAEJFJ != 0 {
            my_size += ::protobuf::rt::uint32_size(13, self.KFHANKAEJFJ);
        }
        for value in &self.LBBGEEFLGNO {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        if self.FILMAOEBILH != 0 {
            my_size += ::protobuf::rt::uint32_size(12, self.FILMAOEBILH);
        }
        if self.FHGIHLGILMG != 0 {
            my_size += ::protobuf::rt::uint32_size(14, self.FHGIHLGILMG);
        }
        if self.status != ::protobuf::EnumOrUnknown::new(super::EFDKGILOGKI::EFDKGILOGKI::CHALLENGE_UNKNOWN) {
            my_size += ::protobuf::rt::int32_size(8, self.status.value());
        }
        if let Some(v) = self.HLDLDAPNILF.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.IOKADLFHEHK != ::protobuf::EnumOrUnknown::new(super::ExtraLineupType::ExtraLineupType::LINEUP_NONE) {
            os.write_enum(9, ::protobuf::EnumOrUnknown::value(&self.IOKADLFHEHK))?;
        }
        if self.DBKHFAEKNKL != 0 {
            os.write_uint32(6, self.DBKHFAEKNKL)?;
        }
        if self.CMIPCBOJJIC != 0 {
            os.write_uint32(1, self.CMIPCBOJJIC)?;
        }
        if self.KFHANKAEJFJ != 0 {
            os.write_uint32(13, self.KFHANKAEJFJ)?;
        }
        for v in &self.LBBGEEFLGNO {
            ::protobuf::rt::write_message_field_with_cached_size(5, v, os)?;
        };
        if self.FILMAOEBILH != 0 {
            os.write_uint32(12, self.FILMAOEBILH)?;
        }
        if self.FHGIHLGILMG != 0 {
            os.write_uint32(14, self.FHGIHLGILMG)?;
        }
        if self.status != ::protobuf::EnumOrUnknown::new(super::EFDKGILOGKI::EFDKGILOGKI::CHALLENGE_UNKNOWN) {
            os.write_enum(8, ::protobuf::EnumOrUnknown::value(&self.status))?;
        }
        if let Some(v) = self.HLDLDAPNILF.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(2, v, os)?;
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

    fn new() -> NDACGJLONGF {
        NDACGJLONGF::new()
    }

    fn clear(&mut self) {
        self.IOKADLFHEHK = ::protobuf::EnumOrUnknown::new(super::ExtraLineupType::ExtraLineupType::LINEUP_NONE);
        self.DBKHFAEKNKL = 0;
        self.CMIPCBOJJIC = 0;
        self.KFHANKAEJFJ = 0;
        self.LBBGEEFLGNO.clear();
        self.FILMAOEBILH = 0;
        self.FHGIHLGILMG = 0;
        self.status = ::protobuf::EnumOrUnknown::new(super::EFDKGILOGKI::EFDKGILOGKI::CHALLENGE_UNKNOWN);
        self.HLDLDAPNILF.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static NDACGJLONGF {
        static instance: NDACGJLONGF = NDACGJLONGF {
            IOKADLFHEHK: ::protobuf::EnumOrUnknown::from_i32(0),
            DBKHFAEKNKL: 0,
            CMIPCBOJJIC: 0,
            KFHANKAEJFJ: 0,
            LBBGEEFLGNO: ::std::vec::Vec::new(),
            FILMAOEBILH: 0,
            FHGIHLGILMG: 0,
            status: ::protobuf::EnumOrUnknown::from_i32(0),
            HLDLDAPNILF: ::protobuf::MessageField::none(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for NDACGJLONGF {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("NDACGJLONGF").unwrap()).clone()
    }
}

impl ::std::fmt::Display for NDACGJLONGF {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for NDACGJLONGF {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11NDACGJLONGF.proto\x1a\x11EFDKGILOGKI.proto\x1a\x15ExtraLineupType.\
    proto\x1a\x11FCHOCBIKFLL.proto\x1a\x11INBKDCICJCE.proto\"\xf1\x02\n\x0bN\
    DACGJLONGF\x122\n\x0bIOKADLFHEHK\x18\t\x20\x01(\x0e2\x10.ExtraLineupType\
    R\x0bIOKADLFHEHK\x12\x20\n\x0bDBKHFAEKNKL\x18\x06\x20\x01(\rR\x0bDBKHFAE\
    KNKL\x12\x20\n\x0bCMIPCBOJJIC\x18\x01\x20\x01(\rR\x0bCMIPCBOJJIC\x12\x20\
    \n\x0bKFHANKAEJFJ\x18\r\x20\x01(\rR\x0bKFHANKAEJFJ\x12.\n\x0bLBBGEEFLGNO\
    \x18\x05\x20\x03(\x0b2\x0c.INBKDCICJCER\x0bLBBGEEFLGNO\x12\x20\n\x0bFILM\
    AOEBILH\x18\x0c\x20\x01(\rR\x0bFILMAOEBILH\x12\x20\n\x0bFHGIHLGILMG\x18\
    \x0e\x20\x01(\rR\x0bFHGIHLGILMG\x12$\n\x06status\x18\x08\x20\x01(\x0e2\
    \x0c.EFDKGILOGKIR\x06status\x12.\n\x0bHLDLDAPNILF\x18\x02\x20\x01(\x0b2\
    \x0c.FCHOCBIKFLLR\x0bHLDLDAPNILFb\x06proto3\
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
            let mut deps = ::std::vec::Vec::with_capacity(4);
            deps.push(super::EFDKGILOGKI::file_descriptor().clone());
            deps.push(super::ExtraLineupType::file_descriptor().clone());
            deps.push(super::FCHOCBIKFLL::file_descriptor().clone());
            deps.push(super::INBKDCICJCE::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(NDACGJLONGF::generated_message_descriptor_data());
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
