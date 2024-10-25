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

//! Generated file from `AlleyPlacingGameScRsp.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:AlleyPlacingGameScRsp)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct AlleyPlacingGameScRsp {
    // message fields
    // @@protoc_insertion_point(field:AlleyPlacingGameScRsp.IOMGNNBMCDC)
    pub IOMGNNBMCDC: u32,
    // @@protoc_insertion_point(field:AlleyPlacingGameScRsp.AKHLIBFGLBO)
    pub AKHLIBFGLBO: u32,
    // @@protoc_insertion_point(field:AlleyPlacingGameScRsp.LFNFOBHMGML)
    pub LFNFOBHMGML: u32,
    // @@protoc_insertion_point(field:AlleyPlacingGameScRsp.HAECOJINNDC)
    pub HAECOJINNDC: u32,
    // @@protoc_insertion_point(field:AlleyPlacingGameScRsp.LOHNCLANBBI)
    pub LOHNCLANBBI: u32,
    // @@protoc_insertion_point(field:AlleyPlacingGameScRsp.DAAIBKIKBEJ)
    pub DAAIBKIKBEJ: u32,
    // @@protoc_insertion_point(field:AlleyPlacingGameScRsp.retcode)
    pub retcode: u32,
    // @@protoc_insertion_point(field:AlleyPlacingGameScRsp.FELCJEININH)
    pub FELCJEININH: u32,
    // special fields
    // @@protoc_insertion_point(special_field:AlleyPlacingGameScRsp.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a AlleyPlacingGameScRsp {
    fn default() -> &'a AlleyPlacingGameScRsp {
        <AlleyPlacingGameScRsp as ::protobuf::Message>::default_instance()
    }
}

