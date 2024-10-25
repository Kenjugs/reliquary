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

//! Generated file from `PPKNNAOJJMK.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:PPKNNAOJJMK)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct PPKNNAOJJMK {
    // message fields
    // @@protoc_insertion_point(field:PPKNNAOJJMK.FNGENOKBNHL)
    pub FNGENOKBNHL: u32,
    // @@protoc_insertion_point(field:PPKNNAOJJMK.FFIDBDCECOD)
    pub FFIDBDCECOD: ::std::vec::Vec<super::MHKCHGMGODK::MHKCHGMGODK>,
    // @@protoc_insertion_point(field:PPKNNAOJJMK.level)
    pub level: u32,
    // @@protoc_insertion_point(field:PPKNNAOJJMK.FHKLLHPOFMH)
    pub FHKLLHPOFMH: ::std::collections::HashMap<u32, u32>,
    // special fields
    // @@protoc_insertion_point(special_field:PPKNNAOJJMK.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a PPKNNAOJJMK {
    fn default() -> &'a PPKNNAOJJMK {
        <PPKNNAOJJMK as ::protobuf::Message>::default_instance()
    }
}

impl PPKNNAOJJMK {
    pub fn new() -> PPKNNAOJJMK {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(4);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "FNGENOKBNHL",
            |m: &PPKNNAOJJMK| { &m.FNGENOKBNHL },
            |m: &mut PPKNNAOJJMK| { &mut m.FNGENOKBNHL },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "FFIDBDCECOD",
            |m: &PPKNNAOJJMK| { &m.FFIDBDCECOD },
            |m: &mut PPKNNAOJJMK| { &mut m.FFIDBDCECOD },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "level",
            |m: &PPKNNAOJJMK| { &m.level },
            |m: &mut PPKNNAOJJMK| { &mut m.level },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_map_simpler_accessor::<_, _, _>(
            "FHKLLHPOFMH",
            |m: &PPKNNAOJJMK| { &m.FHKLLHPOFMH },
            |m: &mut PPKNNAOJJMK| { &mut m.FHKLLHPOFMH },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<PPKNNAOJJMK>(
            "PPKNNAOJJMK",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for PPKNNAOJJMK {
    const NAME: &'static str = "PPKNNAOJJMK";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                8 => {
                    self.FNGENOKBNHL = is.read_uint32()?;
                },
                18 => {
                    self.FFIDBDCECOD.push(is.read_message()?);
                },
                24 => {
                    self.level = is.read_uint32()?;
                },
                34 => {
                    let len = is.read_raw_varint32()?;
                    let old_limit = is.push_limit(len as u64)?;
                    let mut key = ::std::default::Default::default();
                    let mut value = ::std::default::Default::default();
                    while let Some(tag) = is.read_raw_tag_or_eof()? {
                        match tag {
                            8 => key = is.read_uint32()?,
                            16 => value = is.read_uint32()?,
                            _ => ::protobuf::rt::skip_field_for_tag(tag, is)?,
                        };
                    }
                    is.pop_limit(old_limit);
                    self.FHKLLHPOFMH.insert(key, value);
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
        if self.FNGENOKBNHL != 0 {
            my_size += ::protobuf::rt::uint32_size(1, self.FNGENOKBNHL);
        }
        for value in &self.FFIDBDCECOD {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        if self.level != 0 {
            my_size += ::protobuf::rt::uint32_size(3, self.level);
        }
        for (k, v) in &self.FHKLLHPOFMH {
            let mut entry_size = 0;
            entry_size += ::protobuf::rt::uint32_size(1, *k);
            entry_size += ::protobuf::rt::uint32_size(2, *v);
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(entry_size) + entry_size
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.FNGENOKBNHL != 0 {
            os.write_uint32(1, self.FNGENOKBNHL)?;
        }
        for v in &self.FFIDBDCECOD {
            ::protobuf::rt::write_message_field_with_cached_size(2, v, os)?;
        };
        if self.level != 0 {
            os.write_uint32(3, self.level)?;
        }
        for (k, v) in &self.FHKLLHPOFMH {
            let mut entry_size = 0;
            entry_size += ::protobuf::rt::uint32_size(1, *k);
            entry_size += ::protobuf::rt::uint32_size(2, *v);
            os.write_raw_varint32(34)?; // Tag.
            os.write_raw_varint32(entry_size as u32)?;
            os.write_uint32(1, *k)?;
            os.write_uint32(2, *v)?;
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

    fn new() -> PPKNNAOJJMK {
        PPKNNAOJJMK::new()
    }

    fn clear(&mut self) {
        self.FNGENOKBNHL = 0;
        self.FFIDBDCECOD.clear();
        self.level = 0;
        self.FHKLLHPOFMH.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static PPKNNAOJJMK {
        static instance: ::protobuf::rt::Lazy<PPKNNAOJJMK> = ::protobuf::rt::Lazy::new();
        instance.get(PPKNNAOJJMK::new)
    }
}

impl ::protobuf::MessageFull for PPKNNAOJJMK {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("PPKNNAOJJMK").unwrap()).clone()
    }
}

impl ::std::fmt::Display for PPKNNAOJJMK {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for PPKNNAOJJMK {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11PPKNNAOJJMK.proto\x1a\x11MHKCHGMGODK.proto\"\xf6\x01\n\x0bPPKNNAOJ\
    JMK\x12\x20\n\x0bFNGENOKBNHL\x18\x01\x20\x01(\rR\x0bFNGENOKBNHL\x12.\n\
    \x0bFFIDBDCECOD\x18\x02\x20\x03(\x0b2\x0c.MHKCHGMGODKR\x0bFFIDBDCECOD\
    \x12\x14\n\x05level\x18\x03\x20\x01(\rR\x05level\x12?\n\x0bFHKLLHPOFMH\
    \x18\x04\x20\x03(\x0b2\x1d.PPKNNAOJJMK.FHKLLHPOFMHEntryR\x0bFHKLLHPOFMH\
    \x1a>\n\x10FHKLLHPOFMHEntry\x12\x10\n\x03key\x18\x01\x20\x01(\rR\x03key\
    \x12\x14\n\x05value\x18\x02\x20\x01(\rR\x05value:\x028\x01b\x06proto3\
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
            deps.push(super::MHKCHGMGODK::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(PPKNNAOJJMK::generated_message_descriptor_data());
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
