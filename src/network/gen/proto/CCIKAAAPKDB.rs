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

//! Generated file from `CCIKAAAPKDB.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:CCIKAAAPKDB)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct CCIKAAAPKDB {
    // message fields
    // @@protoc_insertion_point(field:CCIKAAAPKDB.DJLKCHKMNMI)
    pub DJLKCHKMNMI: u32,
    // @@protoc_insertion_point(field:CCIKAAAPKDB.NGOMALGCJNF)
    pub NGOMALGCJNF: bool,
    // @@protoc_insertion_point(field:CCIKAAAPKDB.status)
    pub status: ::protobuf::EnumOrUnknown<super::TrainVisitorStatus::TrainVisitorStatus>,
    // @@protoc_insertion_point(field:CCIKAAAPKDB.ECDLFEANJMA)
    pub ECDLFEANJMA: ::std::vec::Vec<u32>,
    // @@protoc_insertion_point(field:CCIKAAAPKDB.ABPLNPMNDAD)
    pub ABPLNPMNDAD: u32,
    // special fields
    // @@protoc_insertion_point(special_field:CCIKAAAPKDB.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a CCIKAAAPKDB {
    fn default() -> &'a CCIKAAAPKDB {
        <CCIKAAAPKDB as ::protobuf::Message>::default_instance()
    }
}

impl CCIKAAAPKDB {
    pub fn new() -> CCIKAAAPKDB {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(5);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "DJLKCHKMNMI",
            |m: &CCIKAAAPKDB| { &m.DJLKCHKMNMI },
            |m: &mut CCIKAAAPKDB| { &mut m.DJLKCHKMNMI },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "NGOMALGCJNF",
            |m: &CCIKAAAPKDB| { &m.NGOMALGCJNF },
            |m: &mut CCIKAAAPKDB| { &mut m.NGOMALGCJNF },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "status",
            |m: &CCIKAAAPKDB| { &m.status },
            |m: &mut CCIKAAAPKDB| { &mut m.status },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "ECDLFEANJMA",
            |m: &CCIKAAAPKDB| { &m.ECDLFEANJMA },
            |m: &mut CCIKAAAPKDB| { &mut m.ECDLFEANJMA },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "ABPLNPMNDAD",
            |m: &CCIKAAAPKDB| { &m.ABPLNPMNDAD },
            |m: &mut CCIKAAAPKDB| { &mut m.ABPLNPMNDAD },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<CCIKAAAPKDB>(
            "CCIKAAAPKDB",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for CCIKAAAPKDB {
    const NAME: &'static str = "CCIKAAAPKDB";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                40 => {
                    self.DJLKCHKMNMI = is.read_uint32()?;
                },
                96 => {
                    self.NGOMALGCJNF = is.read_bool()?;
                },
                8 => {
                    self.status = is.read_enum_or_unknown()?;
                },
                58 => {
                    is.read_repeated_packed_uint32_into(&mut self.ECDLFEANJMA)?;
                },
                56 => {
                    self.ECDLFEANJMA.push(is.read_uint32()?);
                },
                16 => {
                    self.ABPLNPMNDAD = is.read_uint32()?;
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
        if self.DJLKCHKMNMI != 0 {
            my_size += ::protobuf::rt::uint32_size(5, self.DJLKCHKMNMI);
        }
        if self.NGOMALGCJNF != false {
            my_size += 1 + 1;
        }
        if self.status != ::protobuf::EnumOrUnknown::new(super::TrainVisitorStatus::TrainVisitorStatus::TRAIN_VISITOR_STATUS_NONE) {
            my_size += ::protobuf::rt::int32_size(1, self.status.value());
        }
        for value in &self.ECDLFEANJMA {
            my_size += ::protobuf::rt::uint32_size(7, *value);
        };
        if self.ABPLNPMNDAD != 0 {
            my_size += ::protobuf::rt::uint32_size(2, self.ABPLNPMNDAD);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.DJLKCHKMNMI != 0 {
            os.write_uint32(5, self.DJLKCHKMNMI)?;
        }
        if self.NGOMALGCJNF != false {
            os.write_bool(12, self.NGOMALGCJNF)?;
        }
        if self.status != ::protobuf::EnumOrUnknown::new(super::TrainVisitorStatus::TrainVisitorStatus::TRAIN_VISITOR_STATUS_NONE) {
            os.write_enum(1, ::protobuf::EnumOrUnknown::value(&self.status))?;
        }
        for v in &self.ECDLFEANJMA {
            os.write_uint32(7, *v)?;
        };
        if self.ABPLNPMNDAD != 0 {
            os.write_uint32(2, self.ABPLNPMNDAD)?;
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

    fn new() -> CCIKAAAPKDB {
        CCIKAAAPKDB::new()
    }

    fn clear(&mut self) {
        self.DJLKCHKMNMI = 0;
        self.NGOMALGCJNF = false;
        self.status = ::protobuf::EnumOrUnknown::new(super::TrainVisitorStatus::TrainVisitorStatus::TRAIN_VISITOR_STATUS_NONE);
        self.ECDLFEANJMA.clear();
        self.ABPLNPMNDAD = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static CCIKAAAPKDB {
        static instance: CCIKAAAPKDB = CCIKAAAPKDB {
            DJLKCHKMNMI: 0,
            NGOMALGCJNF: false,
            status: ::protobuf::EnumOrUnknown::from_i32(0),
            ECDLFEANJMA: ::std::vec::Vec::new(),
            ABPLNPMNDAD: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for CCIKAAAPKDB {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("CCIKAAAPKDB").unwrap()).clone()
    }
}

impl ::std::fmt::Display for CCIKAAAPKDB {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CCIKAAAPKDB {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11CCIKAAAPKDB.proto\x1a\x18TrainVisitorStatus.proto\"\xc2\x01\n\x0bC\
    CIKAAAPKDB\x12\x20\n\x0bDJLKCHKMNMI\x18\x05\x20\x01(\rR\x0bDJLKCHKMNMI\
    \x12\x20\n\x0bNGOMALGCJNF\x18\x0c\x20\x01(\x08R\x0bNGOMALGCJNF\x12+\n\
    \x06status\x18\x01\x20\x01(\x0e2\x13.TrainVisitorStatusR\x06status\x12\
    \x20\n\x0bECDLFEANJMA\x18\x07\x20\x03(\rR\x0bECDLFEANJMA\x12\x20\n\x0bAB\
    PLNPMNDAD\x18\x02\x20\x01(\rR\x0bABPLNPMNDADb\x06proto3\
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
            deps.push(super::TrainVisitorStatus::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(CCIKAAAPKDB::generated_message_descriptor_data());
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
