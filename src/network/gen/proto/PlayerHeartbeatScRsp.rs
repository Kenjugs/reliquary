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

//! Generated file from `PlayerHeartBeatScRsp.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:PlayerHeartBeatScRsp)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct PlayerHeartBeatScRsp {
    // message fields
    // @@protoc_insertion_point(field:PlayerHeartBeatScRsp.PHPLKLDJPBF)
    pub PHPLKLDJPBF: u64,
    // @@protoc_insertion_point(field:PlayerHeartBeatScRsp.retcode)
    pub retcode: u32,
    // @@protoc_insertion_point(field:PlayerHeartBeatScRsp.KDMJFLFLCHN)
    pub KDMJFLFLCHN: ::protobuf::MessageField<super::HGGLCIFHMJM::HGGLCIFHMJM>,
    // @@protoc_insertion_point(field:PlayerHeartBeatScRsp.BOLBGEEGOJN)
    pub BOLBGEEGOJN: u64,
    // special fields
    // @@protoc_insertion_point(special_field:PlayerHeartBeatScRsp.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a PlayerHeartBeatScRsp {
    fn default() -> &'a PlayerHeartBeatScRsp {
        <PlayerHeartBeatScRsp as ::protobuf::Message>::default_instance()
    }
}

impl PlayerHeartBeatScRsp {
    pub fn new() -> PlayerHeartBeatScRsp {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(4);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "PHPLKLDJPBF",
            |m: &PlayerHeartBeatScRsp| { &m.PHPLKLDJPBF },
            |m: &mut PlayerHeartBeatScRsp| { &mut m.PHPLKLDJPBF },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "retcode",
            |m: &PlayerHeartBeatScRsp| { &m.retcode },
            |m: &mut PlayerHeartBeatScRsp| { &mut m.retcode },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::HGGLCIFHMJM::HGGLCIFHMJM>(
            "KDMJFLFLCHN",
            |m: &PlayerHeartBeatScRsp| { &m.KDMJFLFLCHN },
            |m: &mut PlayerHeartBeatScRsp| { &mut m.KDMJFLFLCHN },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "BOLBGEEGOJN",
            |m: &PlayerHeartBeatScRsp| { &m.BOLBGEEGOJN },
            |m: &mut PlayerHeartBeatScRsp| { &mut m.BOLBGEEGOJN },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<PlayerHeartBeatScRsp>(
            "PlayerHeartBeatScRsp",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for PlayerHeartBeatScRsp {
    const NAME: &'static str = "PlayerHeartBeatScRsp";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                32 => {
                    self.PHPLKLDJPBF = is.read_uint64()?;
                },
                96 => {
                    self.retcode = is.read_uint32()?;
                },
                58 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.KDMJFLFLCHN)?;
                },
                8 => {
                    self.BOLBGEEGOJN = is.read_uint64()?;
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
        if self.PHPLKLDJPBF != 0 {
            my_size += ::protobuf::rt::uint64_size(4, self.PHPLKLDJPBF);
        }
        if self.retcode != 0 {
            my_size += ::protobuf::rt::uint32_size(12, self.retcode);
        }
        if let Some(v) = self.KDMJFLFLCHN.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if self.BOLBGEEGOJN != 0 {
            my_size += ::protobuf::rt::uint64_size(1, self.BOLBGEEGOJN);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.PHPLKLDJPBF != 0 {
            os.write_uint64(4, self.PHPLKLDJPBF)?;
        }
        if self.retcode != 0 {
            os.write_uint32(12, self.retcode)?;
        }
        if let Some(v) = self.KDMJFLFLCHN.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(7, v, os)?;
        }
        if self.BOLBGEEGOJN != 0 {
            os.write_uint64(1, self.BOLBGEEGOJN)?;
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

    fn new() -> PlayerHeartBeatScRsp {
        PlayerHeartBeatScRsp::new()
    }

    fn clear(&mut self) {
        self.PHPLKLDJPBF = 0;
        self.retcode = 0;
        self.KDMJFLFLCHN.clear();
        self.BOLBGEEGOJN = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static PlayerHeartBeatScRsp {
        static instance: PlayerHeartBeatScRsp = PlayerHeartBeatScRsp {
            PHPLKLDJPBF: 0,
            retcode: 0,
            KDMJFLFLCHN: ::protobuf::MessageField::none(),
            BOLBGEEGOJN: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for PlayerHeartBeatScRsp {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("PlayerHeartBeatScRsp").unwrap()).clone()
    }
}

impl ::std::fmt::Display for PlayerHeartBeatScRsp {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for PlayerHeartBeatScRsp {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x1aPlayerHeartBeatScRsp.proto\x1a\x11HGGLCIFHMJM.proto\"\xa4\x01\n\
    \x14PlayerHeartBeatScRsp\x12\x20\n\x0bPHPLKLDJPBF\x18\x04\x20\x01(\x04R\
    \x0bPHPLKLDJPBF\x12\x18\n\x07retcode\x18\x0c\x20\x01(\rR\x07retcode\x12.\
    \n\x0bKDMJFLFLCHN\x18\x07\x20\x01(\x0b2\x0c.HGGLCIFHMJMR\x0bKDMJFLFLCHN\
    \x12\x20\n\x0bBOLBGEEGOJN\x18\x01\x20\x01(\x04R\x0bBOLBGEEGOJNb\x06proto\
    3\
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
            deps.push(super::HGGLCIFHMJM::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(PlayerHeartBeatScRsp::generated_message_descriptor_data());
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
