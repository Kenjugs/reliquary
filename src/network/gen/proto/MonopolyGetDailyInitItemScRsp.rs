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

//! Generated file from `MonopolyGetDailyInitItemScRsp.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:MonopolyGetDailyInitItemScRsp)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct MonopolyGetDailyInitItemScRsp {
    // message fields
    // @@protoc_insertion_point(field:MonopolyGetDailyInitItemScRsp.LBMEIHABLKN)
    pub LBMEIHABLKN: u32,
    // @@protoc_insertion_point(field:MonopolyGetDailyInitItemScRsp.FBMLKJJMMFJ)
    pub FBMLKJJMMFJ: u32,
    // @@protoc_insertion_point(field:MonopolyGetDailyInitItemScRsp.DBABLKNONJF)
    pub DBABLKNONJF: u32,
    // @@protoc_insertion_point(field:MonopolyGetDailyInitItemScRsp.AABJAEIBGGM)
    pub AABJAEIBGGM: i64,
    // @@protoc_insertion_point(field:MonopolyGetDailyInitItemScRsp.OBIDNHGHHMH)
    pub OBIDNHGHHMH: u32,
    // @@protoc_insertion_point(field:MonopolyGetDailyInitItemScRsp.retcode)
    pub retcode: u32,
    // @@protoc_insertion_point(field:MonopolyGetDailyInitItemScRsp.GELALEKMCGH)
    pub GELALEKMCGH: u32,
    // special fields
    // @@protoc_insertion_point(special_field:MonopolyGetDailyInitItemScRsp.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a MonopolyGetDailyInitItemScRsp {
    fn default() -> &'a MonopolyGetDailyInitItemScRsp {
        <MonopolyGetDailyInitItemScRsp as ::protobuf::Message>::default_instance()
    }
}

