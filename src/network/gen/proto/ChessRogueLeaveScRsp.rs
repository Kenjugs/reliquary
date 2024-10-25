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

//! Generated file from `ChessRogueLeaveScRsp.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:ChessRogueLeaveScRsp)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct ChessRogueLeaveScRsp {
    // message fields
    // @@protoc_insertion_point(field:ChessRogueLeaveScRsp.CBLEOKIPEEA)
    pub CBLEOKIPEEA: ::protobuf::MessageField<super::LCGKENFJICO::LCGKENFJICO>,
    // @@protoc_insertion_point(field:ChessRogueLeaveScRsp.HLDLDAPNILF)
    pub HLDLDAPNILF: ::protobuf::MessageField<super::MOHDEOFNBNK::MOHDEOFNBNK>,
    // @@protoc_insertion_point(field:ChessRogueLeaveScRsp.retcode)
    pub retcode: u32,
    // @@protoc_insertion_point(field:ChessRogueLeaveScRsp.DAKOPHLKOGK)
    pub DAKOPHLKOGK: ::protobuf::MessageField<super::ANGHJLBFOAL::ANGHJLBFOAL>,
    // @@protoc_insertion_point(field:ChessRogueLeaveScRsp.CMDGDKBACOD)
    pub CMDGDKBACOD: ::protobuf::MessageField<super::EOIFAHBJKDA::EOIFAHBJKDA>,
    // special fields
    // @@protoc_insertion_point(special_field:ChessRogueLeaveScRsp.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a ChessRogueLeaveScRsp {
    fn default() -> &'a ChessRogueLeaveScRsp {
        <ChessRogueLeaveScRsp as ::protobuf::Message>::default_instance()
    }
}

impl ChessRogueLeaveScRsp {
    pub fn new() -> ChessRogueLeaveScRsp {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(5);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::LCGKENFJICO::LCGKENFJICO>(
            "CBLEOKIPEEA",
            |m: &ChessRogueLeaveScRsp| { &m.CBLEOKIPEEA },
            |m: &mut ChessRogueLeaveScRsp| { &mut m.CBLEOKIPEEA },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::MOHDEOFNBNK::MOHDEOFNBNK>(
            "HLDLDAPNILF",
            |m: &ChessRogueLeaveScRsp| { &m.HLDLDAPNILF },
            |m: &mut ChessRogueLeaveScRsp| { &mut m.HLDLDAPNILF },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "retcode",
            |m: &ChessRogueLeaveScRsp| { &m.retcode },
            |m: &mut ChessRogueLeaveScRsp| { &mut m.retcode },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::ANGHJLBFOAL::ANGHJLBFOAL>(
            "DAKOPHLKOGK",
            |m: &ChessRogueLeaveScRsp| { &m.DAKOPHLKOGK },
            |m: &mut ChessRogueLeaveScRsp| { &mut m.DAKOPHLKOGK },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::EOIFAHBJKDA::EOIFAHBJKDA>(
            "CMDGDKBACOD",
            |m: &ChessRogueLeaveScRsp| { &m.CMDGDKBACOD },
            |m: &mut ChessRogueLeaveScRsp| { &mut m.CMDGDKBACOD },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<ChessRogueLeaveScRsp>(
            "ChessRogueLeaveScRsp",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for ChessRogueLeaveScRsp {
    const NAME: &'static str = "ChessRogueLeaveScRsp";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                50 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.CBLEOKIPEEA)?;
                },
                74 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.HLDLDAPNILF)?;
                },
                104 => {
                    self.retcode = is.read_uint32()?;
                },
                42 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.DAKOPHLKOGK)?;
                },
                90 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.CMDGDKBACOD)?;
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
        if let Some(v) = self.CBLEOKIPEEA.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if let Some(v) = self.HLDLDAPNILF.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if self.retcode != 0 {
            my_size += ::protobuf::rt::uint32_size(13, self.retcode);
        }
        if let Some(v) = self.DAKOPHLKOGK.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if let Some(v) = self.CMDGDKBACOD.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if let Some(v) = self.CBLEOKIPEEA.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(6, v, os)?;
        }
        if let Some(v) = self.HLDLDAPNILF.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(9, v, os)?;
        }
        if self.retcode != 0 {
            os.write_uint32(13, self.retcode)?;
        }
        if let Some(v) = self.DAKOPHLKOGK.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(5, v, os)?;
        }
        if let Some(v) = self.CMDGDKBACOD.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(11, v, os)?;
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

    fn new() -> ChessRogueLeaveScRsp {
        ChessRogueLeaveScRsp::new()
    }

    fn clear(&mut self) {
        self.CBLEOKIPEEA.clear();
        self.HLDLDAPNILF.clear();
        self.retcode = 0;
        self.DAKOPHLKOGK.clear();
        self.CMDGDKBACOD.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static ChessRogueLeaveScRsp {
        static instance: ChessRogueLeaveScRsp = ChessRogueLeaveScRsp {
            CBLEOKIPEEA: ::protobuf::MessageField::none(),
            HLDLDAPNILF: ::protobuf::MessageField::none(),
            retcode: 0,
            DAKOPHLKOGK: ::protobuf::MessageField::none(),
            CMDGDKBACOD: ::protobuf::MessageField::none(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for ChessRogueLeaveScRsp {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("ChessRogueLeaveScRsp").unwrap()).clone()
    }
}

impl ::std::fmt::Display for ChessRogueLeaveScRsp {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ChessRogueLeaveScRsp {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x1aChessRogueLeaveScRsp.proto\x1a\x11ANGHJLBFOAL.proto\x1a\x11EOIFAHB\
    JKDA.proto\x1a\x11LCGKENFJICO.proto\x1a\x11MOHDEOFNBNK.proto\"\xf0\x01\n\
    \x14ChessRogueLeaveScRsp\x12.\n\x0bCBLEOKIPEEA\x18\x06\x20\x01(\x0b2\x0c\
    .LCGKENFJICOR\x0bCBLEOKIPEEA\x12.\n\x0bHLDLDAPNILF\x18\t\x20\x01(\x0b2\
    \x0c.MOHDEOFNBNKR\x0bHLDLDAPNILF\x12\x18\n\x07retcode\x18\r\x20\x01(\rR\
    \x07retcode\x12.\n\x0bDAKOPHLKOGK\x18\x05\x20\x01(\x0b2\x0c.ANGHJLBFOALR\
    \x0bDAKOPHLKOGK\x12.\n\x0bCMDGDKBACOD\x18\x0b\x20\x01(\x0b2\x0c.EOIFAHBJ\
    KDAR\x0bCMDGDKBACODb\x06proto3\
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
            let mut deps = ::std::vec::Vec::with_capacity(4);
            deps.push(super::ANGHJLBFOAL::file_descriptor().clone());
            deps.push(super::EOIFAHBJKDA::file_descriptor().clone());
            deps.push(super::LCGKENFJICO::file_descriptor().clone());
            deps.push(super::MOHDEOFNBNK::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(ChessRogueLeaveScRsp::generated_message_descriptor_data());
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
