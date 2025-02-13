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

//! Generated file from `ChessRogueMoveCellNotify.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:ChessRogueMoveCellNotify)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct ChessRogueMoveCellNotify {
    // message fields
    // @@protoc_insertion_point(field:ChessRogueMoveCellNotify.BPJOLPLHEEP)
    pub BPJOLPLHEEP: u32,
    // @@protoc_insertion_point(field:ChessRogueMoveCellNotify.NIAHBHIGEIL)
    pub NIAHBHIGEIL: u32,
    // @@protoc_insertion_point(field:ChessRogueMoveCellNotify.NKDJAFDAKBP)
    pub NKDJAFDAKBP: ::protobuf::MessageField<super::IMABPHEFCDN::IMABPHEFCDN>,
    // special fields
    // @@protoc_insertion_point(special_field:ChessRogueMoveCellNotify.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a ChessRogueMoveCellNotify {
    fn default() -> &'a ChessRogueMoveCellNotify {
        <ChessRogueMoveCellNotify as ::protobuf::Message>::default_instance()
    }
}

impl ChessRogueMoveCellNotify {
    pub fn new() -> ChessRogueMoveCellNotify {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(3);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "BPJOLPLHEEP",
            |m: &ChessRogueMoveCellNotify| { &m.BPJOLPLHEEP },
            |m: &mut ChessRogueMoveCellNotify| { &mut m.BPJOLPLHEEP },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "NIAHBHIGEIL",
            |m: &ChessRogueMoveCellNotify| { &m.NIAHBHIGEIL },
            |m: &mut ChessRogueMoveCellNotify| { &mut m.NIAHBHIGEIL },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::IMABPHEFCDN::IMABPHEFCDN>(
            "NKDJAFDAKBP",
            |m: &ChessRogueMoveCellNotify| { &m.NKDJAFDAKBP },
            |m: &mut ChessRogueMoveCellNotify| { &mut m.NKDJAFDAKBP },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<ChessRogueMoveCellNotify>(
            "ChessRogueMoveCellNotify",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for ChessRogueMoveCellNotify {
    const NAME: &'static str = "ChessRogueMoveCellNotify";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                88 => {
                    self.BPJOLPLHEEP = is.read_uint32()?;
                },
                104 => {
                    self.NIAHBHIGEIL = is.read_uint32()?;
                },
                26 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.NKDJAFDAKBP)?;
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
        if self.BPJOLPLHEEP != 0 {
            my_size += ::protobuf::rt::uint32_size(11, self.BPJOLPLHEEP);
        }
        if self.NIAHBHIGEIL != 0 {
            my_size += ::protobuf::rt::uint32_size(13, self.NIAHBHIGEIL);
        }
        if let Some(v) = self.NKDJAFDAKBP.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.BPJOLPLHEEP != 0 {
            os.write_uint32(11, self.BPJOLPLHEEP)?;
        }
        if self.NIAHBHIGEIL != 0 {
            os.write_uint32(13, self.NIAHBHIGEIL)?;
        }
        if let Some(v) = self.NKDJAFDAKBP.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(3, v, os)?;
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

    fn new() -> ChessRogueMoveCellNotify {
        ChessRogueMoveCellNotify::new()
    }

    fn clear(&mut self) {
        self.BPJOLPLHEEP = 0;
        self.NIAHBHIGEIL = 0;
        self.NKDJAFDAKBP.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static ChessRogueMoveCellNotify {
        static instance: ChessRogueMoveCellNotify = ChessRogueMoveCellNotify {
            BPJOLPLHEEP: 0,
            NIAHBHIGEIL: 0,
            NKDJAFDAKBP: ::protobuf::MessageField::none(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for ChessRogueMoveCellNotify {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("ChessRogueMoveCellNotify").unwrap()).clone()
    }
}

impl ::std::fmt::Display for ChessRogueMoveCellNotify {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ChessRogueMoveCellNotify {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x1eChessRogueMoveCellNotify.proto\x1a\x11IMABPHEFCDN.proto\"\x8e\x01\
    \n\x18ChessRogueMoveCellNotify\x12\x20\n\x0bBPJOLPLHEEP\x18\x0b\x20\x01(\
    \rR\x0bBPJOLPLHEEP\x12\x20\n\x0bNIAHBHIGEIL\x18\r\x20\x01(\rR\x0bNIAHBHI\
    GEIL\x12.\n\x0bNKDJAFDAKBP\x18\x03\x20\x01(\x0b2\x0c.IMABPHEFCDNR\x0bNKD\
    JAFDAKBPb\x06proto3\
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
            deps.push(super::IMABPHEFCDN::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(ChessRogueMoveCellNotify::generated_message_descriptor_data());
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
