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

//! Generated file from `Equipment.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:Equipment)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct Equipment {
    // message fields
    // @@protoc_insertion_point(field:Equipment.exp)
    pub exp: u32,
    // @@protoc_insertion_point(field:Equipment.promotion)
    pub promotion: u32,
    // @@protoc_insertion_point(field:Equipment.level)
    pub level: u32,
    // @@protoc_insertion_point(field:Equipment.base_avatar_id)
    pub base_avatar_id: u32,
    // @@protoc_insertion_point(field:Equipment.is_protected)
    pub is_protected: bool,
    // @@protoc_insertion_point(field:Equipment.rank)
    pub rank: u32,
    // @@protoc_insertion_point(field:Equipment.unique_id)
    pub unique_id: u32,
    // @@protoc_insertion_point(field:Equipment.equip_avatar_id)
    pub equip_avatar_id: u32,
    // @@protoc_insertion_point(field:Equipment.tid)
    pub tid: u32,
    // special fields
    // @@protoc_insertion_point(special_field:Equipment.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a Equipment {
    fn default() -> &'a Equipment {
        <Equipment as ::protobuf::Message>::default_instance()
    }
}

impl Equipment {
    pub fn new() -> Equipment {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(9);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "exp",
            |m: &Equipment| { &m.exp },
            |m: &mut Equipment| { &mut m.exp },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "promotion",
            |m: &Equipment| { &m.promotion },
            |m: &mut Equipment| { &mut m.promotion },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "level",
            |m: &Equipment| { &m.level },
            |m: &mut Equipment| { &mut m.level },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "base_avatar_id",
            |m: &Equipment| { &m.base_avatar_id },
            |m: &mut Equipment| { &mut m.base_avatar_id },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "is_protected",
            |m: &Equipment| { &m.is_protected },
            |m: &mut Equipment| { &mut m.is_protected },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "rank",
            |m: &Equipment| { &m.rank },
            |m: &mut Equipment| { &mut m.rank },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "unique_id",
            |m: &Equipment| { &m.unique_id },
            |m: &mut Equipment| { &mut m.unique_id },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "equip_avatar_id",
            |m: &Equipment| { &m.equip_avatar_id },
            |m: &mut Equipment| { &mut m.equip_avatar_id },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "tid",
            |m: &Equipment| { &m.tid },
            |m: &mut Equipment| { &mut m.tid },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<Equipment>(
            "Equipment",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for Equipment {
    const NAME: &'static str = "Equipment";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                56 => {
                    self.exp = is.read_uint32()?;
                },
                80 => {
                    self.promotion = is.read_uint32()?;
                },
                72 => {
                    self.level = is.read_uint32()?;
                },
                16 => {
                    self.base_avatar_id = is.read_uint32()?;
                },
                112 => {
                    self.is_protected = is.read_bool()?;
                },
                40 => {
                    self.rank = is.read_uint32()?;
                },
                88 => {
                    self.unique_id = is.read_uint32()?;
                },
                104 => {
                    self.equip_avatar_id = is.read_uint32()?;
                },
                96 => {
                    self.tid = is.read_uint32()?;
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
        if self.exp != 0 {
            my_size += ::protobuf::rt::uint32_size(7, self.exp);
        }
        if self.promotion != 0 {
            my_size += ::protobuf::rt::uint32_size(10, self.promotion);
        }
        if self.level != 0 {
            my_size += ::protobuf::rt::uint32_size(9, self.level);
        }
        if self.base_avatar_id != 0 {
            my_size += ::protobuf::rt::uint32_size(2, self.base_avatar_id);
        }
        if self.is_protected != false {
            my_size += 1 + 1;
        }
        if self.rank != 0 {
            my_size += ::protobuf::rt::uint32_size(5, self.rank);
        }
        if self.unique_id != 0 {
            my_size += ::protobuf::rt::uint32_size(11, self.unique_id);
        }
        if self.equip_avatar_id != 0 {
            my_size += ::protobuf::rt::uint32_size(13, self.equip_avatar_id);
        }
        if self.tid != 0 {
            my_size += ::protobuf::rt::uint32_size(12, self.tid);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.exp != 0 {
            os.write_uint32(7, self.exp)?;
        }
        if self.promotion != 0 {
            os.write_uint32(10, self.promotion)?;
        }
        if self.level != 0 {
            os.write_uint32(9, self.level)?;
        }
        if self.base_avatar_id != 0 {
            os.write_uint32(2, self.base_avatar_id)?;
        }
        if self.is_protected != false {
            os.write_bool(14, self.is_protected)?;
        }
        if self.rank != 0 {
            os.write_uint32(5, self.rank)?;
        }
        if self.unique_id != 0 {
            os.write_uint32(11, self.unique_id)?;
        }
        if self.equip_avatar_id != 0 {
            os.write_uint32(13, self.equip_avatar_id)?;
        }
        if self.tid != 0 {
            os.write_uint32(12, self.tid)?;
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

    fn new() -> Equipment {
        Equipment::new()
    }

    fn clear(&mut self) {
        self.exp = 0;
        self.promotion = 0;
        self.level = 0;
        self.base_avatar_id = 0;
        self.is_protected = false;
        self.rank = 0;
        self.unique_id = 0;
        self.equip_avatar_id = 0;
        self.tid = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static Equipment {
        static instance: Equipment = Equipment {
            exp: 0,
            promotion: 0,
            level: 0,
            base_avatar_id: 0,
            is_protected: false,
            rank: 0,
            unique_id: 0,
            equip_avatar_id: 0,
            tid: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for Equipment {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("Equipment").unwrap()).clone()
    }
}

impl ::std::fmt::Display for Equipment {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Equipment {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x0fEquipment.proto\"\x85\x02\n\tEquipment\x12\x10\n\x03exp\x18\x07\
    \x20\x01(\rR\x03exp\x12\x1c\n\tpromotion\x18\n\x20\x01(\rR\tpromotion\
    \x12\x14\n\x05level\x18\t\x20\x01(\rR\x05level\x12$\n\x0ebase_avatar_id\
    \x18\x02\x20\x01(\rR\x0cbaseAvatarId\x12!\n\x0cis_protected\x18\x0e\x20\
    \x01(\x08R\x0bisProtected\x12\x12\n\x04rank\x18\x05\x20\x01(\rR\x04rank\
    \x12\x1b\n\tunique_id\x18\x0b\x20\x01(\rR\x08uniqueId\x12&\n\x0fequip_av\
    atar_id\x18\r\x20\x01(\rR\requipAvatarId\x12\x10\n\x03tid\x18\x0c\x20\
    \x01(\rR\x03tidB\x15\n\x13emu.lunarcore.protob\x06proto3\
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
            messages.push(Equipment::generated_message_descriptor_data());
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
