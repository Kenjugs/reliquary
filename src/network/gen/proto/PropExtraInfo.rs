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

//! Generated file from `PropExtraInfo.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:PropExtraInfo)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct PropExtraInfo {
    // message oneof groups
    pub Info: ::std::option::Option<prop_extra_info::Info>,
    // special fields
    // @@protoc_insertion_point(special_field:PropExtraInfo.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a PropExtraInfo {
    fn default() -> &'a PropExtraInfo {
        <PropExtraInfo as ::protobuf::Message>::default_instance()
    }
}

impl PropExtraInfo {
    pub fn new() -> PropExtraInfo {
        ::std::default::Default::default()
    }

    // .FPEKBFOCFBF rogue_info = 13;

    pub fn rogue_info(&self) -> &super::FPEKBFOCFBF::FPEKBFOCFBF {
        match self.Info {
            ::std::option::Option::Some(prop_extra_info::Info::RogueInfo(ref v)) => v,
            _ => <super::FPEKBFOCFBF::FPEKBFOCFBF as ::protobuf::Message>::default_instance(),
        }
    }

    pub fn clear_rogue_info(&mut self) {
        self.Info = ::std::option::Option::None;
    }

    pub fn has_rogue_info(&self) -> bool {
        match self.Info {
            ::std::option::Option::Some(prop_extra_info::Info::RogueInfo(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_rogue_info(&mut self, v: super::FPEKBFOCFBF::FPEKBFOCFBF) {
        self.Info = ::std::option::Option::Some(prop_extra_info::Info::RogueInfo(v))
    }

    // Mutable pointer to the field.
    pub fn mut_rogue_info(&mut self) -> &mut super::FPEKBFOCFBF::FPEKBFOCFBF {
        if let ::std::option::Option::Some(prop_extra_info::Info::RogueInfo(_)) = self.Info {
        } else {
            self.Info = ::std::option::Option::Some(prop_extra_info::Info::RogueInfo(super::FPEKBFOCFBF::FPEKBFOCFBF::new()));
        }
        match self.Info {
            ::std::option::Option::Some(prop_extra_info::Info::RogueInfo(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_rogue_info(&mut self) -> super::FPEKBFOCFBF::FPEKBFOCFBF {
        if self.has_rogue_info() {
            match self.Info.take() {
                ::std::option::Option::Some(prop_extra_info::Info::RogueInfo(v)) => v,
                _ => panic!(),
            }
        } else {
            super::FPEKBFOCFBF::FPEKBFOCFBF::new()
        }
    }

    // .PropAeonInfo aeon_info = 2;

    pub fn aeon_info(&self) -> &super::PropAeonInfo::PropAeonInfo {
        match self.Info {
            ::std::option::Option::Some(prop_extra_info::Info::AeonInfo(ref v)) => v,
            _ => <super::PropAeonInfo::PropAeonInfo as ::protobuf::Message>::default_instance(),
        }
    }

    pub fn clear_aeon_info(&mut self) {
        self.Info = ::std::option::Option::None;
    }

    pub fn has_aeon_info(&self) -> bool {
        match self.Info {
            ::std::option::Option::Some(prop_extra_info::Info::AeonInfo(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_aeon_info(&mut self, v: super::PropAeonInfo::PropAeonInfo) {
        self.Info = ::std::option::Option::Some(prop_extra_info::Info::AeonInfo(v))
    }

    // Mutable pointer to the field.
    pub fn mut_aeon_info(&mut self) -> &mut super::PropAeonInfo::PropAeonInfo {
        if let ::std::option::Option::Some(prop_extra_info::Info::AeonInfo(_)) = self.Info {
        } else {
            self.Info = ::std::option::Option::Some(prop_extra_info::Info::AeonInfo(super::PropAeonInfo::PropAeonInfo::new()));
        }
        match self.Info {
            ::std::option::Option::Some(prop_extra_info::Info::AeonInfo(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_aeon_info(&mut self) -> super::PropAeonInfo::PropAeonInfo {
        if self.has_aeon_info() {
            match self.Info.take() {
                ::std::option::Option::Some(prop_extra_info::Info::AeonInfo(v)) => v,
                _ => panic!(),
            }
        } else {
            super::PropAeonInfo::PropAeonInfo::new()
        }
    }

    // .KCGLJNCALBF chess_rogue_info = 6;

    pub fn chess_rogue_info(&self) -> &super::KCGLJNCALBF::KCGLJNCALBF {
        match self.Info {
            ::std::option::Option::Some(prop_extra_info::Info::ChessRogueInfo(ref v)) => v,
            _ => <super::KCGLJNCALBF::KCGLJNCALBF as ::protobuf::Message>::default_instance(),
        }
    }

    pub fn clear_chess_rogue_info(&mut self) {
        self.Info = ::std::option::Option::None;
    }

    pub fn has_chess_rogue_info(&self) -> bool {
        match self.Info {
            ::std::option::Option::Some(prop_extra_info::Info::ChessRogueInfo(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_chess_rogue_info(&mut self, v: super::KCGLJNCALBF::KCGLJNCALBF) {
        self.Info = ::std::option::Option::Some(prop_extra_info::Info::ChessRogueInfo(v))
    }

    // Mutable pointer to the field.
    pub fn mut_chess_rogue_info(&mut self) -> &mut super::KCGLJNCALBF::KCGLJNCALBF {
        if let ::std::option::Option::Some(prop_extra_info::Info::ChessRogueInfo(_)) = self.Info {
        } else {
            self.Info = ::std::option::Option::Some(prop_extra_info::Info::ChessRogueInfo(super::KCGLJNCALBF::KCGLJNCALBF::new()));
        }
        match self.Info {
            ::std::option::Option::Some(prop_extra_info::Info::ChessRogueInfo(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_chess_rogue_info(&mut self) -> super::KCGLJNCALBF::KCGLJNCALBF {
        if self.has_chess_rogue_info() {
            match self.Info.take() {
                ::std::option::Option::Some(prop_extra_info::Info::ChessRogueInfo(v)) => v,
                _ => panic!(),
            }
        } else {
            super::KCGLJNCALBF::KCGLJNCALBF::new()
        }
    }

    // .KHMPKKHDEAL rogue_tourn_door_info = 7;

    pub fn rogue_tourn_door_info(&self) -> &super::KHMPKKHDEAL::KHMPKKHDEAL {
        match self.Info {
            ::std::option::Option::Some(prop_extra_info::Info::RogueTournDoorInfo(ref v)) => v,
            _ => <super::KHMPKKHDEAL::KHMPKKHDEAL as ::protobuf::Message>::default_instance(),
        }
    }

    pub fn clear_rogue_tourn_door_info(&mut self) {
        self.Info = ::std::option::Option::None;
    }

    pub fn has_rogue_tourn_door_info(&self) -> bool {
        match self.Info {
            ::std::option::Option::Some(prop_extra_info::Info::RogueTournDoorInfo(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_rogue_tourn_door_info(&mut self, v: super::KHMPKKHDEAL::KHMPKKHDEAL) {
        self.Info = ::std::option::Option::Some(prop_extra_info::Info::RogueTournDoorInfo(v))
    }

    // Mutable pointer to the field.
    pub fn mut_rogue_tourn_door_info(&mut self) -> &mut super::KHMPKKHDEAL::KHMPKKHDEAL {
        if let ::std::option::Option::Some(prop_extra_info::Info::RogueTournDoorInfo(_)) = self.Info {
        } else {
            self.Info = ::std::option::Option::Some(prop_extra_info::Info::RogueTournDoorInfo(super::KHMPKKHDEAL::KHMPKKHDEAL::new()));
        }
        match self.Info {
            ::std::option::Option::Some(prop_extra_info::Info::RogueTournDoorInfo(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_rogue_tourn_door_info(&mut self) -> super::KHMPKKHDEAL::KHMPKKHDEAL {
        if self.has_rogue_tourn_door_info() {
            match self.Info.take() {
                ::std::option::Option::Some(prop_extra_info::Info::RogueTournDoorInfo(v)) => v,
                _ => panic!(),
            }
        } else {
            super::KHMPKKHDEAL::KHMPKKHDEAL::new()
        }
    }

    // .ADNACDBEIFL rogue_tourn_workbench_info = 4;

    pub fn rogue_tourn_workbench_info(&self) -> &super::ADNACDBEIFL::ADNACDBEIFL {
        match self.Info {
            ::std::option::Option::Some(prop_extra_info::Info::RogueTournWorkbenchInfo(ref v)) => v,
            _ => <super::ADNACDBEIFL::ADNACDBEIFL as ::protobuf::Message>::default_instance(),
        }
    }

    pub fn clear_rogue_tourn_workbench_info(&mut self) {
        self.Info = ::std::option::Option::None;
    }

    pub fn has_rogue_tourn_workbench_info(&self) -> bool {
        match self.Info {
            ::std::option::Option::Some(prop_extra_info::Info::RogueTournWorkbenchInfo(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_rogue_tourn_workbench_info(&mut self, v: super::ADNACDBEIFL::ADNACDBEIFL) {
        self.Info = ::std::option::Option::Some(prop_extra_info::Info::RogueTournWorkbenchInfo(v))
    }

    // Mutable pointer to the field.
    pub fn mut_rogue_tourn_workbench_info(&mut self) -> &mut super::ADNACDBEIFL::ADNACDBEIFL {
        if let ::std::option::Option::Some(prop_extra_info::Info::RogueTournWorkbenchInfo(_)) = self.Info {
        } else {
            self.Info = ::std::option::Option::Some(prop_extra_info::Info::RogueTournWorkbenchInfo(super::ADNACDBEIFL::ADNACDBEIFL::new()));
        }
        match self.Info {
            ::std::option::Option::Some(prop_extra_info::Info::RogueTournWorkbenchInfo(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_rogue_tourn_workbench_info(&mut self) -> super::ADNACDBEIFL::ADNACDBEIFL {
        if self.has_rogue_tourn_workbench_info() {
            match self.Info.take() {
                ::std::option::Option::Some(prop_extra_info::Info::RogueTournWorkbenchInfo(v)) => v,
                _ => panic!(),
            }
        } else {
            super::ADNACDBEIFL::ADNACDBEIFL::new()
        }
    }

    // .DFECDOAMNLK rogue_gamble_machine_info = 10;

    pub fn rogue_gamble_machine_info(&self) -> &super::DFECDOAMNLK::DFECDOAMNLK {
        match self.Info {
            ::std::option::Option::Some(prop_extra_info::Info::RogueGambleMachineInfo(ref v)) => v,
            _ => <super::DFECDOAMNLK::DFECDOAMNLK as ::protobuf::Message>::default_instance(),
        }
    }

    pub fn clear_rogue_gamble_machine_info(&mut self) {
        self.Info = ::std::option::Option::None;
    }

    pub fn has_rogue_gamble_machine_info(&self) -> bool {
        match self.Info {
            ::std::option::Option::Some(prop_extra_info::Info::RogueGambleMachineInfo(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_rogue_gamble_machine_info(&mut self, v: super::DFECDOAMNLK::DFECDOAMNLK) {
        self.Info = ::std::option::Option::Some(prop_extra_info::Info::RogueGambleMachineInfo(v))
    }

    // Mutable pointer to the field.
    pub fn mut_rogue_gamble_machine_info(&mut self) -> &mut super::DFECDOAMNLK::DFECDOAMNLK {
        if let ::std::option::Option::Some(prop_extra_info::Info::RogueGambleMachineInfo(_)) = self.Info {
        } else {
            self.Info = ::std::option::Option::Some(prop_extra_info::Info::RogueGambleMachineInfo(super::DFECDOAMNLK::DFECDOAMNLK::new()));
        }
        match self.Info {
            ::std::option::Option::Some(prop_extra_info::Info::RogueGambleMachineInfo(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_rogue_gamble_machine_info(&mut self) -> super::DFECDOAMNLK::DFECDOAMNLK {
        if self.has_rogue_gamble_machine_info() {
            match self.Info.take() {
                ::std::option::Option::Some(prop_extra_info::Info::RogueGambleMachineInfo(v)) => v,
                _ => panic!(),
            }
        } else {
            super::DFECDOAMNLK::DFECDOAMNLK::new()
        }
    }

    // .FEJEPOJFHMM rogue_curse_chest_info = 8;

    pub fn rogue_curse_chest_info(&self) -> &super::FEJEPOJFHMM::FEJEPOJFHMM {
        match self.Info {
            ::std::option::Option::Some(prop_extra_info::Info::RogueCurseChestInfo(ref v)) => v,
            _ => <super::FEJEPOJFHMM::FEJEPOJFHMM as ::protobuf::Message>::default_instance(),
        }
    }

    pub fn clear_rogue_curse_chest_info(&mut self) {
        self.Info = ::std::option::Option::None;
    }

    pub fn has_rogue_curse_chest_info(&self) -> bool {
        match self.Info {
            ::std::option::Option::Some(prop_extra_info::Info::RogueCurseChestInfo(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_rogue_curse_chest_info(&mut self, v: super::FEJEPOJFHMM::FEJEPOJFHMM) {
        self.Info = ::std::option::Option::Some(prop_extra_info::Info::RogueCurseChestInfo(v))
    }

    // Mutable pointer to the field.
    pub fn mut_rogue_curse_chest_info(&mut self) -> &mut super::FEJEPOJFHMM::FEJEPOJFHMM {
        if let ::std::option::Option::Some(prop_extra_info::Info::RogueCurseChestInfo(_)) = self.Info {
        } else {
            self.Info = ::std::option::Option::Some(prop_extra_info::Info::RogueCurseChestInfo(super::FEJEPOJFHMM::FEJEPOJFHMM::new()));
        }
        match self.Info {
            ::std::option::Option::Some(prop_extra_info::Info::RogueCurseChestInfo(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_rogue_curse_chest_info(&mut self) -> super::FEJEPOJFHMM::FEJEPOJFHMM {
        if self.has_rogue_curse_chest_info() {
            match self.Info.take() {
                ::std::option::Option::Some(prop_extra_info::Info::RogueCurseChestInfo(v)) => v,
                _ => panic!(),
            }
        } else {
            super::FEJEPOJFHMM::FEJEPOJFHMM::new()
        }
    }

    // .KGDCGLKBDJD rogue_magic_door_info = 3;

    pub fn rogue_magic_door_info(&self) -> &super::KGDCGLKBDJD::KGDCGLKBDJD {
        match self.Info {
            ::std::option::Option::Some(prop_extra_info::Info::RogueMagicDoorInfo(ref v)) => v,
            _ => <super::KGDCGLKBDJD::KGDCGLKBDJD as ::protobuf::Message>::default_instance(),
        }
    }

    pub fn clear_rogue_magic_door_info(&mut self) {
        self.Info = ::std::option::Option::None;
    }

    pub fn has_rogue_magic_door_info(&self) -> bool {
        match self.Info {
            ::std::option::Option::Some(prop_extra_info::Info::RogueMagicDoorInfo(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_rogue_magic_door_info(&mut self, v: super::KGDCGLKBDJD::KGDCGLKBDJD) {
        self.Info = ::std::option::Option::Some(prop_extra_info::Info::RogueMagicDoorInfo(v))
    }

    // Mutable pointer to the field.
    pub fn mut_rogue_magic_door_info(&mut self) -> &mut super::KGDCGLKBDJD::KGDCGLKBDJD {
        if let ::std::option::Option::Some(prop_extra_info::Info::RogueMagicDoorInfo(_)) = self.Info {
        } else {
            self.Info = ::std::option::Option::Some(prop_extra_info::Info::RogueMagicDoorInfo(super::KGDCGLKBDJD::KGDCGLKBDJD::new()));
        }
        match self.Info {
            ::std::option::Option::Some(prop_extra_info::Info::RogueMagicDoorInfo(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_rogue_magic_door_info(&mut self) -> super::KGDCGLKBDJD::KGDCGLKBDJD {
        if self.has_rogue_magic_door_info() {
            match self.Info.take() {
                ::std::option::Option::Some(prop_extra_info::Info::RogueMagicDoorInfo(v)) => v,
                _ => panic!(),
            }
        } else {
            super::KGDCGLKBDJD::KGDCGLKBDJD::new()
        }
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(8);
        let mut oneofs = ::std::vec::Vec::with_capacity(1);
        fields.push(::protobuf::reflect::rt::v2::make_oneof_message_has_get_mut_set_accessor::<_, super::FPEKBFOCFBF::FPEKBFOCFBF>(
            "rogue_info",
            PropExtraInfo::has_rogue_info,
            PropExtraInfo::rogue_info,
            PropExtraInfo::mut_rogue_info,
            PropExtraInfo::set_rogue_info,
        ));
        fields.push(::protobuf::reflect::rt::v2::make_oneof_message_has_get_mut_set_accessor::<_, super::PropAeonInfo::PropAeonInfo>(
            "aeon_info",
            PropExtraInfo::has_aeon_info,
            PropExtraInfo::aeon_info,
            PropExtraInfo::mut_aeon_info,
            PropExtraInfo::set_aeon_info,
        ));
        fields.push(::protobuf::reflect::rt::v2::make_oneof_message_has_get_mut_set_accessor::<_, super::KCGLJNCALBF::KCGLJNCALBF>(
            "chess_rogue_info",
            PropExtraInfo::has_chess_rogue_info,
            PropExtraInfo::chess_rogue_info,
            PropExtraInfo::mut_chess_rogue_info,
            PropExtraInfo::set_chess_rogue_info,
        ));
        fields.push(::protobuf::reflect::rt::v2::make_oneof_message_has_get_mut_set_accessor::<_, super::KHMPKKHDEAL::KHMPKKHDEAL>(
            "rogue_tourn_door_info",
            PropExtraInfo::has_rogue_tourn_door_info,
            PropExtraInfo::rogue_tourn_door_info,
            PropExtraInfo::mut_rogue_tourn_door_info,
            PropExtraInfo::set_rogue_tourn_door_info,
        ));
        fields.push(::protobuf::reflect::rt::v2::make_oneof_message_has_get_mut_set_accessor::<_, super::ADNACDBEIFL::ADNACDBEIFL>(
            "rogue_tourn_workbench_info",
            PropExtraInfo::has_rogue_tourn_workbench_info,
            PropExtraInfo::rogue_tourn_workbench_info,
            PropExtraInfo::mut_rogue_tourn_workbench_info,
            PropExtraInfo::set_rogue_tourn_workbench_info,
        ));
        fields.push(::protobuf::reflect::rt::v2::make_oneof_message_has_get_mut_set_accessor::<_, super::DFECDOAMNLK::DFECDOAMNLK>(
            "rogue_gamble_machine_info",
            PropExtraInfo::has_rogue_gamble_machine_info,
            PropExtraInfo::rogue_gamble_machine_info,
            PropExtraInfo::mut_rogue_gamble_machine_info,
            PropExtraInfo::set_rogue_gamble_machine_info,
        ));
        fields.push(::protobuf::reflect::rt::v2::make_oneof_message_has_get_mut_set_accessor::<_, super::FEJEPOJFHMM::FEJEPOJFHMM>(
            "rogue_curse_chest_info",
            PropExtraInfo::has_rogue_curse_chest_info,
            PropExtraInfo::rogue_curse_chest_info,
            PropExtraInfo::mut_rogue_curse_chest_info,
            PropExtraInfo::set_rogue_curse_chest_info,
        ));
        fields.push(::protobuf::reflect::rt::v2::make_oneof_message_has_get_mut_set_accessor::<_, super::KGDCGLKBDJD::KGDCGLKBDJD>(
            "rogue_magic_door_info",
            PropExtraInfo::has_rogue_magic_door_info,
            PropExtraInfo::rogue_magic_door_info,
            PropExtraInfo::mut_rogue_magic_door_info,
            PropExtraInfo::set_rogue_magic_door_info,
        ));
        oneofs.push(prop_extra_info::Info::generated_oneof_descriptor_data());
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<PropExtraInfo>(
            "PropExtraInfo",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for PropExtraInfo {
    const NAME: &'static str = "PropExtraInfo";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                106 => {
                    self.Info = ::std::option::Option::Some(prop_extra_info::Info::RogueInfo(is.read_message()?));
                },
                18 => {
                    self.Info = ::std::option::Option::Some(prop_extra_info::Info::AeonInfo(is.read_message()?));
                },
                50 => {
                    self.Info = ::std::option::Option::Some(prop_extra_info::Info::ChessRogueInfo(is.read_message()?));
                },
                58 => {
                    self.Info = ::std::option::Option::Some(prop_extra_info::Info::RogueTournDoorInfo(is.read_message()?));
                },
                34 => {
                    self.Info = ::std::option::Option::Some(prop_extra_info::Info::RogueTournWorkbenchInfo(is.read_message()?));
                },
                82 => {
                    self.Info = ::std::option::Option::Some(prop_extra_info::Info::RogueGambleMachineInfo(is.read_message()?));
                },
                66 => {
                    self.Info = ::std::option::Option::Some(prop_extra_info::Info::RogueCurseChestInfo(is.read_message()?));
                },
                26 => {
                    self.Info = ::std::option::Option::Some(prop_extra_info::Info::RogueMagicDoorInfo(is.read_message()?));
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
        if let ::std::option::Option::Some(ref v) = self.Info {
            match v {
                &prop_extra_info::Info::RogueInfo(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
                },
                &prop_extra_info::Info::AeonInfo(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
                },
                &prop_extra_info::Info::ChessRogueInfo(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
                },
                &prop_extra_info::Info::RogueTournDoorInfo(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
                },
                &prop_extra_info::Info::RogueTournWorkbenchInfo(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
                },
                &prop_extra_info::Info::RogueGambleMachineInfo(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
                },
                &prop_extra_info::Info::RogueCurseChestInfo(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
                },
                &prop_extra_info::Info::RogueMagicDoorInfo(ref v) => {
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
        if let ::std::option::Option::Some(ref v) = self.Info {
            match v {
                &prop_extra_info::Info::RogueInfo(ref v) => {
                    ::protobuf::rt::write_message_field_with_cached_size(13, v, os)?;
                },
                &prop_extra_info::Info::AeonInfo(ref v) => {
                    ::protobuf::rt::write_message_field_with_cached_size(2, v, os)?;
                },
                &prop_extra_info::Info::ChessRogueInfo(ref v) => {
                    ::protobuf::rt::write_message_field_with_cached_size(6, v, os)?;
                },
                &prop_extra_info::Info::RogueTournDoorInfo(ref v) => {
                    ::protobuf::rt::write_message_field_with_cached_size(7, v, os)?;
                },
                &prop_extra_info::Info::RogueTournWorkbenchInfo(ref v) => {
                    ::protobuf::rt::write_message_field_with_cached_size(4, v, os)?;
                },
                &prop_extra_info::Info::RogueGambleMachineInfo(ref v) => {
                    ::protobuf::rt::write_message_field_with_cached_size(10, v, os)?;
                },
                &prop_extra_info::Info::RogueCurseChestInfo(ref v) => {
                    ::protobuf::rt::write_message_field_with_cached_size(8, v, os)?;
                },
                &prop_extra_info::Info::RogueMagicDoorInfo(ref v) => {
                    ::protobuf::rt::write_message_field_with_cached_size(3, v, os)?;
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

    fn new() -> PropExtraInfo {
        PropExtraInfo::new()
    }

    fn clear(&mut self) {
        self.Info = ::std::option::Option::None;
        self.Info = ::std::option::Option::None;
        self.Info = ::std::option::Option::None;
        self.Info = ::std::option::Option::None;
        self.Info = ::std::option::Option::None;
        self.Info = ::std::option::Option::None;
        self.Info = ::std::option::Option::None;
        self.Info = ::std::option::Option::None;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static PropExtraInfo {
        static instance: PropExtraInfo = PropExtraInfo {
            Info: ::std::option::Option::None,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for PropExtraInfo {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("PropExtraInfo").unwrap()).clone()
    }
}

impl ::std::fmt::Display for PropExtraInfo {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for PropExtraInfo {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

/// Nested message and enums of message `PropExtraInfo`
pub mod prop_extra_info {

    #[derive(Clone,PartialEq,Debug)]
    #[non_exhaustive]
    // @@protoc_insertion_point(oneof:PropExtraInfo.Info)
    pub enum Info {
        // @@protoc_insertion_point(oneof_field:PropExtraInfo.rogue_info)
        RogueInfo(super::super::FPEKBFOCFBF::FPEKBFOCFBF),
        // @@protoc_insertion_point(oneof_field:PropExtraInfo.aeon_info)
        AeonInfo(super::super::PropAeonInfo::PropAeonInfo),
        // @@protoc_insertion_point(oneof_field:PropExtraInfo.chess_rogue_info)
        ChessRogueInfo(super::super::KCGLJNCALBF::KCGLJNCALBF),
        // @@protoc_insertion_point(oneof_field:PropExtraInfo.rogue_tourn_door_info)
        RogueTournDoorInfo(super::super::KHMPKKHDEAL::KHMPKKHDEAL),
        // @@protoc_insertion_point(oneof_field:PropExtraInfo.rogue_tourn_workbench_info)
        RogueTournWorkbenchInfo(super::super::ADNACDBEIFL::ADNACDBEIFL),
        // @@protoc_insertion_point(oneof_field:PropExtraInfo.rogue_gamble_machine_info)
        RogueGambleMachineInfo(super::super::DFECDOAMNLK::DFECDOAMNLK),
        // @@protoc_insertion_point(oneof_field:PropExtraInfo.rogue_curse_chest_info)
        RogueCurseChestInfo(super::super::FEJEPOJFHMM::FEJEPOJFHMM),
        // @@protoc_insertion_point(oneof_field:PropExtraInfo.rogue_magic_door_info)
        RogueMagicDoorInfo(super::super::KGDCGLKBDJD::KGDCGLKBDJD),
    }

    impl ::protobuf::Oneof for Info {
    }

    impl ::protobuf::OneofFull for Info {
        fn descriptor() -> ::protobuf::reflect::OneofDescriptor {
            static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::OneofDescriptor> = ::protobuf::rt::Lazy::new();
            descriptor.get(|| <super::PropExtraInfo as ::protobuf::MessageFull>::descriptor().oneof_by_name("Info").unwrap()).clone()
        }
    }

    impl Info {
        pub(in super) fn generated_oneof_descriptor_data() -> ::protobuf::reflect::GeneratedOneofDescriptorData {
            ::protobuf::reflect::GeneratedOneofDescriptorData::new::<Info>("Info")
        }
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x13PropExtraInfo.proto\x1a\x11ADNACDBEIFL.proto\x1a\x11DFECDOAMNLK.pr\
    oto\x1a\x11FEJEPOJFHMM.proto\x1a\x11FPEKBFOCFBF.proto\x1a\x11KCGLJNCALBF\
    .proto\x1a\x11KGDCGLKBDJD.proto\x1a\x11KHMPKKHDEAL.proto\x1a\x12PropAeon\
    Info.proto\"\x91\x04\n\rPropExtraInfo\x12-\n\nrogue_info\x18\r\x20\x01(\
    \x0b2\x0c.FPEKBFOCFBFH\0R\trogueInfo\x12,\n\taeon_info\x18\x02\x20\x01(\
    \x0b2\r.PropAeonInfoH\0R\x08aeonInfo\x128\n\x10chess_rogue_info\x18\x06\
    \x20\x01(\x0b2\x0c.KCGLJNCALBFH\0R\x0echessRogueInfo\x12A\n\x15rogue_tou\
    rn_door_info\x18\x07\x20\x01(\x0b2\x0c.KHMPKKHDEALH\0R\x12rogueTournDoor\
    Info\x12K\n\x1arogue_tourn_workbench_info\x18\x04\x20\x01(\x0b2\x0c.ADNA\
    CDBEIFLH\0R\x17rogueTournWorkbenchInfo\x12I\n\x19rogue_gamble_machine_in\
    fo\x18\n\x20\x01(\x0b2\x0c.DFECDOAMNLKH\0R\x16rogueGambleMachineInfo\x12\
    C\n\x16rogue_curse_chest_info\x18\x08\x20\x01(\x0b2\x0c.FEJEPOJFHMMH\0R\
    \x13rogueCurseChestInfo\x12A\n\x15rogue_magic_door_info\x18\x03\x20\x01(\
    \x0b2\x0c.KGDCGLKBDJDH\0R\x12rogueMagicDoorInfoB\x06\n\x04Infob\x06proto\
    3\
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
            let mut deps = ::std::vec::Vec::with_capacity(8);
            deps.push(super::ADNACDBEIFL::file_descriptor().clone());
            deps.push(super::DFECDOAMNLK::file_descriptor().clone());
            deps.push(super::FEJEPOJFHMM::file_descriptor().clone());
            deps.push(super::FPEKBFOCFBF::file_descriptor().clone());
            deps.push(super::KCGLJNCALBF::file_descriptor().clone());
            deps.push(super::KGDCGLKBDJD::file_descriptor().clone());
            deps.push(super::KHMPKKHDEAL::file_descriptor().clone());
            deps.push(super::PropAeonInfo::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(PropExtraInfo::generated_message_descriptor_data());
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
