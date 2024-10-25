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

//! Generated file from `KPBGPCMAAJA.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:KPBGPCMAAJA)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct KPBGPCMAAJA {
    // message fields
    // @@protoc_insertion_point(field:KPBGPCMAAJA.NPONEEKCLOH)
    pub NPONEEKCLOH: u32,
    // @@protoc_insertion_point(field:KPBGPCMAAJA.unique_id)
    pub unique_id: u64,
    // @@protoc_insertion_point(field:KPBGPCMAAJA.CIPAGEJLFKB)
    pub CIPAGEJLFKB: u32,
    // special fields
    // @@protoc_insertion_point(special_field:KPBGPCMAAJA.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a KPBGPCMAAJA {
    fn default() -> &'a KPBGPCMAAJA {
        <KPBGPCMAAJA as ::protobuf::Message>::default_instance()
    }
}

impl KPBGPCMAAJA {
    pub fn new() -> KPBGPCMAAJA {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(3);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "NPONEEKCLOH",
            |m: &KPBGPCMAAJA| { &m.NPONEEKCLOH },
            |m: &mut KPBGPCMAAJA| { &mut m.NPONEEKCLOH },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "unique_id",
            |m: &KPBGPCMAAJA| { &m.unique_id },
            |m: &mut KPBGPCMAAJA| { &mut m.unique_id },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "CIPAGEJLFKB",
            |m: &KPBGPCMAAJA| { &m.CIPAGEJLFKB },
            |m: &mut KPBGPCMAAJA| { &mut m.CIPAGEJLFKB },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<KPBGPCMAAJA>(
            "KPBGPCMAAJA",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for KPBGPCMAAJA {
    const NAME: &'static str = "KPBGPCMAAJA";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                72 => {
                    self.NPONEEKCLOH = is.read_uint32()?;
                },
                80 => {
                    self.unique_id = is.read_uint64()?;
                },
                24 => {
                    self.CIPAGEJLFKB = is.read_uint32()?;
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
        if self.NPONEEKCLOH != 0 {
            my_size += ::protobuf::rt::uint32_size(9, self.NPONEEKCLOH);
        }
        if self.unique_id != 0 {
            my_size += ::protobuf::rt::uint64_size(10, self.unique_id);
        }
        if self.CIPAGEJLFKB != 0 {
            my_size += ::protobuf::rt::uint32_size(3, self.CIPAGEJLFKB);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.NPONEEKCLOH != 0 {
            os.write_uint32(9, self.NPONEEKCLOH)?;
        }
        if self.unique_id != 0 {
            os.write_uint64(10, self.unique_id)?;
        }
        if self.CIPAGEJLFKB != 0 {
            os.write_uint32(3, self.CIPAGEJLFKB)?;
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

    fn new() -> KPBGPCMAAJA {
        KPBGPCMAAJA::new()
    }

    fn clear(&mut self) {
        self.NPONEEKCLOH = 0;
        self.unique_id = 0;
        self.CIPAGEJLFKB = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static KPBGPCMAAJA {
        static instance: KPBGPCMAAJA = KPBGPCMAAJA {
            NPONEEKCLOH: 0,
            unique_id: 0,
            CIPAGEJLFKB: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for KPBGPCMAAJA {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("KPBGPCMAAJA").unwrap()).clone()
    }
}

impl ::std::fmt::Display for KPBGPCMAAJA {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for KPBGPCMAAJA {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11KPBGPCMAAJA.proto\"n\n\x0bKPBGPCMAAJA\x12\x20\n\x0bNPONEEKCLOH\x18\
    \t\x20\x01(\rR\x0bNPONEEKCLOH\x12\x1b\n\tunique_id\x18\n\x20\x01(\x04R\
    \x08uniqueId\x12\x20\n\x0bCIPAGEJLFKB\x18\x03\x20\x01(\rR\x0bCIPAGEJLFKB\
    b\x06proto3\
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
            messages.push(KPBGPCMAAJA::generated_message_descriptor_data());
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