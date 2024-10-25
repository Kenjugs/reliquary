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

//! Generated file from `IOBHFJEEOOP.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:IOBHFJEEOOP)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct IOBHFJEEOOP {
    // message fields
    // @@protoc_insertion_point(field:IOBHFJEEOOP.DFFMIACMHLD)
    pub DFFMIACMHLD: bool,
    // @@protoc_insertion_point(field:IOBHFJEEOOP.KFHANKAEJFJ)
    pub KFHANKAEJFJ: u32,
    // @@protoc_insertion_point(field:IOBHFJEEOOP.COKOKCJMJAI)
    pub COKOKCJMJAI: u32,
    // @@protoc_insertion_point(field:IOBHFJEEOOP.DMIEBIKLCPG)
    pub DMIEBIKLCPG: u32,
    // @@protoc_insertion_point(field:IOBHFJEEOOP.HLDLDAPNILF)
    pub HLDLDAPNILF: ::protobuf::MessageField<super::KHLGBOHOGPD::KHLGBOHOGPD>,
    // @@protoc_insertion_point(field:IOBHFJEEOOP.DDBDIGFKBIC)
    pub DDBDIGFKBIC: u32,
    // @@protoc_insertion_point(field:IOBHFJEEOOP.FILMAOEBILH)
    pub FILMAOEBILH: u32,
    // @@protoc_insertion_point(field:IOBHFJEEOOP.DBKHFAEKNKL)
    pub DBKHFAEKNKL: u32,
    // special fields
    // @@protoc_insertion_point(special_field:IOBHFJEEOOP.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a IOBHFJEEOOP {
    fn default() -> &'a IOBHFJEEOOP {
        <IOBHFJEEOOP as ::protobuf::Message>::default_instance()
    }
}

