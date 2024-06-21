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

//! Generated file from `RevcMsgScNotify.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:RevcMsgScNotify)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct RevcMsgScNotify {
    // message fields
    // @@protoc_insertion_point(field:RevcMsgScNotify.to_uid)
    pub to_uid: u32,
    // @@protoc_insertion_point(field:RevcMsgScNotify.chat_type)
    pub chat_type: ::protobuf::EnumOrUnknown<super::ChatType::ChatType>,
    // @@protoc_insertion_point(field:RevcMsgScNotify.emote)
    pub emote: u32,
    // @@protoc_insertion_point(field:RevcMsgScNotify.msg_type)
    pub msg_type: ::protobuf::EnumOrUnknown<super::MsgType::MsgType>,
    // @@protoc_insertion_point(field:RevcMsgScNotify.text)
    pub text: ::std::string::String,
    // @@protoc_insertion_point(field:RevcMsgScNotify.from_uid)
    pub from_uid: u32,
    // special fields
    // @@protoc_insertion_point(special_field:RevcMsgScNotify.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a RevcMsgScNotify {
    fn default() -> &'a RevcMsgScNotify {
        <RevcMsgScNotify as ::protobuf::Message>::default_instance()
    }
}

impl RevcMsgScNotify {
    pub fn new() -> RevcMsgScNotify {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(6);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "to_uid",
            |m: &RevcMsgScNotify| { &m.to_uid },
            |m: &mut RevcMsgScNotify| { &mut m.to_uid },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "chat_type",
            |m: &RevcMsgScNotify| { &m.chat_type },
            |m: &mut RevcMsgScNotify| { &mut m.chat_type },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "emote",
            |m: &RevcMsgScNotify| { &m.emote },
            |m: &mut RevcMsgScNotify| { &mut m.emote },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "msg_type",
            |m: &RevcMsgScNotify| { &m.msg_type },
            |m: &mut RevcMsgScNotify| { &mut m.msg_type },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "text",
            |m: &RevcMsgScNotify| { &m.text },
            |m: &mut RevcMsgScNotify| { &mut m.text },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "from_uid",
            |m: &RevcMsgScNotify| { &m.from_uid },
            |m: &mut RevcMsgScNotify| { &mut m.from_uid },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<RevcMsgScNotify>(
            "RevcMsgScNotify",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for RevcMsgScNotify {
    const NAME: &'static str = "RevcMsgScNotify";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                88 => {
                    self.to_uid = is.read_uint32()?;
                },
                32 => {
                    self.chat_type = is.read_enum_or_unknown()?;
                },
                56 => {
                    self.emote = is.read_uint32()?;
                },
                64 => {
                    self.msg_type = is.read_enum_or_unknown()?;
                },
                50 => {
                    self.text = is.read_string()?;
                },
                120 => {
                    self.from_uid = is.read_uint32()?;
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
        if self.to_uid != 0 {
            my_size += ::protobuf::rt::uint32_size(11, self.to_uid);
        }
        if self.chat_type != ::protobuf::EnumOrUnknown::new(super::ChatType::ChatType::CHAT_TYPE_NONE) {
            my_size += ::protobuf::rt::int32_size(4, self.chat_type.value());
        }
        if self.emote != 0 {
            my_size += ::protobuf::rt::uint32_size(7, self.emote);
        }
        if self.msg_type != ::protobuf::EnumOrUnknown::new(super::MsgType::MsgType::MSG_TYPE_NONE) {
            my_size += ::protobuf::rt::int32_size(8, self.msg_type.value());
        }
        if !self.text.is_empty() {
            my_size += ::protobuf::rt::string_size(6, &self.text);
        }
        if self.from_uid != 0 {
            my_size += ::protobuf::rt::uint32_size(15, self.from_uid);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.to_uid != 0 {
            os.write_uint32(11, self.to_uid)?;
        }
        if self.chat_type != ::protobuf::EnumOrUnknown::new(super::ChatType::ChatType::CHAT_TYPE_NONE) {
            os.write_enum(4, ::protobuf::EnumOrUnknown::value(&self.chat_type))?;
        }
        if self.emote != 0 {
            os.write_uint32(7, self.emote)?;
        }
        if self.msg_type != ::protobuf::EnumOrUnknown::new(super::MsgType::MsgType::MSG_TYPE_NONE) {
            os.write_enum(8, ::protobuf::EnumOrUnknown::value(&self.msg_type))?;
        }
        if !self.text.is_empty() {
            os.write_string(6, &self.text)?;
        }
        if self.from_uid != 0 {
            os.write_uint32(15, self.from_uid)?;
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

    fn new() -> RevcMsgScNotify {
        RevcMsgScNotify::new()
    }

    fn clear(&mut self) {
        self.to_uid = 0;
        self.chat_type = ::protobuf::EnumOrUnknown::new(super::ChatType::ChatType::CHAT_TYPE_NONE);
        self.emote = 0;
        self.msg_type = ::protobuf::EnumOrUnknown::new(super::MsgType::MsgType::MSG_TYPE_NONE);
        self.text.clear();
        self.from_uid = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static RevcMsgScNotify {
        static instance: RevcMsgScNotify = RevcMsgScNotify {
            to_uid: 0,
            chat_type: ::protobuf::EnumOrUnknown::from_i32(0),
            emote: 0,
            msg_type: ::protobuf::EnumOrUnknown::from_i32(0),
            text: ::std::string::String::new(),
            from_uid: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for RevcMsgScNotify {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("RevcMsgScNotify").unwrap()).clone()
    }
}

impl ::std::fmt::Display for RevcMsgScNotify {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for RevcMsgScNotify {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x15RevcMsgScNotify.proto\x1a\x0eChatType.proto\x1a\rMsgType.proto\"\
    \xba\x01\n\x0fRevcMsgScNotify\x12\x15\n\x06to_uid\x18\x0b\x20\x01(\rR\
    \x05toUid\x12&\n\tchat_type\x18\x04\x20\x01(\x0e2\t.ChatTypeR\x08chatTyp\
    e\x12\x14\n\x05emote\x18\x07\x20\x01(\rR\x05emote\x12#\n\x08msg_type\x18\
    \x08\x20\x01(\x0e2\x08.MsgTypeR\x07msgType\x12\x12\n\x04text\x18\x06\x20\
    \x01(\tR\x04text\x12\x19\n\x08from_uid\x18\x0f\x20\x01(\rR\x07fromUidB\
    \x15\n\x13emu.lunarcore.protob\x06proto3\
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
            deps.push(super::ChatType::file_descriptor().clone());
            deps.push(super::MsgType::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(RevcMsgScNotify::generated_message_descriptor_data());
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
