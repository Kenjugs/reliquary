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

//! Generated file from `OPEMMBHKCFD.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:OPEMMBHKCFD)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct OPEMMBHKCFD {
    // message fields
    // @@protoc_insertion_point(field:OPEMMBHKCFD.IBAJGALJKHJ)
    pub IBAJGALJKHJ: u32,
    // @@protoc_insertion_point(field:OPEMMBHKCFD.DBMFHIMKNCC)
    pub DBMFHIMKNCC: u32,
    // @@protoc_insertion_point(field:OPEMMBHKCFD.DAAIBKIKBEJ)
    pub DAAIBKIKBEJ: u32,
    // @@protoc_insertion_point(field:OPEMMBHKCFD.IDMAMIMFCPG)
    pub IDMAMIMFCPG: u32,
    // @@protoc_insertion_point(field:OPEMMBHKCFD.DPAAMCBCKDA)
    pub DPAAMCBCKDA: ::std::vec::Vec<u32>,
    // @@protoc_insertion_point(field:OPEMMBHKCFD.LELABBBOIKN)
    pub LELABBBOIKN: ::protobuf::EnumOrUnknown<super::MuseumRandomEventState::MuseumRandomEventState>,
    // @@protoc_insertion_point(field:OPEMMBHKCFD.APOELPDBOHG)
    pub APOELPDBOHG: u32,
    // special fields
    // @@protoc_insertion_point(special_field:OPEMMBHKCFD.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a OPEMMBHKCFD {
    fn default() -> &'a OPEMMBHKCFD {
        <OPEMMBHKCFD as ::protobuf::Message>::default_instance()
    }
}

