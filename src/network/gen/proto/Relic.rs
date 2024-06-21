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

//! Generated file from `Relic.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:Relic)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct Relic {
    // message fields
    // @@protoc_insertion_point(field:Relic.base_avatar_id)
    pub base_avatar_id: u32,
    // @@protoc_insertion_point(field:Relic.level)
    pub level: u32,
    // @@protoc_insertion_point(field:Relic.is_protected)
    pub is_protected: bool,
    // @@protoc_insertion_point(field:Relic.exp)
    pub exp: u32,
    // @@protoc_insertion_point(field:Relic.is_discarded)
    pub is_discarded: bool,
    // @@protoc_insertion_point(field:Relic.tid)
    pub tid: u32,
    // @@protoc_insertion_point(field:Relic.main_affix_id)
    pub main_affix_id: u32,
    // @@protoc_insertion_point(field:Relic.equip_avatar_id)
    pub equip_avatar_id: u32,
    // @@protoc_insertion_point(field:Relic.unique_id)
    pub unique_id: u32,
    // @@protoc_insertion_point(field:Relic.sub_affix_list)
    pub sub_affix_list: ::std::vec::Vec<super::RelicAffix::RelicAffix>,
    // special fields
    // @@protoc_insertion_point(special_field:Relic.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a Relic {
    fn default() -> &'a Relic {
        <Relic as ::protobuf::Message>::default_instance()
    }
}

