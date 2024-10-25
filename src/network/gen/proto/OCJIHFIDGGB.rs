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

//! Generated file from `OCJIHFIDGGB.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:OCJIHFIDGGB)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct OCJIHFIDGGB {
    // message fields
    // @@protoc_insertion_point(field:OCJIHFIDGGB.NHADNDCNFMA)
    pub NHADNDCNFMA: u32,
    // message oneof groups
    pub EFJJGIIKNEH: ::std::option::Option<ocjihfidggb::EFJJGIIKNEH>,
    // special fields
    // @@protoc_insertion_point(special_field:OCJIHFIDGGB.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a OCJIHFIDGGB {
    fn default() -> &'a OCJIHFIDGGB {
        <OCJIHFIDGGB as ::protobuf::Message>::default_instance()
    }
}

impl OCJIHFIDGGB {
    pub fn new() -> OCJIHFIDGGB {
        ::std::default::Default::default()
    }

    // .ItemList DKKJOEJNIAK = 1;

    pub fn DKKJOEJNIAK(&self) -> &super::ItemList::ItemList {
        match self.EFJJGIIKNEH {
            ::std::option::Option::Some(ocjihfidggb::EFJJGIIKNEH::DKKJOEJNIAK(ref v)) => v,
            _ => <super::ItemList::ItemList as ::protobuf::Message>::default_instance(),
        }
    }

    pub fn clear_DKKJOEJNIAK(&mut self) {
        self.EFJJGIIKNEH = ::std::option::Option::None;
    }

