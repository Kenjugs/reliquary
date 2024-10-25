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

//! Generated file from `DANPADNDDGD.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:DANPADNDDGD)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct DANPADNDDGD {
    // message fields
    // @@protoc_insertion_point(field:DANPADNDDGD.KANKGGIGMJK)
    pub KANKGGIGMJK: ::protobuf::MessageField<super::KOHEAIPMFKB::KOHEAIPMFKB>,
    // message oneof groups
    pub MEFKKGMHAOK: ::std::option::Option<danpadnddgd::MEFKKGMHAOK>,
    // special fields
    // @@protoc_insertion_point(special_field:DANPADNDDGD.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a DANPADNDDGD {
    fn default() -> &'a DANPADNDDGD {
        <DANPADNDDGD as ::protobuf::Message>::default_instance()
    }
}

impl DANPADNDDGD {
    pub fn new() -> DANPADNDDGD {
        ::std::default::Default::default()
    }

    // .OONEOFNPFCC NLMGHEELMHE = 296;

    pub fn NLMGHEELMHE(&self) -> &super::OONEOFNPFCC::OONEOFNPFCC {
        match self.MEFKKGMHAOK {
            ::std::option::Option::Some(danpadnddgd::MEFKKGMHAOK::NLMGHEELMHE(ref v)) => v,
            _ => <super::OONEOFNPFCC::OONEOFNPFCC as ::protobuf::Message>::default_instance(),
        }
    }

    pub fn clear_NLMGHEELMHE(&mut self) {
        self.MEFKKGMHAOK = ::std::option::Option::None;
    }