impl Relic {
    pub fn new() -> Relic {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(10);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "base_avatar_id",
            |m: &Relic| { &m.base_avatar_id },
            |m: &mut Relic| { &mut m.base_avatar_id },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "level",
            |m: &Relic| { &m.level },
            |m: &mut Relic| { &mut m.level },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "is_protected",
            |m: &Relic| { &m.is_protected },
            |m: &mut Relic| { &mut m.is_protected },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "exp",
            |m: &Relic| { &m.exp },
            |m: &mut Relic| { &mut m.exp },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "is_discarded",
            |m: &Relic| { &m.is_discarded },
            |m: &mut Relic| { &mut m.is_discarded },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "tid",
            |m: &Relic| { &m.tid },
            |m: &mut Relic| { &mut m.tid },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "main_affix_id",
            |m: &Relic| { &m.main_affix_id },
            |m: &mut Relic| { &mut m.main_affix_id },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "equip_avatar_id",
            |m: &Relic| { &m.equip_avatar_id },
            |m: &mut Relic| { &mut m.equip_avatar_id },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "unique_id",
            |m: &Relic| { &m.unique_id },
            |m: &mut Relic| { &mut m.unique_id },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "sub_affix_list",
            |m: &Relic| { &m.sub_affix_list },
            |m: &mut Relic| { &mut m.sub_affix_list },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<Relic>(
            "Relic",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for Relic {
    const NAME: &'static str = "Relic";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                56 => {
                    self.base_avatar_id = is.read_uint32()?;
                },
                72 => {
                    self.level = is.read_uint32()?;
                },
                88 => {
                    self.is_protected = is.read_bool()?;
                },
                16 => {
                    self.exp = is.read_uint32()?;
                },
                40 => {
                    self.is_discarded = is.read_bool()?;
                },
                48 => {
                    self.tid = is.read_uint32()?;
                },
                8 => {
                    self.main_affix_id = is.read_uint32()?;
                },
                120 => {
                    self.equip_avatar_id = is.read_uint32()?;
                },
                24 => {
                    self.unique_id = is.read_uint32()?;
                },
                114 => {
                    self.sub_affix_list.push(is.read_message()?);
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
        if self.base_avatar_id != 0 {
            my_size += ::protobuf::rt::uint32_size(7, self.base_avatar_id);
        }
        if self.level != 0 {
            my_size += ::protobuf::rt::uint32_size(9, self.level);
        }
        if self.is_protected != false {
            my_size += 1 + 1;
        }
        if self.exp != 0 {
            my_size += ::protobuf::rt::uint32_size(2, self.exp);
        }
        if self.is_discarded != false {
            my_size += 1 + 1;
        }
        if self.tid != 0 {
            my_size += ::protobuf::rt::uint32_size(6, self.tid);
        }
        if self.main_affix_id != 0 {
            my_size += ::protobuf::rt::uint32_size(1, self.main_affix_id);
        }
        if self.equip_avatar_id != 0 {
            my_size += ::protobuf::rt::uint32_size(15, self.equip_avatar_id);
        }
        if self.unique_id != 0 {
            my_size += ::protobuf::rt::uint32_size(3, self.unique_id);
        }
        for value in &self.sub_affix_list {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.base_avatar_id != 0 {
            os.write_uint32(7, self.base_avatar_id)?;
        }
        if self.level != 0 {
            os.write_uint32(9, self.level)?;
        }
        if self.is_protected != false {
            os.write_bool(11, self.is_protected)?;
        }
        if self.exp != 0 {
            os.write_uint32(2, self.exp)?;
        }
        if self.is_discarded != false {
            os.write_bool(5, self.is_discarded)?;
        }
        if self.tid != 0 {
            os.write_uint32(6, self.tid)?;
        }
        if self.main_affix_id != 0 {
            os.write_uint32(1, self.main_affix_id)?;
        }
        if self.equip_avatar_id != 0 {
            os.write_uint32(15, self.equip_avatar_id)?;
        }
        if self.unique_id != 0 {
            os.write_uint32(3, self.unique_id)?;
        }
        for v in &self.sub_affix_list {
            ::protobuf::rt::write_message_field_with_cached_size(14, v, os)?;
        };
        os.write_unknown_fields(self.special_fields.unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn special_fields(&self) -> &::protobuf::SpecialFields {
        &self.special_fields
    }

    fn mut_special_fields(&mut self) -> &mut ::protobuf::SpecialFields {
        &mut self.special_fields
    }

    fn new() -> Relic {
        Relic::new()
    }

    fn clear(&mut self) {
        self.base_avatar_id = 0;
        self.level = 0;
        self.is_protected = false;
        self.exp = 0;
        self.is_discarded = false;
        self.tid = 0;
        self.main_affix_id = 0;
        self.equip_avatar_id = 0;
        self.unique_id = 0;
        self.sub_affix_list.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static Relic {
        static instance: Relic = Relic {
            base_avatar_id: 0,
            level: 0,
            is_protected: false,
            exp: 0,
            is_discarded: false,
            tid: 0,
            main_affix_id: 0,
            equip_avatar_id: 0,
            unique_id: 0,
            sub_affix_list: ::std::vec::Vec::new(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for Relic {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("Relic").unwrap()).clone()
    }
}

impl ::std::fmt::Display for Relic {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Relic {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x0bRelic.proto\x1a\x10RelicAffix.proto\"\xc9\x02\n\x05Relic\x12$\n\
    \x0ebase_avatar_id\x18\x07\x20\x01(\rR\x0cbaseAvatarId\x12\x14\n\x05leve\
    l\x18\t\x20\x01(\rR\x05level\x12!\n\x0cis_protected\x18\x0b\x20\x01(\x08\
    R\x0bisProtected\x12\x10\n\x03exp\x18\x02\x20\x01(\rR\x03exp\x12!\n\x0ci\
    s_discarded\x18\x05\x20\x01(\x08R\x0bisDiscarded\x12\x10\n\x03tid\x18\
    \x06\x20\x01(\rR\x03tid\x12\"\n\rmain_affix_id\x18\x01\x20\x01(\rR\x0bma\
    inAffixId\x12&\n\x0fequip_avatar_id\x18\x0f\x20\x01(\rR\requipAvatarId\
    \x12\x1b\n\tunique_id\x18\x03\x20\x01(\rR\x08uniqueId\x121\n\x0esub_affi\
    x_list\x18\x0e\x20\x03(\x0b2\x0b.RelicAffixR\x0csubAffixListB\x15\n\x13e\
    mu.lunarcore.protob\x06proto3\
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
            deps.push(super::RelicAffix::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(Relic::generated_message_descriptor_data());
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