impl AlleyPlacingGameScRsp {
    pub fn new() -> AlleyPlacingGameScRsp {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(8);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "IOMGNNBMCDC",
            |m: &AlleyPlacingGameScRsp| { &m.IOMGNNBMCDC },
            |m: &mut AlleyPlacingGameScRsp| { &mut m.IOMGNNBMCDC },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "AKHLIBFGLBO",
            |m: &AlleyPlacingGameScRsp| { &m.AKHLIBFGLBO },
            |m: &mut AlleyPlacingGameScRsp| { &mut m.AKHLIBFGLBO },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "LFNFOBHMGML",
            |m: &AlleyPlacingGameScRsp| { &m.LFNFOBHMGML },
            |m: &mut AlleyPlacingGameScRsp| { &mut m.LFNFOBHMGML },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "HAECOJINNDC",
            |m: &AlleyPlacingGameScRsp| { &m.HAECOJINNDC },
            |m: &mut AlleyPlacingGameScRsp| { &mut m.HAECOJINNDC },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "LOHNCLANBBI",
            |m: &AlleyPlacingGameScRsp| { &m.LOHNCLANBBI },
            |m: &mut AlleyPlacingGameScRsp| { &mut m.LOHNCLANBBI },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "DAAIBKIKBEJ",
            |m: &AlleyPlacingGameScRsp| { &m.DAAIBKIKBEJ },
            |m: &mut AlleyPlacingGameScRsp| { &mut m.DAAIBKIKBEJ },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "retcode",
            |m: &AlleyPlacingGameScRsp| { &m.retcode },
            |m: &mut AlleyPlacingGameScRsp| { &mut m.retcode },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "FELCJEININH",
            |m: &AlleyPlacingGameScRsp| { &m.FELCJEININH },
            |m: &mut AlleyPlacingGameScRsp| { &mut m.FELCJEININH },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<AlleyPlacingGameScRsp>(
            "AlleyPlacingGameScRsp",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for AlleyPlacingGameScRsp {
    const NAME: &'static str = "AlleyPlacingGameScRsp";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                88 => {
                    self.IOMGNNBMCDC = is.read_uint32()?;
                },
                8 => {
                    self.AKHLIBFGLBO = is.read_uint32()?;
                },
                24 => {
                    self.LFNFOBHMGML = is.read_uint32()?;
                },
                112 => {
                    self.HAECOJINNDC = is.read_uint32()?;
                },
                16 => {
                    self.LOHNCLANBBI = is.read_uint32()?;
                },
                96 => {
                    self.DAAIBKIKBEJ = is.read_uint32()?;
                },
                72 => {
                    self.retcode = is.read_uint32()?;
                },
                80 => {
                    self.FELCJEININH = is.read_uint32()?;
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
        if self.IOMGNNBMCDC != 0 {
            my_size += ::protobuf::rt::uint32_size(11, self.IOMGNNBMCDC);
        }
        if self.AKHLIBFGLBO != 0 {
            my_size += ::protobuf::rt::uint32_size(1, self.AKHLIBFGLBO);
        }
        if self.LFNFOBHMGML != 0 {
            my_size += ::protobuf::rt::uint32_size(3, self.LFNFOBHMGML);
        }
        if self.HAECOJINNDC != 0 {
            my_size += ::protobuf::rt::uint32_size(14, self.HAECOJINNDC);
        }
        if self.LOHNCLANBBI != 0 {
            my_size += ::protobuf::rt::uint32_size(2, self.LOHNCLANBBI);
        }
        if self.DAAIBKIKBEJ != 0 {
            my_size += ::protobuf::rt::uint32_size(12, self.DAAIBKIKBEJ);
        }
        if self.retcode != 0 {
            my_size += ::protobuf::rt::uint32_size(9, self.retcode);
        }
        if self.FELCJEININH != 0 {
            my_size += ::protobuf::rt::uint32_size(10, self.FELCJEININH);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.IOMGNNBMCDC != 0 {
            os.write_uint32(11, self.IOMGNNBMCDC)?;
        }
        if self.AKHLIBFGLBO != 0 {
            os.write_uint32(1, self.AKHLIBFGLBO)?;
        }
        if self.LFNFOBHMGML != 0 {
            os.write_uint32(3, self.LFNFOBHMGML)?;
        }
        if self.HAECOJINNDC != 0 {
            os.write_uint32(14, self.HAECOJINNDC)?;
        }
        if self.LOHNCLANBBI != 0 {
            os.write_uint32(2, self.LOHNCLANBBI)?;
        }
        if self.DAAIBKIKBEJ != 0 {
            os.write_uint32(12, self.DAAIBKIKBEJ)?;
        }
        if self.retcode != 0 {
            os.write_uint32(9, self.retcode)?;
        }
        if self.FELCJEININH != 0 {
            os.write_uint32(10, self.FELCJEININH)?;
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

    fn new() -> AlleyPlacingGameScRsp {
        AlleyPlacingGameScRsp::new()
    }

    fn clear(&mut self) {
        self.IOMGNNBMCDC = 0;
        self.AKHLIBFGLBO = 0;
        self.LFNFOBHMGML = 0;
        self.HAECOJINNDC = 0;
        self.LOHNCLANBBI = 0;
        self.DAAIBKIKBEJ = 0;
        self.retcode = 0;
        self.FELCJEININH = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static AlleyPlacingGameScRsp {
        static instance: AlleyPlacingGameScRsp = AlleyPlacingGameScRsp {
            IOMGNNBMCDC: 0,
            AKHLIBFGLBO: 0,
            LFNFOBHMGML: 0,
            HAECOJINNDC: 0,
            LOHNCLANBBI: 0,
            DAAIBKIKBEJ: 0,
            retcode: 0,
            FELCJEININH: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for AlleyPlacingGameScRsp {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("AlleyPlacingGameScRsp").unwrap()).clone()
    }
}

impl ::std::fmt::Display for AlleyPlacingGameScRsp {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for AlleyPlacingGameScRsp {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x1bAlleyPlacingGameScRsp.proto\"\x9f\x02\n\x15AlleyPlacingGameScRsp\
    \x12\x20\n\x0bIOMGNNBMCDC\x18\x0b\x20\x01(\rR\x0bIOMGNNBMCDC\x12\x20\n\
    \x0bAKHLIBFGLBO\x18\x01\x20\x01(\rR\x0bAKHLIBFGLBO\x12\x20\n\x0bLFNFOBHM\
    GML\x18\x03\x20\x01(\rR\x0bLFNFOBHMGML\x12\x20\n\x0bHAECOJINNDC\x18\x0e\
    \x20\x01(\rR\x0bHAECOJINNDC\x12\x20\n\x0bLOHNCLANBBI\x18\x02\x20\x01(\rR\
    \x0bLOHNCLANBBI\x12\x20\n\x0bDAAIBKIKBEJ\x18\x0c\x20\x01(\rR\x0bDAAIBKIK\
    BEJ\x12\x18\n\x07retcode\x18\t\x20\x01(\rR\x07retcode\x12\x20\n\x0bFELCJ\
    EININH\x18\n\x20\x01(\rR\x0bFELCJEININHb\x06proto3\
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
            messages.push(AlleyPlacingGameScRsp::generated_message_descriptor_data());
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