    pub fn has_NLMGHEELMHE(&self) -> bool {
        match self.MEFKKGMHAOK {
            ::std::option::Option::Some(danpadnddgd::MEFKKGMHAOK::NLMGHEELMHE(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_NLMGHEELMHE(&mut self, v: super::OONEOFNPFCC::OONEOFNPFCC) {
        self.MEFKKGMHAOK = ::std::option::Option::Some(danpadnddgd::MEFKKGMHAOK::NLMGHEELMHE(v))
    }

    // Mutable pointer to the field.
    pub fn mut_NLMGHEELMHE(&mut self) -> &mut super::OONEOFNPFCC::OONEOFNPFCC {
        if let ::std::option::Option::Some(danpadnddgd::MEFKKGMHAOK::NLMGHEELMHE(_)) = self.MEFKKGMHAOK {
        } else {
            self.MEFKKGMHAOK = ::std::option::Option::Some(danpadnddgd::MEFKKGMHAOK::NLMGHEELMHE(super::OONEOFNPFCC::OONEOFNPFCC::new()));
        }
        match self.MEFKKGMHAOK {
            ::std::option::Option::Some(danpadnddgd::MEFKKGMHAOK::NLMGHEELMHE(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_NLMGHEELMHE(&mut self) -> super::OONEOFNPFCC::OONEOFNPFCC {
        if self.has_NLMGHEELMHE() {
            match self.MEFKKGMHAOK.take() {
                ::std::option::Option::Some(danpadnddgd::MEFKKGMHAOK::NLMGHEELMHE(v)) => v,
                _ => panic!(),
            }
        } else {
            super::OONEOFNPFCC::OONEOFNPFCC::new()
        }
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(2);
        let mut oneofs = ::std::vec::Vec::with_capacity(1);
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::KOHEAIPMFKB::KOHEAIPMFKB>(
            "KANKGGIGMJK",
            |m: &DANPADNDDGD| { &m.KANKGGIGMJK },
            |m: &mut DANPADNDDGD| { &mut m.KANKGGIGMJK },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_oneof_message_has_get_mut_set_accessor::<_, super::OONEOFNPFCC::OONEOFNPFCC>(
            "NLMGHEELMHE",
            DANPADNDDGD::has_NLMGHEELMHE,
            DANPADNDDGD::NLMGHEELMHE,
            DANPADNDDGD::mut_NLMGHEELMHE,
            DANPADNDDGD::set_NLMGHEELMHE,
        ));
        oneofs.push(danpadnddgd::MEFKKGMHAOK::generated_oneof_descriptor_data());
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<DANPADNDDGD>(
            "DANPADNDDGD",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for DANPADNDDGD {
    const NAME: &'static str = "DANPADNDDGD";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                98 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.KANKGGIGMJK)?;
                },
                2370 => {
                    self.MEFKKGMHAOK = ::std::option::Option::Some(danpadnddgd::MEFKKGMHAOK::NLMGHEELMHE(is.read_message()?));
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
        if let Some(v) = self.KANKGGIGMJK.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if let ::std::option::Option::Some(ref v) = self.MEFKKGMHAOK {
            match v {
                &danpadnddgd::MEFKKGMHAOK::NLMGHEELMHE(ref v) => {
                    let len = v.compute_size();
                    my_size += 2 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
                },
            };
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if let Some(v) = self.KANKGGIGMJK.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(12, v, os)?;
        }
        if let ::std::option::Option::Some(ref v) = self.MEFKKGMHAOK {
            match v {
                &danpadnddgd::MEFKKGMHAOK::NLMGHEELMHE(ref v) => {
                    ::protobuf::rt::write_message_field_with_cached_size(296, v, os)?;
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

    fn new() -> DANPADNDDGD {
        DANPADNDDGD::new()
    }

    fn clear(&mut self) {
        self.KANKGGIGMJK.clear();
        self.MEFKKGMHAOK = ::std::option::Option::None;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static DANPADNDDGD {
        static instance: DANPADNDDGD = DANPADNDDGD {
            KANKGGIGMJK: ::protobuf::MessageField::none(),
            MEFKKGMHAOK: ::std::option::Option::None,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for DANPADNDDGD {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("DANPADNDDGD").unwrap()).clone()
    }
}

impl ::std::fmt::Display for DANPADNDDGD {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for DANPADNDDGD {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

/// Nested message and enums of message `DANPADNDDGD`
pub mod danpadnddgd {

    #[derive(Clone,PartialEq,Debug)]
    #[non_exhaustive]
    // @@protoc_insertion_point(oneof:DANPADNDDGD.MEFKKGMHAOK)
    pub enum MEFKKGMHAOK {
        // @@protoc_insertion_point(oneof_field:DANPADNDDGD.NLMGHEELMHE)
        NLMGHEELMHE(super::super::OONEOFNPFCC::OONEOFNPFCC),
    }

    impl ::protobuf::Oneof for MEFKKGMHAOK {
    }

    impl ::protobuf::OneofFull for MEFKKGMHAOK {
        fn descriptor() -> ::protobuf::reflect::OneofDescriptor {
            static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::OneofDescriptor> = ::protobuf::rt::Lazy::new();
            descriptor.get(|| <super::DANPADNDDGD as ::protobuf::MessageFull>::descriptor().oneof_by_name("MEFKKGMHAOK").unwrap()).clone()
        }
    }

    impl MEFKKGMHAOK {
        pub(in super) fn generated_oneof_descriptor_data() -> ::protobuf::reflect::GeneratedOneofDescriptorData {
            ::protobuf::reflect::GeneratedOneofDescriptorData::new::<MEFKKGMHAOK>("MEFKKGMHAOK")
        }
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11DANPADNDDGD.proto\x1a\x11KOHEAIPMFKB.proto\x1a\x11OONEOFNPFCC.prot\
    o\"\x7f\n\x0bDANPADNDDGD\x12.\n\x0bKANKGGIGMJK\x18\x0c\x20\x01(\x0b2\x0c\
    .KOHEAIPMFKBR\x0bKANKGGIGMJK\x121\n\x0bNLMGHEELMHE\x18\xa8\x02\x20\x01(\
    \x0b2\x0c.OONEOFNPFCCH\0R\x0bNLMGHEELMHEB\r\n\x0bMEFKKGMHAOKb\x06proto3\
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
            deps.push(super::KOHEAIPMFKB::file_descriptor().clone());
            deps.push(super::OONEOFNPFCC::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(DANPADNDDGD::generated_message_descriptor_data());
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
