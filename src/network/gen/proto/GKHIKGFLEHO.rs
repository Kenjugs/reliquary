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

//! Generated file from `GKHIKGFLEHO.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:GKHIKGFLEHO)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct GKHIKGFLEHO {
    // message fields
    // @@protoc_insertion_point(field:GKHIKGFLEHO.HNEPCEBMJKJ)
    pub HNEPCEBMJKJ: u32,
    // @@protoc_insertion_point(field:GKHIKGFLEHO.NPONEEKCLOH)
    pub NPONEEKCLOH: u32,
    // special fields
    // @@protoc_insertion_point(special_field:GKHIKGFLEHO.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a GKHIKGFLEHO {
    fn default() -> &'a GKHIKGFLEHO {
        <GKHIKGFLEHO as ::protobuf::Message>::default_instance()
    }
}

impl GKHIKGFLEHO {
    pub fn new() -> GKHIKGFLEHO {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(2);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "HNEPCEBMJKJ",
            |m: &GKHIKGFLEHO| { &m.HNEPCEBMJKJ },
            |m: &mut GKHIKGFLEHO| { &mut m.HNEPCEBMJKJ },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "NPONEEKCLOH",
            |m: &GKHIKGFLEHO| { &m.NPONEEKCLOH },
            |m: &mut GKHIKGFLEHO| { &mut m.NPONEEKCLOH },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<GKHIKGFLEHO>(
            "GKHIKGFLEHO",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for GKHIKGFLEHO {
    const NAME: &'static str = "GKHIKGFLEHO";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                8 => {
                    self.HNEPCEBMJKJ = is.read_uint32()?;
                },
                56 => {
                    self.NPONEEKCLOH = is.read_uint32()?;
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
        if self.HNEPCEBMJKJ != 0 {
            my_size += ::protobuf::rt::uint32_size(1, self.HNEPCEBMJKJ);
        }
        if self.NPONEEKCLOH != 0 {
            my_size += ::protobuf::rt::uint32_size(7, self.NPONEEKCLOH);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.HNEPCEBMJKJ != 0 {
            os.write_uint32(1, self.HNEPCEBMJKJ)?;
        }
        if self.NPONEEKCLOH != 0 {
            os.write_uint32(7, self.NPONEEKCLOH)?;
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

    fn new() -> GKHIKGFLEHO {
        GKHIKGFLEHO::new()
    }

    fn clear(&mut self) {
        self.HNEPCEBMJKJ = 0;
        self.NPONEEKCLOH = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static GKHIKGFLEHO {
        static instance: GKHIKGFLEHO = GKHIKGFLEHO {
            HNEPCEBMJKJ: 0,
            NPONEEKCLOH: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for GKHIKGFLEHO {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("GKHIKGFLEHO").unwrap()).clone()
    }
}

impl ::std::fmt::Display for GKHIKGFLEHO {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for GKHIKGFLEHO {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11GKHIKGFLEHO.proto\"Q\n\x0bGKHIKGFLEHO\x12\x20\n\x0bHNEPCEBMJKJ\x18\
    \x01\x20\x01(\rR\x0bHNEPCEBMJKJ\x12\x20\n\x0bNPONEEKCLOH\x18\x07\x20\x01\
    (\rR\x0bNPONEEKCLOHb\x06proto3\
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
            messages.push(GKHIKGFLEHO::generated_message_descriptor_data());
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
