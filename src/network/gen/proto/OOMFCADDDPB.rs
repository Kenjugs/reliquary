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

//! Generated file from `OOMFCADDDPB.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:OOMFCADDDPB)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct OOMFCADDDPB {
    // message fields
    // @@protoc_insertion_point(field:OOMFCADDDPB.GACLCFENKLM)
    pub GACLCFENKLM: ::std::collections::HashMap<u32, u32>,
    // @@protoc_insertion_point(field:OOMFCADDDPB.NAJELOIIAIH)
    pub NAJELOIIAIH: u32,
    // @@protoc_insertion_point(field:OOMFCADDDPB.HPHIIJLNKAE)
    pub HPHIIJLNKAE: u32,
    // special fields
    // @@protoc_insertion_point(special_field:OOMFCADDDPB.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a OOMFCADDDPB {
    fn default() -> &'a OOMFCADDDPB {
        <OOMFCADDDPB as ::protobuf::Message>::default_instance()
    }
}

impl OOMFCADDDPB {
    pub fn new() -> OOMFCADDDPB {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(3);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_map_simpler_accessor::<_, _, _>(
            "GACLCFENKLM",
            |m: &OOMFCADDDPB| { &m.GACLCFENKLM },
            |m: &mut OOMFCADDDPB| { &mut m.GACLCFENKLM },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "NAJELOIIAIH",
            |m: &OOMFCADDDPB| { &m.NAJELOIIAIH },
            |m: &mut OOMFCADDDPB| { &mut m.NAJELOIIAIH },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "HPHIIJLNKAE",
            |m: &OOMFCADDDPB| { &m.HPHIIJLNKAE },
            |m: &mut OOMFCADDDPB| { &mut m.HPHIIJLNKAE },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<OOMFCADDDPB>(
            "OOMFCADDDPB",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for OOMFCADDDPB {
    const NAME: &'static str = "OOMFCADDDPB";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
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
                    self.GACLCFENKLM.insert(key, value);
                },
                88 => {
                    self.NAJELOIIAIH = is.read_uint32()?;
                },
                16 => {
                    self.HPHIIJLNKAE = is.read_uint32()?;
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
        for (k, v) in &self.GACLCFENKLM {
            let mut entry_size = 0;
            entry_size += ::protobuf::rt::uint32_size(1, *k);
            entry_size += ::protobuf::rt::uint32_size(2, *v);
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(entry_size) + entry_size
        };
        if self.NAJELOIIAIH != 0 {
            my_size += ::protobuf::rt::uint32_size(11, self.NAJELOIIAIH);
        }
        if self.HPHIIJLNKAE != 0 {
            my_size += ::protobuf::rt::uint32_size(2, self.HPHIIJLNKAE);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        for (k, v) in &self.GACLCFENKLM {
            let mut entry_size = 0;
            entry_size += ::protobuf::rt::uint32_size(1, *k);
            entry_size += ::protobuf::rt::uint32_size(2, *v);
            os.write_raw_varint32(34)?; // Tag.
            os.write_raw_varint32(entry_size as u32)?;
            os.write_uint32(1, *k)?;
            os.write_uint32(2, *v)?;
        };
        if self.NAJELOIIAIH != 0 {
            os.write_uint32(11, self.NAJELOIIAIH)?;
        }
        if self.HPHIIJLNKAE != 0 {
            os.write_uint32(2, self.HPHIIJLNKAE)?;
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

    fn new() -> OOMFCADDDPB {
        OOMFCADDDPB::new()
    }

    fn clear(&mut self) {
        self.GACLCFENKLM.clear();
        self.NAJELOIIAIH = 0;
        self.HPHIIJLNKAE = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static OOMFCADDDPB {
        static instance: ::protobuf::rt::Lazy<OOMFCADDDPB> = ::protobuf::rt::Lazy::new();
        instance.get(OOMFCADDDPB::new)
    }
}

impl ::protobuf::MessageFull for OOMFCADDDPB {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("OOMFCADDDPB").unwrap()).clone()
    }
}

impl ::std::fmt::Display for OOMFCADDDPB {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for OOMFCADDDPB {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11OOMFCADDDPB.proto\"\xd2\x01\n\x0bOOMFCADDDPB\x12?\n\x0bGACLCFENKLM\
    \x18\x04\x20\x03(\x0b2\x1d.OOMFCADDDPB.GACLCFENKLMEntryR\x0bGACLCFENKLM\
    \x12\x20\n\x0bNAJELOIIAIH\x18\x0b\x20\x01(\rR\x0bNAJELOIIAIH\x12\x20\n\
    \x0bHPHIIJLNKAE\x18\x02\x20\x01(\rR\x0bHPHIIJLNKAE\x1a>\n\x10GACLCFENKLM\
    Entry\x12\x10\n\x03key\x18\x01\x20\x01(\rR\x03key\x12\x14\n\x05value\x18\
    \x02\x20\x01(\rR\x05value:\x028\x01b\x06proto3\
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
            messages.push(OOMFCADDDPB::generated_message_descriptor_data());
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