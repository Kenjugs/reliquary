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

//! Generated file from `FightMatch3SwapCsReq.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:FightMatch3SwapCsReq)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct FightMatch3SwapCsReq {
    // message fields
    // @@protoc_insertion_point(field:FightMatch3SwapCsReq.GMMHLEKIJHP)
    pub GMMHLEKIJHP: ::std::vec::Vec<super::KEKABJPICIP::KEKABJPICIP>,
    // @@protoc_insertion_point(field:FightMatch3SwapCsReq.NGGOBIKLHLC)
    pub NGGOBIKLHLC: ::protobuf::MessageField<super::MDLLCLADJCP::MDLLCLADJCP>,
    // @@protoc_insertion_point(field:FightMatch3SwapCsReq.JJHAOGEHFEN)
    pub JJHAOGEHFEN: ::protobuf::MessageField<super::MDLLCLADJCP::MDLLCLADJCP>,
    // @@protoc_insertion_point(field:FightMatch3SwapCsReq.PPBIIDKNIDA)
    pub PPBIIDKNIDA: u32,
    // special fields
    // @@protoc_insertion_point(special_field:FightMatch3SwapCsReq.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a FightMatch3SwapCsReq {
    fn default() -> &'a FightMatch3SwapCsReq {
        <FightMatch3SwapCsReq as ::protobuf::Message>::default_instance()
    }
}

impl FightMatch3SwapCsReq {
    pub fn new() -> FightMatch3SwapCsReq {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(4);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "GMMHLEKIJHP",
            |m: &FightMatch3SwapCsReq| { &m.GMMHLEKIJHP },
            |m: &mut FightMatch3SwapCsReq| { &mut m.GMMHLEKIJHP },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::MDLLCLADJCP::MDLLCLADJCP>(
            "NGGOBIKLHLC",
            |m: &FightMatch3SwapCsReq| { &m.NGGOBIKLHLC },
            |m: &mut FightMatch3SwapCsReq| { &mut m.NGGOBIKLHLC },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::MDLLCLADJCP::MDLLCLADJCP>(
            "JJHAOGEHFEN",
            |m: &FightMatch3SwapCsReq| { &m.JJHAOGEHFEN },
            |m: &mut FightMatch3SwapCsReq| { &mut m.JJHAOGEHFEN },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "PPBIIDKNIDA",
            |m: &FightMatch3SwapCsReq| { &m.PPBIIDKNIDA },
            |m: &mut FightMatch3SwapCsReq| { &mut m.PPBIIDKNIDA },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<FightMatch3SwapCsReq>(
            "FightMatch3SwapCsReq",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for FightMatch3SwapCsReq {
    const NAME: &'static str = "FightMatch3SwapCsReq";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                82 => {
                    self.GMMHLEKIJHP.push(is.read_message()?);
                },
                122 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.NGGOBIKLHLC)?;
                },
                66 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.JJHAOGEHFEN)?;
                },
                88 => {
                    self.PPBIIDKNIDA = is.read_uint32()?;
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
        for value in &self.GMMHLEKIJHP {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        if let Some(v) = self.NGGOBIKLHLC.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if let Some(v) = self.JJHAOGEHFEN.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if self.PPBIIDKNIDA != 0 {
            my_size += ::protobuf::rt::uint32_size(11, self.PPBIIDKNIDA);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        for v in &self.GMMHLEKIJHP {
            ::protobuf::rt::write_message_field_with_cached_size(10, v, os)?;
        };
        if let Some(v) = self.NGGOBIKLHLC.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(15, v, os)?;
        }
        if let Some(v) = self.JJHAOGEHFEN.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(8, v, os)?;
        }
        if self.PPBIIDKNIDA != 0 {
            os.write_uint32(11, self.PPBIIDKNIDA)?;
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

    fn new() -> FightMatch3SwapCsReq {
        FightMatch3SwapCsReq::new()
    }

    fn clear(&mut self) {
        self.GMMHLEKIJHP.clear();
        self.NGGOBIKLHLC.clear();
        self.JJHAOGEHFEN.clear();
        self.PPBIIDKNIDA = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static FightMatch3SwapCsReq {
        static instance: FightMatch3SwapCsReq = FightMatch3SwapCsReq {
            GMMHLEKIJHP: ::std::vec::Vec::new(),
            NGGOBIKLHLC: ::protobuf::MessageField::none(),
            JJHAOGEHFEN: ::protobuf::MessageField::none(),
            PPBIIDKNIDA: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for FightMatch3SwapCsReq {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("FightMatch3SwapCsReq").unwrap()).clone()
    }
}

impl ::std::fmt::Display for FightMatch3SwapCsReq {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for FightMatch3SwapCsReq {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x1aFightMatch3SwapCsReq.proto\x1a\x11KEKABJPICIP.proto\x1a\x11MDLLCLA\
    DJCP.proto\"\xc8\x01\n\x14FightMatch3SwapCsReq\x12.\n\x0bGMMHLEKIJHP\x18\
    \n\x20\x03(\x0b2\x0c.KEKABJPICIPR\x0bGMMHLEKIJHP\x12.\n\x0bNGGOBIKLHLC\
    \x18\x0f\x20\x01(\x0b2\x0c.MDLLCLADJCPR\x0bNGGOBIKLHLC\x12.\n\x0bJJHAOGE\
    HFEN\x18\x08\x20\x01(\x0b2\x0c.MDLLCLADJCPR\x0bJJHAOGEHFEN\x12\x20\n\x0b\
    PPBIIDKNIDA\x18\x0b\x20\x01(\rR\x0bPPBIIDKNIDAb\x06proto3\
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
            deps.push(super::KEKABJPICIP::file_descriptor().clone());
            deps.push(super::MDLLCLADJCP::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(FightMatch3SwapCsReq::generated_message_descriptor_data());
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