impl IOBHFJEEOOP {
    pub fn new() -> IOBHFJEEOOP {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(8);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "DFFMIACMHLD",
            |m: &IOBHFJEEOOP| { &m.DFFMIACMHLD },
            |m: &mut IOBHFJEEOOP| { &mut m.DFFMIACMHLD },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "KFHANKAEJFJ",
            |m: &IOBHFJEEOOP| { &m.KFHANKAEJFJ },
            |m: &mut IOBHFJEEOOP| { &mut m.KFHANKAEJFJ },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "COKOKCJMJAI",
            |m: &IOBHFJEEOOP| { &m.COKOKCJMJAI },
            |m: &mut IOBHFJEEOOP| { &mut m.COKOKCJMJAI },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "DMIEBIKLCPG",
            |m: &IOBHFJEEOOP| { &m.DMIEBIKLCPG },
            |m: &mut IOBHFJEEOOP| { &mut m.DMIEBIKLCPG },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::KHLGBOHOGPD::KHLGBOHOGPD>(
            "HLDLDAPNILF",
            |m: &IOBHFJEEOOP| { &m.HLDLDAPNILF },
            |m: &mut IOBHFJEEOOP| { &mut m.HLDLDAPNILF },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "DDBDIGFKBIC",
            |m: &IOBHFJEEOOP| { &m.DDBDIGFKBIC },
            |m: &mut IOBHFJEEOOP| { &mut m.DDBDIGFKBIC },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "FILMAOEBILH",
            |m: &IOBHFJEEOOP| { &m.FILMAOEBILH },
            |m: &mut IOBHFJEEOOP| { &mut m.FILMAOEBILH },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "DBKHFAEKNKL",
            |m: &IOBHFJEEOOP| { &m.DBKHFAEKNKL },
            |m: &mut IOBHFJEEOOP| { &mut m.DBKHFAEKNKL },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<IOBHFJEEOOP>(
            "IOBHFJEEOOP",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for IOBHFJEEOOP {
    const NAME: &'static str = "IOBHFJEEOOP";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                72 => {
                    self.DFFMIACMHLD = is.read_bool()?;
                },
                64 => {
                    self.KFHANKAEJFJ = is.read_uint32()?;
                },
                80 => {
                    self.COKOKCJMJAI = is.read_uint32()?;
                },
                104 => {
                    self.DMIEBIKLCPG = is.read_uint32()?;
                },
                42 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.HLDLDAPNILF)?;
                },
                56 => {
                    self.DDBDIGFKBIC = is.read_uint32()?;
                },
                96 => {
                    self.FILMAOEBILH = is.read_uint32()?;
                },
                24 => {
                    self.DBKHFAEKNKL = is.read_uint32()?;
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
        if self.DFFMIACMHLD != false {
            my_size += 1 + 1;
        }
        if self.KFHANKAEJFJ != 0 {
            my_size += ::protobuf::rt::uint32_size(8, self.KFHANKAEJFJ);
        }
        if self.COKOKCJMJAI != 0 {
            my_size += ::protobuf::rt::uint32_size(10, self.COKOKCJMJAI);
        }
        if self.DMIEBIKLCPG != 0 {
            my_size += ::protobuf::rt::uint32_size(13, self.DMIEBIKLCPG);
        }
        if let Some(v) = self.HLDLDAPNILF.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if self.DDBDIGFKBIC != 0 {
            my_size += ::protobuf::rt::uint32_size(7, self.DDBDIGFKBIC);
        }
        if self.FILMAOEBILH != 0 {
            my_size += ::protobuf::rt::uint32_size(12, self.FILMAOEBILH);
        }
        if self.DBKHFAEKNKL != 0 {
            my_size += ::protobuf::rt::uint32_size(3, self.DBKHFAEKNKL);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.DFFMIACMHLD != false {
            os.write_bool(9, self.DFFMIACMHLD)?;
        }
        if self.KFHANKAEJFJ != 0 {
            os.write_uint32(8, self.KFHANKAEJFJ)?;
        }
        if self.COKOKCJMJAI != 0 {
            os.write_uint32(10, self.COKOKCJMJAI)?;
        }
        if self.DMIEBIKLCPG != 0 {
            os.write_uint32(13, self.DMIEBIKLCPG)?;
        }
        if let Some(v) = self.HLDLDAPNILF.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(5, v, os)?;
        }
        if self.DDBDIGFKBIC != 0 {
            os.write_uint32(7, self.DDBDIGFKBIC)?;
        }
        if self.FILMAOEBILH != 0 {
            os.write_uint32(12, self.FILMAOEBILH)?;
        }
        if self.DBKHFAEKNKL != 0 {
            os.write_uint32(3, self.DBKHFAEKNKL)?;
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

    fn new() -> IOBHFJEEOOP {
        IOBHFJEEOOP::new()
    }

    fn clear(&mut self) {
        self.DFFMIACMHLD = false;
        self.KFHANKAEJFJ = 0;
        self.COKOKCJMJAI = 0;
        self.DMIEBIKLCPG = 0;
        self.HLDLDAPNILF.clear();
        self.DDBDIGFKBIC = 0;
        self.FILMAOEBILH = 0;
        self.DBKHFAEKNKL = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static IOBHFJEEOOP {
        static instance: IOBHFJEEOOP = IOBHFJEEOOP {
            DFFMIACMHLD: false,
            KFHANKAEJFJ: 0,
            COKOKCJMJAI: 0,
            DMIEBIKLCPG: 0,
            HLDLDAPNILF: ::protobuf::MessageField::none(),
            DDBDIGFKBIC: 0,
            FILMAOEBILH: 0,
            DBKHFAEKNKL: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for IOBHFJEEOOP {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("IOBHFJEEOOP").unwrap()).clone()
    }
}

impl ::std::fmt::Display for IOBHFJEEOOP {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for IOBHFJEEOOP {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11IOBHFJEEOOP.proto\x1a\x11KHLGBOHOGPD.proto\"\xab\x02\n\x0bIOBHFJEE\
    OOP\x12\x20\n\x0bDFFMIACMHLD\x18\t\x20\x01(\x08R\x0bDFFMIACMHLD\x12\x20\
    \n\x0bKFHANKAEJFJ\x18\x08\x20\x01(\rR\x0bKFHANKAEJFJ\x12\x20\n\x0bCOKOKC\
    JMJAI\x18\n\x20\x01(\rR\x0bCOKOKCJMJAI\x12\x20\n\x0bDMIEBIKLCPG\x18\r\
    \x20\x01(\rR\x0bDMIEBIKLCPG\x12.\n\x0bHLDLDAPNILF\x18\x05\x20\x01(\x0b2\
    \x0c.KHLGBOHOGPDR\x0bHLDLDAPNILF\x12\x20\n\x0bDDBDIGFKBIC\x18\x07\x20\
    \x01(\rR\x0bDDBDIGFKBIC\x12\x20\n\x0bFILMAOEBILH\x18\x0c\x20\x01(\rR\x0b\
    FILMAOEBILH\x12\x20\n\x0bDBKHFAEKNKL\x18\x03\x20\x01(\rR\x0bDBKHFAEKNKLb\
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
            deps.push(super::KHLGBOHOGPD::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(IOBHFJEEOOP::generated_message_descriptor_data());
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
