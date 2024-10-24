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

//! Generated file from `IDFECBMCHMA.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:IDFECBMCHMA)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct IDFECBMCHMA {
    // message fields
    // @@protoc_insertion_point(field:IDFECBMCHMA.PJBNNFPEMFO)
    pub PJBNNFPEMFO: u32,
    // @@protoc_insertion_point(field:IDFECBMCHMA.DBIBGOLGHAN)
    pub DBIBGOLGHAN: ::std::vec::Vec<super::ItemList::ItemList>,
    // @@protoc_insertion_point(field:IDFECBMCHMA.CJDEBPDJPKC)
    pub CJDEBPDJPKC: ::std::vec::Vec<super::MEDLIGEEBNO::MEDLIGEEBNO>,
    // @@protoc_insertion_point(field:IDFECBMCHMA.IJODPIOGLHG)
    pub IJODPIOGLHG: ::protobuf::MessageField<super::GEHFGPNEMNC::GEHFGPNEMNC>,
    // @@protoc_insertion_point(field:IDFECBMCHMA.ALEENEJHPHM)
    pub ALEENEJHPHM: u32,
    // special fields
    // @@protoc_insertion_point(special_field:IDFECBMCHMA.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a IDFECBMCHMA {
    fn default() -> &'a IDFECBMCHMA {
        <IDFECBMCHMA as ::protobuf::Message>::default_instance()
    }
}

impl IDFECBMCHMA {
    pub fn new() -> IDFECBMCHMA {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(5);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "PJBNNFPEMFO",
            |m: &IDFECBMCHMA| { &m.PJBNNFPEMFO },
            |m: &mut IDFECBMCHMA| { &mut m.PJBNNFPEMFO },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "DBIBGOLGHAN",
            |m: &IDFECBMCHMA| { &m.DBIBGOLGHAN },
            |m: &mut IDFECBMCHMA| { &mut m.DBIBGOLGHAN },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "CJDEBPDJPKC",
            |m: &IDFECBMCHMA| { &m.CJDEBPDJPKC },
            |m: &mut IDFECBMCHMA| { &mut m.CJDEBPDJPKC },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::GEHFGPNEMNC::GEHFGPNEMNC>(
            "IJODPIOGLHG",
            |m: &IDFECBMCHMA| { &m.IJODPIOGLHG },
            |m: &mut IDFECBMCHMA| { &mut m.IJODPIOGLHG },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "ALEENEJHPHM",
            |m: &IDFECBMCHMA| { &m.ALEENEJHPHM },
            |m: &mut IDFECBMCHMA| { &mut m.ALEENEJHPHM },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<IDFECBMCHMA>(
            "IDFECBMCHMA",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for IDFECBMCHMA {
    const NAME: &'static str = "IDFECBMCHMA";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                56 => {
                    self.PJBNNFPEMFO = is.read_uint32()?;
                },
                90 => {
                    self.DBIBGOLGHAN.push(is.read_message()?);
                },
                74 => {
                    self.CJDEBPDJPKC.push(is.read_message()?);
                },
                50 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.IJODPIOGLHG)?;
                },
                16 => {
                    self.ALEENEJHPHM = is.read_uint32()?;
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
        if self.PJBNNFPEMFO != 0 {
            my_size += ::protobuf::rt::uint32_size(7, self.PJBNNFPEMFO);
        }
        for value in &self.DBIBGOLGHAN {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        for value in &self.CJDEBPDJPKC {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        if let Some(v) = self.IJODPIOGLHG.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if self.ALEENEJHPHM != 0 {
            my_size += ::protobuf::rt::uint32_size(2, self.ALEENEJHPHM);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.PJBNNFPEMFO != 0 {
            os.write_uint32(7, self.PJBNNFPEMFO)?;
        }
        for v in &self.DBIBGOLGHAN {
            ::protobuf::rt::write_message_field_with_cached_size(11, v, os)?;
        };
        for v in &self.CJDEBPDJPKC {
            ::protobuf::rt::write_message_field_with_cached_size(9, v, os)?;
        };
        if let Some(v) = self.IJODPIOGLHG.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(6, v, os)?;
        }
        if self.ALEENEJHPHM != 0 {
            os.write_uint32(2, self.ALEENEJHPHM)?;
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

    fn new() -> IDFECBMCHMA {
        IDFECBMCHMA::new()
    }

    fn clear(&mut self) {
        self.PJBNNFPEMFO = 0;
        self.DBIBGOLGHAN.clear();
        self.CJDEBPDJPKC.clear();
        self.IJODPIOGLHG.clear();
        self.ALEENEJHPHM = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static IDFECBMCHMA {
        static instance: IDFECBMCHMA = IDFECBMCHMA {
            PJBNNFPEMFO: 0,
            DBIBGOLGHAN: ::std::vec::Vec::new(),
            CJDEBPDJPKC: ::std::vec::Vec::new(),
            IJODPIOGLHG: ::protobuf::MessageField::none(),
            ALEENEJHPHM: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for IDFECBMCHMA {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("IDFECBMCHMA").unwrap()).clone()
    }
}

impl ::std::fmt::Display for IDFECBMCHMA {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for IDFECBMCHMA {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11IDFECBMCHMA.proto\x1a\x11GEHFGPNEMNC.proto\x1a\x0eItemList.proto\
    \x1a\x11MEDLIGEEBNO.proto\"\xde\x01\n\x0bIDFECBMCHMA\x12\x20\n\x0bPJBNNF\
    PEMFO\x18\x07\x20\x01(\rR\x0bPJBNNFPEMFO\x12+\n\x0bDBIBGOLGHAN\x18\x0b\
    \x20\x03(\x0b2\t.ItemListR\x0bDBIBGOLGHAN\x12.\n\x0bCJDEBPDJPKC\x18\t\
    \x20\x03(\x0b2\x0c.MEDLIGEEBNOR\x0bCJDEBPDJPKC\x12.\n\x0bIJODPIOGLHG\x18\
    \x06\x20\x01(\x0b2\x0c.GEHFGPNEMNCR\x0bIJODPIOGLHG\x12\x20\n\x0bALEENEJH\
    PHM\x18\x02\x20\x01(\rR\x0bALEENEJHPHMb\x06proto3\
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
            let mut deps = ::std::vec::Vec::with_capacity(3);
            deps.push(super::GEHFGPNEMNC::file_descriptor().clone());
            deps.push(super::ItemList::file_descriptor().clone());
            deps.push(super::MEDLIGEEBNO::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(IDFECBMCHMA::generated_message_descriptor_data());
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