impl MonopolyGetDailyInitItemScRsp {
    pub fn new() -> MonopolyGetDailyInitItemScRsp {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(7);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "LBMEIHABLKN",
            |m: &MonopolyGetDailyInitItemScRsp| { &m.LBMEIHABLKN },
            |m: &mut MonopolyGetDailyInitItemScRsp| { &mut m.LBMEIHABLKN },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "FBMLKJJMMFJ",
            |m: &MonopolyGetDailyInitItemScRsp| { &m.FBMLKJJMMFJ },
            |m: &mut MonopolyGetDailyInitItemScRsp| { &mut m.FBMLKJJMMFJ },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "DBABLKNONJF",
            |m: &MonopolyGetDailyInitItemScRsp| { &m.DBABLKNONJF },
            |m: &mut MonopolyGetDailyInitItemScRsp| { &mut m.DBABLKNONJF },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "AABJAEIBGGM",
            |m: &MonopolyGetDailyInitItemScRsp| { &m.AABJAEIBGGM },
            |m: &mut MonopolyGetDailyInitItemScRsp| { &mut m.AABJAEIBGGM },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "OBIDNHGHHMH",
            |m: &MonopolyGetDailyInitItemScRsp| { &m.OBIDNHGHHMH },
            |m: &mut MonopolyGetDailyInitItemScRsp| { &mut m.OBIDNHGHHMH },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "retcode",
            |m: &MonopolyGetDailyInitItemScRsp| { &m.retcode },
            |m: &mut MonopolyGetDailyInitItemScRsp| { &mut m.retcode },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "GELALEKMCGH",
            |m: &MonopolyGetDailyInitItemScRsp| { &m.GELALEKMCGH },
            |m: &mut MonopolyGetDailyInitItemScRsp| { &mut m.GELALEKMCGH },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<MonopolyGetDailyInitItemScRsp>(
            "MonopolyGetDailyInitItemScRsp",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for MonopolyGetDailyInitItemScRsp {
    const NAME: &'static str = "MonopolyGetDailyInitItemScRsp";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                64 => {
                    self.LBMEIHABLKN = is.read_uint32()?;
                },
                16 => {
                    self.FBMLKJJMMFJ = is.read_uint32()?;
                },
                120 => {
                    self.DBABLKNONJF = is.read_uint32()?;
                },
                112 => {
                    self.AABJAEIBGGM = is.read_int64()?;
                },
                32 => {
                    self.OBIDNHGHHMH = is.read_uint32()?;
                },
                24 => {
                    self.retcode = is.read_uint32()?;
                },
                40 => {
                    self.GELALEKMCGH = is.read_uint32()?;
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
        if self.LBMEIHABLKN != 0 {
            my_size += ::protobuf::rt::uint32_size(8, self.LBMEIHABLKN);
        }
        if self.FBMLKJJMMFJ != 0 {
            my_size += ::protobuf::rt::uint32_size(2, self.FBMLKJJMMFJ);
        }
        if self.DBABLKNONJF != 0 {
            my_size += ::protobuf::rt::uint32_size(15, self.DBABLKNONJF);
        }
        if self.AABJAEIBGGM != 0 {
            my_size += ::protobuf::rt::int64_size(14, self.AABJAEIBGGM);
        }
        if self.OBIDNHGHHMH != 0 {
            my_size += ::protobuf::rt::uint32_size(4, self.OBIDNHGHHMH);
        }
        if self.retcode != 0 {
            my_size += ::protobuf::rt::uint32_size(3, self.retcode);
        }
        if self.GELALEKMCGH != 0 {
            my_size += ::protobuf::rt::uint32_size(5, self.GELALEKMCGH);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.LBMEIHABLKN != 0 {
            os.write_uint32(8, self.LBMEIHABLKN)?;
        }
        if self.FBMLKJJMMFJ != 0 {
            os.write_uint32(2, self.FBMLKJJMMFJ)?;
        }
        if self.DBABLKNONJF != 0 {
            os.write_uint32(15, self.DBABLKNONJF)?;
        }
        if self.AABJAEIBGGM != 0 {
            os.write_int64(14, self.AABJAEIBGGM)?;
        }
        if self.OBIDNHGHHMH != 0 {
            os.write_uint32(4, self.OBIDNHGHHMH)?;
        }
        if self.retcode != 0 {
            os.write_uint32(3, self.retcode)?;
        }
        if self.GELALEKMCGH != 0 {
            os.write_uint32(5, self.GELALEKMCGH)?;
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

    fn new() -> MonopolyGetDailyInitItemScRsp {
        MonopolyGetDailyInitItemScRsp::new()
    }

    fn clear(&mut self) {
        self.LBMEIHABLKN = 0;
        self.FBMLKJJMMFJ = 0;
        self.DBABLKNONJF = 0;
        self.AABJAEIBGGM = 0;
        self.OBIDNHGHHMH = 0;
        self.retcode = 0;
        self.GELALEKMCGH = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static MonopolyGetDailyInitItemScRsp {
        static instance: MonopolyGetDailyInitItemScRsp = MonopolyGetDailyInitItemScRsp {
            LBMEIHABLKN: 0,
            FBMLKJJMMFJ: 0,
            DBABLKNONJF: 0,
            AABJAEIBGGM: 0,
            OBIDNHGHHMH: 0,
            retcode: 0,
            GELALEKMCGH: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for MonopolyGetDailyInitItemScRsp {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("MonopolyGetDailyInitItemScRsp").unwrap()).clone()
    }
}

impl ::std::fmt::Display for MonopolyGetDailyInitItemScRsp {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for MonopolyGetDailyInitItemScRsp {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n#MonopolyGetDailyInitItemScRsp.proto\"\x85\x02\n\x1dMonopolyGetDailyIn\
    itItemScRsp\x12\x20\n\x0bLBMEIHABLKN\x18\x08\x20\x01(\rR\x0bLBMEIHABLKN\
    \x12\x20\n\x0bFBMLKJJMMFJ\x18\x02\x20\x01(\rR\x0bFBMLKJJMMFJ\x12\x20\n\
    \x0bDBABLKNONJF\x18\x0f\x20\x01(\rR\x0bDBABLKNONJF\x12\x20\n\x0bAABJAEIB\
    GGM\x18\x0e\x20\x01(\x03R\x0bAABJAEIBGGM\x12\x20\n\x0bOBIDNHGHHMH\x18\
    \x04\x20\x01(\rR\x0bOBIDNHGHHMH\x12\x18\n\x07retcode\x18\x03\x20\x01(\rR\
    \x07retcode\x12\x20\n\x0bGELALEKMCGH\x18\x05\x20\x01(\rR\x0bGELALEKMCGHb\
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
            messages.push(MonopolyGetDailyInitItemScRsp::generated_message_descriptor_data());
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