impl OPEMMBHKCFD {
    pub fn new() -> OPEMMBHKCFD {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(7);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "IBAJGALJKHJ",
            |m: &OPEMMBHKCFD| { &m.IBAJGALJKHJ },
            |m: &mut OPEMMBHKCFD| { &mut m.IBAJGALJKHJ },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "DBMFHIMKNCC",
            |m: &OPEMMBHKCFD| { &m.DBMFHIMKNCC },
            |m: &mut OPEMMBHKCFD| { &mut m.DBMFHIMKNCC },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "DAAIBKIKBEJ",
            |m: &OPEMMBHKCFD| { &m.DAAIBKIKBEJ },
            |m: &mut OPEMMBHKCFD| { &mut m.DAAIBKIKBEJ },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "IDMAMIMFCPG",
            |m: &OPEMMBHKCFD| { &m.IDMAMIMFCPG },
            |m: &mut OPEMMBHKCFD| { &mut m.IDMAMIMFCPG },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "DPAAMCBCKDA",
            |m: &OPEMMBHKCFD| { &m.DPAAMCBCKDA },
            |m: &mut OPEMMBHKCFD| { &mut m.DPAAMCBCKDA },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "LELABBBOIKN",
            |m: &OPEMMBHKCFD| { &m.LELABBBOIKN },
            |m: &mut OPEMMBHKCFD| { &mut m.LELABBBOIKN },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "APOELPDBOHG",
            |m: &OPEMMBHKCFD| { &m.APOELPDBOHG },
            |m: &mut OPEMMBHKCFD| { &mut m.APOELPDBOHG },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<OPEMMBHKCFD>(
            "OPEMMBHKCFD",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for OPEMMBHKCFD {
    const NAME: &'static str = "OPEMMBHKCFD";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                48 => {
                    self.IBAJGALJKHJ = is.read_uint32()?;
                },
                88 => {
                    self.DBMFHIMKNCC = is.read_uint32()?;
                },
                72 => {
                    self.DAAIBKIKBEJ = is.read_uint32()?;
                },
                96 => {
                    self.IDMAMIMFCPG = is.read_uint32()?;
                },
                10 => {
                    is.read_repeated_packed_uint32_into(&mut self.DPAAMCBCKDA)?;
                },
                8 => {
                    self.DPAAMCBCKDA.push(is.read_uint32()?);
                },
                32 => {
                    self.LELABBBOIKN = is.read_enum_or_unknown()?;
                },
                120 => {
                    self.APOELPDBOHG = is.read_uint32()?;
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
        if self.IBAJGALJKHJ != 0 {
            my_size += ::protobuf::rt::uint32_size(6, self.IBAJGALJKHJ);
        }
        if self.DBMFHIMKNCC != 0 {
            my_size += ::protobuf::rt::uint32_size(11, self.DBMFHIMKNCC);
        }
        if self.DAAIBKIKBEJ != 0 {
            my_size += ::protobuf::rt::uint32_size(9, self.DAAIBKIKBEJ);
        }
        if self.IDMAMIMFCPG != 0 {
            my_size += ::protobuf::rt::uint32_size(12, self.IDMAMIMFCPG);
        }
        for value in &self.DPAAMCBCKDA {
            my_size += ::protobuf::rt::uint32_size(1, *value);
        };
        if self.LELABBBOIKN != ::protobuf::EnumOrUnknown::new(super::MuseumRandomEventState::MuseumRandomEventState::MUSEUM_RANDOM_EVENT_STATE_NONE) {
            my_size += ::protobuf::rt::int32_size(4, self.LELABBBOIKN.value());
        }
        if self.APOELPDBOHG != 0 {
            my_size += ::protobuf::rt::uint32_size(15, self.APOELPDBOHG);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.IBAJGALJKHJ != 0 {
            os.write_uint32(6, self.IBAJGALJKHJ)?;
        }
        if self.DBMFHIMKNCC != 0 {
            os.write_uint32(11, self.DBMFHIMKNCC)?;
        }
        if self.DAAIBKIKBEJ != 0 {
            os.write_uint32(9, self.DAAIBKIKBEJ)?;
        }
        if self.IDMAMIMFCPG != 0 {
            os.write_uint32(12, self.IDMAMIMFCPG)?;
        }
        for v in &self.DPAAMCBCKDA {
            os.write_uint32(1, *v)?;
        };
        if self.LELABBBOIKN != ::protobuf::EnumOrUnknown::new(super::MuseumRandomEventState::MuseumRandomEventState::MUSEUM_RANDOM_EVENT_STATE_NONE) {
            os.write_enum(4, ::protobuf::EnumOrUnknown::value(&self.LELABBBOIKN))?;
        }
        if self.APOELPDBOHG != 0 {
            os.write_uint32(15, self.APOELPDBOHG)?;
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

    fn new() -> OPEMMBHKCFD {
        OPEMMBHKCFD::new()
    }

    fn clear(&mut self) {
        self.IBAJGALJKHJ = 0;
        self.DBMFHIMKNCC = 0;
        self.DAAIBKIKBEJ = 0;
        self.IDMAMIMFCPG = 0;
        self.DPAAMCBCKDA.clear();
        self.LELABBBOIKN = ::protobuf::EnumOrUnknown::new(super::MuseumRandomEventState::MuseumRandomEventState::MUSEUM_RANDOM_EVENT_STATE_NONE);
        self.APOELPDBOHG = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static OPEMMBHKCFD {
        static instance: OPEMMBHKCFD = OPEMMBHKCFD {
            IBAJGALJKHJ: 0,
            DBMFHIMKNCC: 0,
            DAAIBKIKBEJ: 0,
            IDMAMIMFCPG: 0,
            DPAAMCBCKDA: ::std::vec::Vec::new(),
            LELABBBOIKN: ::protobuf::EnumOrUnknown::from_i32(0),
            APOELPDBOHG: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for OPEMMBHKCFD {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("OPEMMBHKCFD").unwrap()).clone()
    }
}

impl ::std::fmt::Display for OPEMMBHKCFD {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for OPEMMBHKCFD {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11OPEMMBHKCFD.proto\x1a\x1cMuseumRandomEventState.proto\"\x94\x02\n\
    \x0bOPEMMBHKCFD\x12\x20\n\x0bIBAJGALJKHJ\x18\x06\x20\x01(\rR\x0bIBAJGALJ\
    KHJ\x12\x20\n\x0bDBMFHIMKNCC\x18\x0b\x20\x01(\rR\x0bDBMFHIMKNCC\x12\x20\
    \n\x0bDAAIBKIKBEJ\x18\t\x20\x01(\rR\x0bDAAIBKIKBEJ\x12\x20\n\x0bIDMAMIMF\
    CPG\x18\x0c\x20\x01(\rR\x0bIDMAMIMFCPG\x12\x20\n\x0bDPAAMCBCKDA\x18\x01\
    \x20\x03(\rR\x0bDPAAMCBCKDA\x129\n\x0bLELABBBOIKN\x18\x04\x20\x01(\x0e2\
    \x17.MuseumRandomEventStateR\x0bLELABBBOIKN\x12\x20\n\x0bAPOELPDBOHG\x18\
    \x0f\x20\x01(\rR\x0bAPOELPDBOHGb\x06proto3\
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
            deps.push(super::MuseumRandomEventState::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(OPEMMBHKCFD::generated_message_descriptor_data());
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
