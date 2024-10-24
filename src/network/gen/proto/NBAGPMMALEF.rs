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

//! Generated file from `NBAGPMMALEF.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:NBAGPMMALEF)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct NBAGPMMALEF {
    // message fields
    // @@protoc_insertion_point(field:NBAGPMMALEF.id)
    pub id: u32,
    // @@protoc_insertion_point(field:NBAGPMMALEF.level)
    pub level: u32,
    // @@protoc_insertion_point(field:NBAGPMMALEF.main_affix_id)
    pub main_affix_id: u32,
    // @@protoc_insertion_point(field:NBAGPMMALEF.sub_affix_list)
    pub sub_affix_list: ::std::vec::Vec<super::RelicAffix::RelicAffix>,
    // @@protoc_insertion_point(field:NBAGPMMALEF.unique_id)
    pub unique_id: u32,
    // @@protoc_insertion_point(field:NBAGPMMALEF.KIDAMGHIOOH)
    pub KIDAMGHIOOH: u32,
    // @@protoc_insertion_point(field:NBAGPMMALEF.slot)
    pub slot: u32,
    // @@protoc_insertion_point(field:NBAGPMMALEF.MEODGAGGOKI)
    pub MEODGAGGOKI: u32,
    // special fields
    // @@protoc_insertion_point(special_field:NBAGPMMALEF.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a NBAGPMMALEF {
    fn default() -> &'a NBAGPMMALEF {
        <NBAGPMMALEF as ::protobuf::Message>::default_instance()
    }
}

impl NBAGPMMALEF {
    pub fn new() -> NBAGPMMALEF {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(8);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "id",
            |m: &NBAGPMMALEF| { &m.id },
            |m: &mut NBAGPMMALEF| { &mut m.id },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "level",
            |m: &NBAGPMMALEF| { &m.level },
            |m: &mut NBAGPMMALEF| { &mut m.level },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "main_affix_id",
            |m: &NBAGPMMALEF| { &m.main_affix_id },
            |m: &mut NBAGPMMALEF| { &mut m.main_affix_id },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "sub_affix_list",
            |m: &NBAGPMMALEF| { &m.sub_affix_list },
            |m: &mut NBAGPMMALEF| { &mut m.sub_affix_list },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "unique_id",
            |m: &NBAGPMMALEF| { &m.unique_id },
            |m: &mut NBAGPMMALEF| { &mut m.unique_id },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "KIDAMGHIOOH",
            |m: &NBAGPMMALEF| { &m.KIDAMGHIOOH },
            |m: &mut NBAGPMMALEF| { &mut m.KIDAMGHIOOH },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "slot",
            |m: &NBAGPMMALEF| { &m.slot },
            |m: &mut NBAGPMMALEF| { &mut m.slot },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "MEODGAGGOKI",
            |m: &NBAGPMMALEF| { &m.MEODGAGGOKI },
            |m: &mut NBAGPMMALEF| { &mut m.MEODGAGGOKI },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<NBAGPMMALEF>(
            "NBAGPMMALEF",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for NBAGPMMALEF {
    const NAME: &'static str = "NBAGPMMALEF";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                8 => {
                    self.id = is.read_uint32()?;
                },
                16 => {
                    self.level = is.read_uint32()?;
                },
                24 => {
                    self.main_affix_id = is.read_uint32()?;
                },
                34 => {
                    self.sub_affix_list.push(is.read_message()?);
                },
                40 => {
                    self.unique_id = is.read_uint32()?;
                },
                48 => {
                    self.KIDAMGHIOOH = is.read_uint32()?;
                },
                56 => {
                    self.slot = is.read_uint32()?;
                },
                64 => {
                    self.MEODGAGGOKI = is.read_uint32()?;
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
        if self.id != 0 {
            my_size += ::protobuf::rt::uint32_size(1, self.id);
        }
        if self.level != 0 {
            my_size += ::protobuf::rt::uint32_size(2, self.level);
        }
        if self.main_affix_id != 0 {
            my_size += ::protobuf::rt::uint32_size(3, self.main_affix_id);
        }
        for value in &self.sub_affix_list {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        if self.unique_id != 0 {
            my_size += ::protobuf::rt::uint32_size(5, self.unique_id);
        }
        if self.KIDAMGHIOOH != 0 {
            my_size += ::protobuf::rt::uint32_size(6, self.KIDAMGHIOOH);
        }
        if self.slot != 0 {
            my_size += ::protobuf::rt::uint32_size(7, self.slot);
        }
        if self.MEODGAGGOKI != 0 {
            my_size += ::protobuf::rt::uint32_size(8, self.MEODGAGGOKI);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.id != 0 {
            os.write_uint32(1, self.id)?;
        }
        if self.level != 0 {
            os.write_uint32(2, self.level)?;
        }
        if self.main_affix_id != 0 {
            os.write_uint32(3, self.main_affix_id)?;
        }
        for v in &self.sub_affix_list {
            ::protobuf::rt::write_message_field_with_cached_size(4, v, os)?;
        };
        if self.unique_id != 0 {
            os.write_uint32(5, self.unique_id)?;
        }
        if self.KIDAMGHIOOH != 0 {
            os.write_uint32(6, self.KIDAMGHIOOH)?;
        }
        if self.slot != 0 {
            os.write_uint32(7, self.slot)?;
        }
        if self.MEODGAGGOKI != 0 {
            os.write_uint32(8, self.MEODGAGGOKI)?;
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

    fn new() -> NBAGPMMALEF {
        NBAGPMMALEF::new()
    }

    fn clear(&mut self) {
        self.id = 0;
        self.level = 0;
        self.main_affix_id = 0;
        self.sub_affix_list.clear();
        self.unique_id = 0;
        self.KIDAMGHIOOH = 0;
        self.slot = 0;
        self.MEODGAGGOKI = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static NBAGPMMALEF {
        static instance: NBAGPMMALEF = NBAGPMMALEF {
            id: 0,
            level: 0,
            main_affix_id: 0,
            sub_affix_list: ::std::vec::Vec::new(),
            unique_id: 0,
            KIDAMGHIOOH: 0,
            slot: 0,
            MEODGAGGOKI: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for NBAGPMMALEF {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("NBAGPMMALEF").unwrap()).clone()
    }
}

impl ::std::fmt::Display for NBAGPMMALEF {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for NBAGPMMALEF {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11NBAGPMMALEF.proto\x1a\x10RelicAffix.proto\"\xff\x01\n\x0bNBAGPMMAL\
    EF\x12\x0e\n\x02id\x18\x01\x20\x01(\rR\x02id\x12\x14\n\x05level\x18\x02\
    \x20\x01(\rR\x05level\x12\"\n\rmain_affix_id\x18\x03\x20\x01(\rR\x0bmain\
    AffixId\x121\n\x0esub_affix_list\x18\x04\x20\x03(\x0b2\x0b.RelicAffixR\
    \x0csubAffixList\x12\x1b\n\tunique_id\x18\x05\x20\x01(\rR\x08uniqueId\
    \x12\x20\n\x0bKIDAMGHIOOH\x18\x06\x20\x01(\rR\x0bKIDAMGHIOOH\x12\x12\n\
    \x04slot\x18\x07\x20\x01(\rR\x04slot\x12\x20\n\x0bMEODGAGGOKI\x18\x08\
    \x20\x01(\rR\x0bMEODGAGGOKIb\x06proto3\
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
            messages.push(NBAGPMMALEF::generated_message_descriptor_data());
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
