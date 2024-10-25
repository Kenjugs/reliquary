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

//! Generated file from `FeverTimeActivityBattleEndScNotify.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:FeverTimeActivityBattleEndScNotify)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct FeverTimeActivityBattleEndScNotify {
    // message fields
    // @@protoc_insertion_point(field:FeverTimeActivityBattleEndScNotify.BFADHPMMODP)
    pub BFADHPMMODP: u32,
    // @@protoc_insertion_point(field:FeverTimeActivityBattleEndScNotify.id)
    pub id: u32,
    // @@protoc_insertion_point(field:FeverTimeActivityBattleEndScNotify.EGPGPJLOHIL)
    pub EGPGPJLOHIL: ::protobuf::EnumOrUnknown<super::FeverTimeBattleRank::FeverTimeBattleRank>,
    // @@protoc_insertion_point(field:FeverTimeActivityBattleEndScNotify.DHFALDJEAAN)
    pub DHFALDJEAAN: u32,
    // special fields
    // @@protoc_insertion_point(special_field:FeverTimeActivityBattleEndScNotify.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a FeverTimeActivityBattleEndScNotify {
    fn default() -> &'a FeverTimeActivityBattleEndScNotify {
        <FeverTimeActivityBattleEndScNotify as ::protobuf::Message>::default_instance()
    }
}

impl FeverTimeActivityBattleEndScNotify {
    pub fn new() -> FeverTimeActivityBattleEndScNotify {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(4);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "BFADHPMMODP",
            |m: &FeverTimeActivityBattleEndScNotify| { &m.BFADHPMMODP },
            |m: &mut FeverTimeActivityBattleEndScNotify| { &mut m.BFADHPMMODP },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "id",
            |m: &FeverTimeActivityBattleEndScNotify| { &m.id },
            |m: &mut FeverTimeActivityBattleEndScNotify| { &mut m.id },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "EGPGPJLOHIL",
            |m: &FeverTimeActivityBattleEndScNotify| { &m.EGPGPJLOHIL },
            |m: &mut FeverTimeActivityBattleEndScNotify| { &mut m.EGPGPJLOHIL },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "DHFALDJEAAN",
            |m: &FeverTimeActivityBattleEndScNotify| { &m.DHFALDJEAAN },
            |m: &mut FeverTimeActivityBattleEndScNotify| { &mut m.DHFALDJEAAN },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<FeverTimeActivityBattleEndScNotify>(
            "FeverTimeActivityBattleEndScNotify",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for FeverTimeActivityBattleEndScNotify {
    const NAME: &'static str = "FeverTimeActivityBattleEndScNotify";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                16 => {
                    self.BFADHPMMODP = is.read_uint32()?;
                },
                64 => {
                    self.id = is.read_uint32()?;
                },
                8 => {
                    self.EGPGPJLOHIL = is.read_enum_or_unknown()?;
                },
                72 => {
                    self.DHFALDJEAAN = is.read_uint32()?;
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
        if self.BFADHPMMODP != 0 {
            my_size += ::protobuf::rt::uint32_size(2, self.BFADHPMMODP);
        }
        if self.id != 0 {
            my_size += ::protobuf::rt::uint32_size(8, self.id);
        }
        if self.EGPGPJLOHIL != ::protobuf::EnumOrUnknown::new(super::FeverTimeBattleRank::FeverTimeBattleRank::FEVER_TIME_BATTLE_RANK_C) {
            my_size += ::protobuf::rt::int32_size(1, self.EGPGPJLOHIL.value());
        }
        if self.DHFALDJEAAN != 0 {
            my_size += ::protobuf::rt::uint32_size(9, self.DHFALDJEAAN);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.BFADHPMMODP != 0 {
            os.write_uint32(2, self.BFADHPMMODP)?;
        }
        if self.id != 0 {
            os.write_uint32(8, self.id)?;
        }
        if self.EGPGPJLOHIL != ::protobuf::EnumOrUnknown::new(super::FeverTimeBattleRank::FeverTimeBattleRank::FEVER_TIME_BATTLE_RANK_C) {
            os.write_enum(1, ::protobuf::EnumOrUnknown::value(&self.EGPGPJLOHIL))?;
        }
        if self.DHFALDJEAAN != 0 {
            os.write_uint32(9, self.DHFALDJEAAN)?;
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

    fn new() -> FeverTimeActivityBattleEndScNotify {
        FeverTimeActivityBattleEndScNotify::new()
    }

    fn clear(&mut self) {
        self.BFADHPMMODP = 0;
        self.id = 0;
        self.EGPGPJLOHIL = ::protobuf::EnumOrUnknown::new(super::FeverTimeBattleRank::FeverTimeBattleRank::FEVER_TIME_BATTLE_RANK_C);
        self.DHFALDJEAAN = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static FeverTimeActivityBattleEndScNotify {
        static instance: FeverTimeActivityBattleEndScNotify = FeverTimeActivityBattleEndScNotify {
            BFADHPMMODP: 0,
            id: 0,
            EGPGPJLOHIL: ::protobuf::EnumOrUnknown::from_i32(0),
            DHFALDJEAAN: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for FeverTimeActivityBattleEndScNotify {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("FeverTimeActivityBattleEndScNotify").unwrap()).clone()
    }
}

impl ::std::fmt::Display for FeverTimeActivityBattleEndScNotify {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for FeverTimeActivityBattleEndScNotify {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n(FeverTimeActivityBattleEndScNotify.proto\x1a\x19FeverTimeBattleRank.p\
    roto\"\xb0\x01\n\"FeverTimeActivityBattleEndScNotify\x12\x20\n\x0bBFADHP\
    MMODP\x18\x02\x20\x01(\rR\x0bBFADHPMMODP\x12\x0e\n\x02id\x18\x08\x20\x01\
    (\rR\x02id\x126\n\x0bEGPGPJLOHIL\x18\x01\x20\x01(\x0e2\x14.FeverTimeBatt\
    leRankR\x0bEGPGPJLOHIL\x12\x20\n\x0bDHFALDJEAAN\x18\t\x20\x01(\rR\x0bDHF\
    ALDJEAANb\x06proto3\
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
            deps.push(super::FeverTimeBattleRank::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(FeverTimeActivityBattleEndScNotify::generated_message_descriptor_data());
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
