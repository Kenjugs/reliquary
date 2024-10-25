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

//! Generated file from `QuitLineupScRsp.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:QuitLineupScRsp)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct QuitLineupScRsp {
    // message fields
    // @@protoc_insertion_point(field:QuitLineupScRsp.retcode)
    pub retcode: u32,
    // @@protoc_insertion_point(field:QuitLineupScRsp.OGPKNEFGNAN)
    pub OGPKNEFGNAN: u32,
    // @@protoc_insertion_point(field:QuitLineupScRsp.NNPBEFLBLPG)
    pub NNPBEFLBLPG: bool,
    // @@protoc_insertion_point(field:QuitLineupScRsp.AGHOPKKDPLI)
    pub AGHOPKKDPLI: bool,
    // @@protoc_insertion_point(field:QuitLineupScRsp.base_avatar_id)
    pub base_avatar_id: u32,
    // special fields
    // @@protoc_insertion_point(special_field:QuitLineupScRsp.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a QuitLineupScRsp {
    fn default() -> &'a QuitLineupScRsp {
        <QuitLineupScRsp as ::protobuf::Message>::default_instance()
    }
}

impl QuitLineupScRsp {
    pub fn new() -> QuitLineupScRsp {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(5);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "retcode",
            |m: &QuitLineupScRsp| { &m.retcode },
            |m: &mut QuitLineupScRsp| { &mut m.retcode },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "OGPKNEFGNAN",
            |m: &QuitLineupScRsp| { &m.OGPKNEFGNAN },
            |m: &mut QuitLineupScRsp| { &mut m.OGPKNEFGNAN },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "NNPBEFLBLPG",
            |m: &QuitLineupScRsp| { &m.NNPBEFLBLPG },
            |m: &mut QuitLineupScRsp| { &mut m.NNPBEFLBLPG },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "AGHOPKKDPLI",
            |m: &QuitLineupScRsp| { &m.AGHOPKKDPLI },
            |m: &mut QuitLineupScRsp| { &mut m.AGHOPKKDPLI },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "base_avatar_id",
            |m: &QuitLineupScRsp| { &m.base_avatar_id },
            |m: &mut QuitLineupScRsp| { &mut m.base_avatar_id },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<QuitLineupScRsp>(
            "QuitLineupScRsp",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for QuitLineupScRsp {
    const NAME: &'static str = "QuitLineupScRsp";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                72 => {
                    self.retcode = is.read_uint32()?;
                },
                64 => {
                    self.OGPKNEFGNAN = is.read_uint32()?;
                },
                16 => {
                    self.NNPBEFLBLPG = is.read_bool()?;
                },
                120 => {
                    self.AGHOPKKDPLI = is.read_bool()?;
                },
                96 => {
                    self.base_avatar_id = is.read_uint32()?;
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
        if self.retcode != 0 {
            my_size += ::protobuf::rt::uint32_size(9, self.retcode);
        }
        if self.OGPKNEFGNAN != 0 {
            my_size += ::protobuf::rt::uint32_size(8, self.OGPKNEFGNAN);
        }
        if self.NNPBEFLBLPG != false {
            my_size += 1 + 1;
        }
        if self.AGHOPKKDPLI != false {
            my_size += 1 + 1;
        }
        if self.base_avatar_id != 0 {
            my_size += ::protobuf::rt::uint32_size(12, self.base_avatar_id);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.retcode != 0 {
            os.write_uint32(9, self.retcode)?;
        }
        if self.OGPKNEFGNAN != 0 {
            os.write_uint32(8, self.OGPKNEFGNAN)?;
        }
        if self.NNPBEFLBLPG != false {
            os.write_bool(2, self.NNPBEFLBLPG)?;
        }
        if self.AGHOPKKDPLI != false {
            os.write_bool(15, self.AGHOPKKDPLI)?;
        }
        if self.base_avatar_id != 0 {
            os.write_uint32(12, self.base_avatar_id)?;
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

    fn new() -> QuitLineupScRsp {
        QuitLineupScRsp::new()
    }

    fn clear(&mut self) {
        self.retcode = 0;
        self.OGPKNEFGNAN = 0;
        self.NNPBEFLBLPG = false;
        self.AGHOPKKDPLI = false;
        self.base_avatar_id = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static QuitLineupScRsp {
        static instance: QuitLineupScRsp = QuitLineupScRsp {
            retcode: 0,
            OGPKNEFGNAN: 0,
            NNPBEFLBLPG: false,
            AGHOPKKDPLI: false,
            base_avatar_id: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for QuitLineupScRsp {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("QuitLineupScRsp").unwrap()).clone()
    }
}

impl ::std::fmt::Display for QuitLineupScRsp {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for QuitLineupScRsp {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x15QuitLineupScRsp.proto\"\xb7\x01\n\x0fQuitLineupScRsp\x12\x18\n\x07\
    retcode\x18\t\x20\x01(\rR\x07retcode\x12\x20\n\x0bOGPKNEFGNAN\x18\x08\
    \x20\x01(\rR\x0bOGPKNEFGNAN\x12\x20\n\x0bNNPBEFLBLPG\x18\x02\x20\x01(\
    \x08R\x0bNNPBEFLBLPG\x12\x20\n\x0bAGHOPKKDPLI\x18\x0f\x20\x01(\x08R\x0bA\
    GHOPKKDPLI\x12$\n\x0ebase_avatar_id\x18\x0c\x20\x01(\rR\x0cbaseAvatarIdb\
    \x06proto3\
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
            messages.push(QuitLineupScRsp::generated_message_descriptor_data());
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
