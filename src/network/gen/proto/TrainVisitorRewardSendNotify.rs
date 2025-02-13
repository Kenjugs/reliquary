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

//! Generated file from `TrainVisitorRewardSendNotify.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:TrainVisitorRewardSendNotify)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct TrainVisitorRewardSendNotify {
    // message fields
    // @@protoc_insertion_point(field:TrainVisitorRewardSendNotify.slot)
    pub slot: ::protobuf::EnumOrUnknown<super::KFHIAMADHKF::KFHIAMADHKF>,
    // @@protoc_insertion_point(field:TrainVisitorRewardSendNotify.DJLKCHKMNMI)
    pub DJLKCHKMNMI: u32,
    // @@protoc_insertion_point(field:TrainVisitorRewardSendNotify.LPDNAMLHGNJ)
    pub LPDNAMLHGNJ: ::protobuf::MessageField<super::ItemList::ItemList>,
    // special fields
    // @@protoc_insertion_point(special_field:TrainVisitorRewardSendNotify.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a TrainVisitorRewardSendNotify {
    fn default() -> &'a TrainVisitorRewardSendNotify {
        <TrainVisitorRewardSendNotify as ::protobuf::Message>::default_instance()
    }
}

impl TrainVisitorRewardSendNotify {
    pub fn new() -> TrainVisitorRewardSendNotify {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(3);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "slot",
            |m: &TrainVisitorRewardSendNotify| { &m.slot },
            |m: &mut TrainVisitorRewardSendNotify| { &mut m.slot },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "DJLKCHKMNMI",
            |m: &TrainVisitorRewardSendNotify| { &m.DJLKCHKMNMI },
            |m: &mut TrainVisitorRewardSendNotify| { &mut m.DJLKCHKMNMI },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::ItemList::ItemList>(
            "LPDNAMLHGNJ",
            |m: &TrainVisitorRewardSendNotify| { &m.LPDNAMLHGNJ },
            |m: &mut TrainVisitorRewardSendNotify| { &mut m.LPDNAMLHGNJ },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<TrainVisitorRewardSendNotify>(
            "TrainVisitorRewardSendNotify",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for TrainVisitorRewardSendNotify {
    const NAME: &'static str = "TrainVisitorRewardSendNotify";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                16 => {
                    self.slot = is.read_enum_or_unknown()?;
                },
                88 => {
                    self.DJLKCHKMNMI = is.read_uint32()?;
                },
                106 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.LPDNAMLHGNJ)?;
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
        if self.slot != ::protobuf::EnumOrUnknown::new(super::KFHIAMADHKF::KFHIAMADHKF::TRAIN_VISITOR_REWARD_SEND_NONE) {
            my_size += ::protobuf::rt::int32_size(2, self.slot.value());
        }
        if self.DJLKCHKMNMI != 0 {
            my_size += ::protobuf::rt::uint32_size(11, self.DJLKCHKMNMI);
        }
        if let Some(v) = self.LPDNAMLHGNJ.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.slot != ::protobuf::EnumOrUnknown::new(super::KFHIAMADHKF::KFHIAMADHKF::TRAIN_VISITOR_REWARD_SEND_NONE) {
            os.write_enum(2, ::protobuf::EnumOrUnknown::value(&self.slot))?;
        }
        if self.DJLKCHKMNMI != 0 {
            os.write_uint32(11, self.DJLKCHKMNMI)?;
        }
        if let Some(v) = self.LPDNAMLHGNJ.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(13, v, os)?;
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

    fn new() -> TrainVisitorRewardSendNotify {
        TrainVisitorRewardSendNotify::new()
    }

    fn clear(&mut self) {
        self.slot = ::protobuf::EnumOrUnknown::new(super::KFHIAMADHKF::KFHIAMADHKF::TRAIN_VISITOR_REWARD_SEND_NONE);
        self.DJLKCHKMNMI = 0;
        self.LPDNAMLHGNJ.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static TrainVisitorRewardSendNotify {
        static instance: TrainVisitorRewardSendNotify = TrainVisitorRewardSendNotify {
            slot: ::protobuf::EnumOrUnknown::from_i32(0),
            DJLKCHKMNMI: 0,
            LPDNAMLHGNJ: ::protobuf::MessageField::none(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for TrainVisitorRewardSendNotify {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("TrainVisitorRewardSendNotify").unwrap()).clone()
    }
}

impl ::std::fmt::Display for TrainVisitorRewardSendNotify {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for TrainVisitorRewardSendNotify {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\"TrainVisitorRewardSendNotify.proto\x1a\x0eItemList.proto\x1a\x11KFHI\
    AMADHKF.proto\"\x8f\x01\n\x1cTrainVisitorRewardSendNotify\x12\x20\n\x04s\
    lot\x18\x02\x20\x01(\x0e2\x0c.KFHIAMADHKFR\x04slot\x12\x20\n\x0bDJLKCHKM\
    NMI\x18\x0b\x20\x01(\rR\x0bDJLKCHKMNMI\x12+\n\x0bLPDNAMLHGNJ\x18\r\x20\
    \x01(\x0b2\t.ItemListR\x0bLPDNAMLHGNJb\x06proto3\
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
            deps.push(super::ItemList::file_descriptor().clone());
            deps.push(super::KFHIAMADHKF::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(TrainVisitorRewardSendNotify::generated_message_descriptor_data());
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