    pub fn has_DKKJOEJNIAK(&self) -> bool {
        match self.EFJJGIIKNEH {
            ::std::option::Option::Some(ocjihfidggb::EFJJGIIKNEH::DKKJOEJNIAK(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_DKKJOEJNIAK(&mut self, v: super::ItemList::ItemList) {
        self.EFJJGIIKNEH = ::std::option::Option::Some(ocjihfidggb::EFJJGIIKNEH::DKKJOEJNIAK(v))
    }

    // Mutable pointer to the field.
    pub fn mut_DKKJOEJNIAK(&mut self) -> &mut super::ItemList::ItemList {
        if let ::std::option::Option::Some(ocjihfidggb::EFJJGIIKNEH::DKKJOEJNIAK(_)) = self.EFJJGIIKNEH {
        } else {
            self.EFJJGIIKNEH = ::std::option::Option::Some(ocjihfidggb::EFJJGIIKNEH::DKKJOEJNIAK(super::ItemList::ItemList::new()));
        }
        match self.EFJJGIIKNEH {
            ::std::option::Option::Some(ocjihfidggb::EFJJGIIKNEH::DKKJOEJNIAK(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_DKKJOEJNIAK(&mut self) -> super::ItemList::ItemList {
        if self.has_DKKJOEJNIAK() {
            match self.EFJJGIIKNEH.take() {
                ::std::option::Option::Some(ocjihfidggb::EFJJGIIKNEH::DKKJOEJNIAK(v)) => v,
                _ => panic!(),
            }
        } else {
            super::ItemList::ItemList::new()
        }
    }

    // .NGJDOBPCNNH DPEOJKICBCD = 7;

    pub fn DPEOJKICBCD(&self) -> &super::NGJDOBPCNNH::NGJDOBPCNNH {
        match self.EFJJGIIKNEH {
            ::std::option::Option::Some(ocjihfidggb::EFJJGIIKNEH::DPEOJKICBCD(ref v)) => v,
            _ => <super::NGJDOBPCNNH::NGJDOBPCNNH as ::protobuf::Message>::default_instance(),
        }
    }

    pub fn clear_DPEOJKICBCD(&mut self) {
        self.EFJJGIIKNEH = ::std::option::Option::None;
    }

    pub fn has_DPEOJKICBCD(&self) -> bool {
        match self.EFJJGIIKNEH {
            ::std::option::Option::Some(ocjihfidggb::EFJJGIIKNEH::DPEOJKICBCD(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_DPEOJKICBCD(&mut self, v: super::NGJDOBPCNNH::NGJDOBPCNNH) {
        self.EFJJGIIKNEH = ::std::option::Option::Some(ocjihfidggb::EFJJGIIKNEH::DPEOJKICBCD(v))
    }

    // Mutable pointer to the field.
    pub fn mut_DPEOJKICBCD(&mut self) -> &mut super::NGJDOBPCNNH::NGJDOBPCNNH {
        if let ::std::option::Option::Some(ocjihfidggb::EFJJGIIKNEH::DPEOJKICBCD(_)) = self.EFJJGIIKNEH {
        } else {
            self.EFJJGIIKNEH = ::std::option::Option::Some(ocjihfidggb::EFJJGIIKNEH::DPEOJKICBCD(super::NGJDOBPCNNH::NGJDOBPCNNH::new()));
        }
        match self.EFJJGIIKNEH {
            ::std::option::Option::Some(ocjihfidggb::EFJJGIIKNEH::DPEOJKICBCD(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_DPEOJKICBCD(&mut self) -> super::NGJDOBPCNNH::NGJDOBPCNNH {
        if self.has_DPEOJKICBCD() {
            match self.EFJJGIIKNEH.take() {
                ::std::option::Option::Some(ocjihfidggb::EFJJGIIKNEH::DPEOJKICBCD(v)) => v,
                _ => panic!(),
            }
        } else {
            super::NGJDOBPCNNH::NGJDOBPCNNH::new()
        }
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(3);
        let mut oneofs = ::std::vec::Vec::with_capacity(1);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "NHADNDCNFMA",
            |m: &OCJIHFIDGGB| { &m.NHADNDCNFMA },
            |m: &mut OCJIHFIDGGB| { &mut m.NHADNDCNFMA },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_oneof_message_has_get_mut_set_accessor::<_, super::ItemList::ItemList>(
            "DKKJOEJNIAK",
            OCJIHFIDGGB::has_DKKJOEJNIAK,
            OCJIHFIDGGB::DKKJOEJNIAK,
            OCJIHFIDGGB::mut_DKKJOEJNIAK,
            OCJIHFIDGGB::set_DKKJOEJNIAK,
        ));
        fields.push(::protobuf::reflect::rt::v2::make_oneof_message_has_get_mut_set_accessor::<_, super::NGJDOBPCNNH::NGJDOBPCNNH>(
            "DPEOJKICBCD",
            OCJIHFIDGGB::has_DPEOJKICBCD,
            OCJIHFIDGGB::DPEOJKICBCD,
            OCJIHFIDGGB::mut_DPEOJKICBCD,
            OCJIHFIDGGB::set_DPEOJKICBCD,
        ));
        oneofs.push(ocjihfidggb::EFJJGIIKNEH::generated_oneof_descriptor_data());
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<OCJIHFIDGGB>(
            "OCJIHFIDGGB",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for OCJIHFIDGGB {
    const NAME: &'static str = "OCJIHFIDGGB";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                120 => {
                    self.NHADNDCNFMA = is.read_uint32()?;
                },
                10 => {
                    self.EFJJGIIKNEH = ::std::option::Option::Some(ocjihfidggb::EFJJGIIKNEH::DKKJOEJNIAK(is.read_message()?));
                },
                58 => {
                    self.EFJJGIIKNEH = ::std::option::Option::Some(ocjihfidggb::EFJJGIIKNEH::DPEOJKICBCD(is.read_message()?));
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
        if self.NHADNDCNFMA != 0 {
            my_size += ::protobuf::rt::uint32_size(15, self.NHADNDCNFMA);
        }
        if let ::std::option::Option::Some(ref v) = self.EFJJGIIKNEH {
            match v {
                &ocjihfidggb::EFJJGIIKNEH::DKKJOEJNIAK(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
                },
                &ocjihfidggb::EFJJGIIKNEH::DPEOJKICBCD(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
                },
            };
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.NHADNDCNFMA != 0 {
            os.write_uint32(15, self.NHADNDCNFMA)?;
        }
        if let ::std::option::Option::Some(ref v) = self.EFJJGIIKNEH {
            match v {
                &ocjihfidggb::EFJJGIIKNEH::DKKJOEJNIAK(ref v) => {
                    ::protobuf::rt::write_message_field_with_cached_size(1, v, os)?;
                },
                &ocjihfidggb::EFJJGIIKNEH::DPEOJKICBCD(ref v) => {
                    ::protobuf::rt::write_message_field_with_cached_size(7, v, os)?;
                },
            };
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

    fn new() -> OCJIHFIDGGB {
        OCJIHFIDGGB::new()
    }

    fn clear(&mut self) {
        self.NHADNDCNFMA = 0;
        self.EFJJGIIKNEH = ::std::option::Option::None;
        self.EFJJGIIKNEH = ::std::option::Option::None;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static OCJIHFIDGGB {
        static instance: OCJIHFIDGGB = OCJIHFIDGGB {
            NHADNDCNFMA: 0,
            EFJJGIIKNEH: ::std::option::Option::None,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for OCJIHFIDGGB {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("OCJIHFIDGGB").unwrap()).clone()
    }
}

impl ::std::fmt::Display for OCJIHFIDGGB {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for OCJIHFIDGGB {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

/// Nested message and enums of message `OCJIHFIDGGB`
pub mod ocjihfidggb {

    #[derive(Clone,PartialEq,Debug)]
    #[non_exhaustive]
    // @@protoc_insertion_point(oneof:OCJIHFIDGGB.EFJJGIIKNEH)
    pub enum EFJJGIIKNEH {
        // @@protoc_insertion_point(oneof_field:OCJIHFIDGGB.DKKJOEJNIAK)
        DKKJOEJNIAK(super::super::ItemList::ItemList),
        // @@protoc_insertion_point(oneof_field:OCJIHFIDGGB.DPEOJKICBCD)
        DPEOJKICBCD(super::super::NGJDOBPCNNH::NGJDOBPCNNH),
    }

    impl ::protobuf::Oneof for EFJJGIIKNEH {
    }

    impl ::protobuf::OneofFull for EFJJGIIKNEH {
        fn descriptor() -> ::protobuf::reflect::OneofDescriptor {
            static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::OneofDescriptor> = ::protobuf::rt::Lazy::new();
            descriptor.get(|| <super::OCJIHFIDGGB as ::protobuf::MessageFull>::descriptor().oneof_by_name("EFJJGIIKNEH").unwrap()).clone()
        }
    }

    impl EFJJGIIKNEH {
        pub(in super) fn generated_oneof_descriptor_data() -> ::protobuf::reflect::GeneratedOneofDescriptorData {
            ::protobuf::reflect::GeneratedOneofDescriptorData::new::<EFJJGIIKNEH>("EFJJGIIKNEH")
        }
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11OCJIHFIDGGB.proto\x1a\x0eItemList.proto\x1a\x11NGJDOBPCNNH.proto\"\
    \x9f\x01\n\x0bOCJIHFIDGGB\x12\x20\n\x0bNHADNDCNFMA\x18\x0f\x20\x01(\rR\
    \x0bNHADNDCNFMA\x12-\n\x0bDKKJOEJNIAK\x18\x01\x20\x01(\x0b2\t.ItemListH\
    \0R\x0bDKKJOEJNIAK\x120\n\x0bDPEOJKICBCD\x18\x07\x20\x01(\x0b2\x0c.NGJDO\
    BPCNNHH\0R\x0bDPEOJKICBCDB\r\n\x0bEFJJGIIKNEHb\x06proto3\
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
            let mut deps = ::std::vec::Vec::with_capacity(2);
            deps.push(super::ItemList::file_descriptor().clone());
            deps.push(super::NGJDOBPCNNH::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(OCJIHFIDGGB::generated_message_descriptor_data());
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
