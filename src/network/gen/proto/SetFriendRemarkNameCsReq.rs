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

//! Generated file from `SetFriendRemarkNameCsReq.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:SetFriendRemarkNameCsReq)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct SetFriendRemarkNameCsReq {
    // message fields
    // @@protoc_insertion_point(field:SetFriendRemarkNameCsReq.uid)
    pub uid: u32,
    // @@protoc_insertion_point(field:SetFriendRemarkNameCsReq.KDLEHAPNACL)
    pub KDLEHAPNACL: ::std::string::String,
    // @@protoc_insertion_point(field:SetFriendRemarkNameCsReq.DGDDHBLKMLI)
    pub DGDDHBLKMLI: u32,
    // special fields
    // @@protoc_insertion_point(special_field:SetFriendRemarkNameCsReq.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a SetFriendRemarkNameCsReq {
    fn default() -> &'a SetFriendRemarkNameCsReq {
        <SetFriendRemarkNameCsReq as ::protobuf::Message>::default_instance()
    }
}

impl SetFriendRemarkNameCsReq {
    pub fn new() -> SetFriendRemarkNameCsReq {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(3);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "uid",
            |m: &SetFriendRemarkNameCsReq| { &m.uid },
            |m: &mut SetFriendRemarkNameCsReq| { &mut m.uid },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "KDLEHAPNACL",
            |m: &SetFriendRemarkNameCsReq| { &m.KDLEHAPNACL },
            |m: &mut SetFriendRemarkNameCsReq| { &mut m.KDLEHAPNACL },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "DGDDHBLKMLI",
            |m: &SetFriendRemarkNameCsReq| { &m.DGDDHBLKMLI },
            |m: &mut SetFriendRemarkNameCsReq| { &mut m.DGDDHBLKMLI },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<SetFriendRemarkNameCsReq>(
            "SetFriendRemarkNameCsReq",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for SetFriendRemarkNameCsReq {
    const NAME: &'static str = "SetFriendRemarkNameCsReq";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                104 => {
                    self.uid = is.read_uint32()?;
                },
                90 => {
                    self.KDLEHAPNACL = is.read_string()?;
                },
                120 => {
                    self.DGDDHBLKMLI = is.read_uint32()?;
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
        if self.uid != 0 {
            my_size += ::protobuf::rt::uint32_size(13, self.uid);
        }
        if !self.KDLEHAPNACL.is_empty() {
            my_size += ::protobuf::rt::string_size(11, &self.KDLEHAPNACL);
        }
        if self.DGDDHBLKMLI != 0 {
            my_size += ::protobuf::rt::uint32_size(15, self.DGDDHBLKMLI);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.uid != 0 {
            os.write_uint32(13, self.uid)?;
        }
        if !self.KDLEHAPNACL.is_empty() {
            os.write_string(11, &self.KDLEHAPNACL)?;
        }
        if self.DGDDHBLKMLI != 0 {
            os.write_uint32(15, self.DGDDHBLKMLI)?;
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

    fn new() -> SetFriendRemarkNameCsReq {
        SetFriendRemarkNameCsReq::new()
    }

    fn clear(&mut self) {
        self.uid = 0;
        self.KDLEHAPNACL.clear();
        self.DGDDHBLKMLI = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static SetFriendRemarkNameCsReq {
        static instance: SetFriendRemarkNameCsReq = SetFriendRemarkNameCsReq {
            uid: 0,
            KDLEHAPNACL: ::std::string::String::new(),
            DGDDHBLKMLI: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for SetFriendRemarkNameCsReq {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("SetFriendRemarkNameCsReq").unwrap()).clone()
    }
}

impl ::std::fmt::Display for SetFriendRemarkNameCsReq {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for SetFriendRemarkNameCsReq {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x1eSetFriendRemarkNameCsReq.proto\"p\n\x18SetFriendRemarkNameCsReq\
    \x12\x10\n\x03uid\x18\r\x20\x01(\rR\x03uid\x12\x20\n\x0bKDLEHAPNACL\x18\
    \x0b\x20\x01(\tR\x0bKDLEHAPNACL\x12\x20\n\x0bDGDDHBLKMLI\x18\x0f\x20\x01\
    (\rR\x0bDGDDHBLKMLIb\x06proto3\
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
            messages.push(SetFriendRemarkNameCsReq::generated_message_descriptor_data());
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
