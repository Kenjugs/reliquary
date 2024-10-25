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

//! Generated file from `MNODDHJLJKH.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:MNODDHJLJKH)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct MNODDHJLJKH {
    // message fields
    // @@protoc_insertion_point(field:MNODDHJLJKH.KGILLKDLGFK)
    pub KGILLKDLGFK: ::std::vec::Vec<u32>,
    // @@protoc_insertion_point(field:MNODDHJLJKH.KGALGMDJIDD)
    pub KGALGMDJIDD: ::std::vec::Vec<u32>,
    // special fields
    // @@protoc_insertion_point(special_field:MNODDHJLJKH.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a MNODDHJLJKH {
    fn default() -> &'a MNODDHJLJKH {
        <MNODDHJLJKH as ::protobuf::Message>::default_instance()
    }
}

impl MNODDHJLJKH {
    pub fn new() -> MNODDHJLJKH {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(2);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "KGILLKDLGFK",
            |m: &MNODDHJLJKH| { &m.KGILLKDLGFK },
            |m: &mut MNODDHJLJKH| { &mut m.KGILLKDLGFK },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "KGALGMDJIDD",
            |m: &MNODDHJLJKH| { &m.KGALGMDJIDD },
            |m: &mut MNODDHJLJKH| { &mut m.KGALGMDJIDD },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<MNODDHJLJKH>(
            "MNODDHJLJKH",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for MNODDHJLJKH {
    const NAME: &'static str = "MNODDHJLJKH";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                90 => {
                    is.read_repeated_packed_uint32_into(&mut self.KGILLKDLGFK)?;
                },
                88 => {
                    self.KGILLKDLGFK.push(is.read_uint32()?);
                },
                82 => {
                    is.read_repeated_packed_uint32_into(&mut self.KGALGMDJIDD)?;
                },
                80 => {
                    self.KGALGMDJIDD.push(is.read_uint32()?);
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
        for value in &self.KGILLKDLGFK {
            my_size += ::protobuf::rt::uint32_size(11, *value);
        };
        for value in &self.KGALGMDJIDD {
            my_size += ::protobuf::rt::uint32_size(10, *value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        for v in &self.KGILLKDLGFK {
            os.write_uint32(11, *v)?;
        };
        for v in &self.KGALGMDJIDD {
            os.write_uint32(10, *v)?;
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

    fn new() -> MNODDHJLJKH {
        MNODDHJLJKH::new()
    }

    fn clear(&mut self) {
        self.KGILLKDLGFK.clear();
        self.KGALGMDJIDD.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static MNODDHJLJKH {
        static instance: MNODDHJLJKH = MNODDHJLJKH {
            KGILLKDLGFK: ::std::vec::Vec::new(),
            KGALGMDJIDD: ::std::vec::Vec::new(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for MNODDHJLJKH {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("MNODDHJLJKH").unwrap()).clone()
    }
}

impl ::std::fmt::Display for MNODDHJLJKH {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for MNODDHJLJKH {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11MNODDHJLJKH.proto\"Q\n\x0bMNODDHJLJKH\x12\x20\n\x0bKGILLKDLGFK\x18\
    \x0b\x20\x03(\rR\x0bKGILLKDLGFK\x12\x20\n\x0bKGALGMDJIDD\x18\n\x20\x03(\
    \rR\x0bKGALGMDJIDDb\x06proto3\
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
            messages.push(MNODDHJLJKH::generated_message_descriptor_data());
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