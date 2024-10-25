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

//! Generated file from `CmdRogueTournType.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

#[derive(Clone,Copy,PartialEq,Eq,Debug,Hash)]
// @@protoc_insertion_point(enum:CmdRogueTournType)
pub enum CmdRogueTournType {
    // @@protoc_insertion_point(enum_value:CmdRogueTournType.CmdRogueTournTypeNone)
    CmdRogueTournTypeNone = 0,
    // @@protoc_insertion_point(enum_value:CmdRogueTournType.CmdRogueTournRenameArchiveCsReq)
    CmdRogueTournRenameArchiveCsReq = 6063,
    // @@protoc_insertion_point(enum_value:CmdRogueTournType.CmdRogueTournGetMiscRealTimeDataCsReq)
    CmdRogueTournGetMiscRealTimeDataCsReq = 6097,
    // @@protoc_insertion_point(enum_value:CmdRogueTournType.CmdRogueTournGetArchiveRepositoryCsReq)
    CmdRogueTournGetArchiveRepositoryCsReq = 6087,
    // @@protoc_insertion_point(enum_value:CmdRogueTournType.CmdRogueTournBattleFailSettleInfoScNotify)
    CmdRogueTournBattleFailSettleInfoScNotify = 6068,
    // @@protoc_insertion_point(enum_value:CmdRogueTournType.CmdRogueTournEnterLayerScRsp)
    CmdRogueTournEnterLayerScRsp = 6062,
    // @@protoc_insertion_point(enum_value:CmdRogueTournType.CmdRogueTournGetPermanentTalentInfoScRsp)
    CmdRogueTournGetPermanentTalentInfoScRsp = 6016,
    // @@protoc_insertion_point(enum_value:CmdRogueTournType.CmdRogueTournEnablePermanentTalentScRsp)
    CmdRogueTournEnablePermanentTalentScRsp = 6026,
    // @@protoc_insertion_point(enum_value:CmdRogueTournType.CmdRogueTournReviveAvatarCsReq)
    CmdRogueTournReviveAvatarCsReq = 6041,
    // @@protoc_insertion_point(enum_value:CmdRogueTournType.CmdRogueTournTakeExpRewardCsReq)
    CmdRogueTournTakeExpRewardCsReq = 6012,
    // @@protoc_insertion_point(enum_value:CmdRogueTournType.CmdRogueTournLeaveCsReq)
    CmdRogueTournLeaveCsReq = 6043,
    // @@protoc_insertion_point(enum_value:CmdRogueTournType.CmdRogueTournSettleScRsp)
    CmdRogueTournSettleScRsp = 6030,
    // @@protoc_insertion_point(enum_value:CmdRogueTournType.CmdRogueTournEnterRoomScRsp)
    CmdRogueTournEnterRoomScRsp = 6067,
    // @@protoc_insertion_point(enum_value:CmdRogueTournType.CmdRogueTournWeekChallengeUpdateScNotify)
    CmdRogueTournWeekChallengeUpdateScNotify = 6013,
    // @@protoc_insertion_point(enum_value:CmdRogueTournType.CmdRogueTournEnterCsReq)
    CmdRogueTournEnterCsReq = 6066,
    // @@protoc_insertion_point(enum_value:CmdRogueTournType.CmdRogueTournQueryCsReq)
    CmdRogueTournQueryCsReq = 6014,
    // @@protoc_insertion_point(enum_value:CmdRogueTournType.CmdRogueTournLevelInfoUpdateScNotify)
    CmdRogueTournLevelInfoUpdateScNotify = 6015,
    // @@protoc_insertion_point(enum_value:CmdRogueTournType.CmdRogueTournDeleteArchiveScRsp)
    CmdRogueTournDeleteArchiveScRsp = 6089,
    // @@protoc_insertion_point(enum_value:CmdRogueTournType.CmdRogueTournEnterRogueCocoonSceneScRsp)
    CmdRogueTournEnterRogueCocoonSceneScRsp = 6025,
    // @@protoc_insertion_point(enum_value:CmdRogueTournType.CmdRogueTournGetPermanentTalentInfoCsReq)
    CmdRogueTournGetPermanentTalentInfoCsReq = 6100,
    // @@protoc_insertion_point(enum_value:CmdRogueTournType.CmdRogueTournEnablePermanentTalentCsReq)
    CmdRogueTournEnablePermanentTalentCsReq = 6048,
    // @@protoc_insertion_point(enum_value:CmdRogueTournType.CmdRogueTournClearArchiveNameScNotify)
    CmdRogueTournClearArchiveNameScNotify = 6011,
    // @@protoc_insertion_point(enum_value:CmdRogueTournType.CmdRogueTournConfirmSettleScRsp)
    CmdRogueTournConfirmSettleScRsp = 6049,
    // @@protoc_insertion_point(enum_value:CmdRogueTournType.CmdRogueTournResetPermanentTalentCsReq)
    CmdRogueTournResetPermanentTalentCsReq = 6084,
    // @@protoc_insertion_point(enum_value:CmdRogueTournType.CmdRogueTournDeleteArchiveCsReq)
    CmdRogueTournDeleteArchiveCsReq = 6060,
    // @@protoc_insertion_point(enum_value:CmdRogueTournType.CmdRogueTournGetSettleInfoScRsp)
    CmdRogueTournGetSettleInfoScRsp = 6065,
    // @@protoc_insertion_point(enum_value:CmdRogueTournType.CmdRogueTournRenameArchiveScRsp)
    CmdRogueTournRenameArchiveScRsp = 6057,
    // @@protoc_insertion_point(enum_value:CmdRogueTournType.CmdRogueTournGetArchiveRepositoryScRsp)
    CmdRogueTournGetArchiveRepositoryScRsp = 6039,
    // @@protoc_insertion_point(enum_value:CmdRogueTournType.CmdRogueTournGetMiscRealTimeDataScRsp)
    CmdRogueTournGetMiscRealTimeDataScRsp = 6042,
    // @@protoc_insertion_point(enum_value:CmdRogueTournType.CmdRogueTournResetPermanentTalentScRsp)
    CmdRogueTournResetPermanentTalentScRsp = 6021,
    // @@protoc_insertion_point(enum_value:CmdRogueTournType.CmdRogueTournGetCurRogueCocoonInfoScRsp)
    CmdRogueTournGetCurRogueCocoonInfoScRsp = 6036,
    // @@protoc_insertion_point(enum_value:CmdRogueTournType.CmdRogueTournLeaveRogueCocoonSceneScRsp)
    CmdRogueTournLeaveRogueCocoonSceneScRsp = 6074,
    // @@protoc_insertion_point(enum_value:CmdRogueTournType.CmdRogueTournConfirmSettleCsReq)
    CmdRogueTournConfirmSettleCsReq = 6073,
    // @@protoc_insertion_point(enum_value:CmdRogueTournType.CmdRogueTournLeaveScRsp)
    CmdRogueTournLeaveScRsp = 6050,
    // @@protoc_insertion_point(enum_value:CmdRogueTournType.CmdRogueTournSettleCsReq)
    CmdRogueTournSettleCsReq = 6094,
    // @@protoc_insertion_point(enum_value:CmdRogueTournType.CmdRogueTournReEnterRogueCocoonStageScRsp)
    CmdRogueTournReEnterRogueCocoonStageScRsp = 6098,
    // @@protoc_insertion_point(enum_value:CmdRogueTournType.CmdRogueTournEnterRogueCocoonSceneCsReq)
    CmdRogueTournEnterRogueCocoonSceneCsReq = 6035,
    // @@protoc_insertion_point(enum_value:CmdRogueTournType.CmdRogueTournStartScRsp)
    CmdRogueTournStartScRsp = 6046,
    // @@protoc_insertion_point(enum_value:CmdRogueTournType.CmdRogueTournGetAllArchiveCsReq)
    CmdRogueTournGetAllArchiveCsReq = 6032,
    // @@protoc_insertion_point(enum_value:CmdRogueTournType.CmdRogueTournQueryScRsp)
    CmdRogueTournQueryScRsp = 6033,
    // @@protoc_insertion_point(enum_value:CmdRogueTournType.CmdRogueTournDifficultyCompNotify)
    CmdRogueTournDifficultyCompNotify = 6082,
    // @@protoc_insertion_point(enum_value:CmdRogueTournType.CmdRogueTournTakeExpRewardScRsp)
    CmdRogueTournTakeExpRewardScRsp = 6052,
    // @@protoc_insertion_point(enum_value:CmdRogueTournType.CmdRogueTournGetCurRogueCocoonInfoCsReq)
    CmdRogueTournGetCurRogueCocoonInfoCsReq = 6055,
    // @@protoc_insertion_point(enum_value:CmdRogueTournType.CmdRogueTournGetAllArchiveScRsp)
    CmdRogueTournGetAllArchiveScRsp = 6081,
    // @@protoc_insertion_point(enum_value:CmdRogueTournType.CmdRogueTournReviveAvatarScRsp)
    CmdRogueTournReviveAvatarScRsp = 6037,
    // @@protoc_insertion_point(enum_value:CmdRogueTournType.CmdRogueTournAreaUpdateScNotify)
    CmdRogueTournAreaUpdateScNotify = 6020,
    // @@protoc_insertion_point(enum_value:CmdRogueTournType.CmdRogueTournExpNotify)
    CmdRogueTournExpNotify = 6088,
    // @@protoc_insertion_point(enum_value:CmdRogueTournType.CmdRogueTournHandBookNotify)
    CmdRogueTournHandBookNotify = 6070,
    // @@protoc_insertion_point(enum_value:CmdRogueTournType.CmdRogueTournGetSettleInfoCsReq)
    CmdRogueTournGetSettleInfoCsReq = 6075,
    // @@protoc_insertion_point(enum_value:CmdRogueTournType.CmdRogueTournLeaveRogueCocoonSceneCsReq)
    CmdRogueTournLeaveRogueCocoonSceneCsReq = 6029,
    // @@protoc_insertion_point(enum_value:CmdRogueTournType.CmdRogueTournReviveCostUpdateScNotify)
    CmdRogueTournReviveCostUpdateScNotify = 6077,
    // @@protoc_insertion_point(enum_value:CmdRogueTournType.CmdRogueTournEnterScRsp)
    CmdRogueTournEnterScRsp = 6059,
    // @@protoc_insertion_point(enum_value:CmdRogueTournType.CmdRogueTournEnterRoomCsReq)
    CmdRogueTournEnterRoomCsReq = 6079,
    // @@protoc_insertion_point(enum_value:CmdRogueTournType.CmdRogueTournStartCsReq)
    CmdRogueTournStartCsReq = 6072,
    // @@protoc_insertion_point(enum_value:CmdRogueTournType.CmdRogueTournEnterLayerCsReq)
    CmdRogueTournEnterLayerCsReq = 6038,
    // @@protoc_insertion_point(enum_value:CmdRogueTournType.CmdRogueTournReEnterRogueCocoonStageCsReq)
    CmdRogueTournReEnterRogueCocoonStageCsReq = 6092,
}

impl ::protobuf::Enum for CmdRogueTournType {
    const NAME: &'static str = "CmdRogueTournType";

    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<CmdRogueTournType> {
        match value {
            0 => ::std::option::Option::Some(CmdRogueTournType::CmdRogueTournTypeNone),
            6063 => ::std::option::Option::Some(CmdRogueTournType::CmdRogueTournRenameArchiveCsReq),
            6097 => ::std::option::Option::Some(CmdRogueTournType::CmdRogueTournGetMiscRealTimeDataCsReq),
            6087 => ::std::option::Option::Some(CmdRogueTournType::CmdRogueTournGetArchiveRepositoryCsReq),
            6068 => ::std::option::Option::Some(CmdRogueTournType::CmdRogueTournBattleFailSettleInfoScNotify),
            6062 => ::std::option::Option::Some(CmdRogueTournType::CmdRogueTournEnterLayerScRsp),
            6016 => ::std::option::Option::Some(CmdRogueTournType::CmdRogueTournGetPermanentTalentInfoScRsp),
            6026 => ::std::option::Option::Some(CmdRogueTournType::CmdRogueTournEnablePermanentTalentScRsp),
            6041 => ::std::option::Option::Some(CmdRogueTournType::CmdRogueTournReviveAvatarCsReq),
            6012 => ::std::option::Option::Some(CmdRogueTournType::CmdRogueTournTakeExpRewardCsReq),
            6043 => ::std::option::Option::Some(CmdRogueTournType::CmdRogueTournLeaveCsReq),
            6030 => ::std::option::Option::Some(CmdRogueTournType::CmdRogueTournSettleScRsp),
            6067 => ::std::option::Option::Some(CmdRogueTournType::CmdRogueTournEnterRoomScRsp),
            6013 => ::std::option::Option::Some(CmdRogueTournType::CmdRogueTournWeekChallengeUpdateScNotify),
            6066 => ::std::option::Option::Some(CmdRogueTournType::CmdRogueTournEnterCsReq),
            6014 => ::std::option::Option::Some(CmdRogueTournType::CmdRogueTournQueryCsReq),
            6015 => ::std::option::Option::Some(CmdRogueTournType::CmdRogueTournLevelInfoUpdateScNotify),
            6089 => ::std::option::Option::Some(CmdRogueTournType::CmdRogueTournDeleteArchiveScRsp),
            6025 => ::std::option::Option::Some(CmdRogueTournType::CmdRogueTournEnterRogueCocoonSceneScRsp),
            6100 => ::std::option::Option::Some(CmdRogueTournType::CmdRogueTournGetPermanentTalentInfoCsReq),
            6048 => ::std::option::Option::Some(CmdRogueTournType::CmdRogueTournEnablePermanentTalentCsReq),
            6011 => ::std::option::Option::Some(CmdRogueTournType::CmdRogueTournClearArchiveNameScNotify),
            6049 => ::std::option::Option::Some(CmdRogueTournType::CmdRogueTournConfirmSettleScRsp),
            6084 => ::std::option::Option::Some(CmdRogueTournType::CmdRogueTournResetPermanentTalentCsReq),
            6060 => ::std::option::Option::Some(CmdRogueTournType::CmdRogueTournDeleteArchiveCsReq),
            6065 => ::std::option::Option::Some(CmdRogueTournType::CmdRogueTournGetSettleInfoScRsp),
            6057 => ::std::option::Option::Some(CmdRogueTournType::CmdRogueTournRenameArchiveScRsp),
            6039 => ::std::option::Option::Some(CmdRogueTournType::CmdRogueTournGetArchiveRepositoryScRsp),
            6042 => ::std::option::Option::Some(CmdRogueTournType::CmdRogueTournGetMiscRealTimeDataScRsp),
            6021 => ::std::option::Option::Some(CmdRogueTournType::CmdRogueTournResetPermanentTalentScRsp),
            6036 => ::std::option::Option::Some(CmdRogueTournType::CmdRogueTournGetCurRogueCocoonInfoScRsp),
            6074 => ::std::option::Option::Some(CmdRogueTournType::CmdRogueTournLeaveRogueCocoonSceneScRsp),
            6073 => ::std::option::Option::Some(CmdRogueTournType::CmdRogueTournConfirmSettleCsReq),
            6050 => ::std::option::Option::Some(CmdRogueTournType::CmdRogueTournLeaveScRsp),
            6094 => ::std::option::Option::Some(CmdRogueTournType::CmdRogueTournSettleCsReq),
            6098 => ::std::option::Option::Some(CmdRogueTournType::CmdRogueTournReEnterRogueCocoonStageScRsp),
            6035 => ::std::option::Option::Some(CmdRogueTournType::CmdRogueTournEnterRogueCocoonSceneCsReq),
            6046 => ::std::option::Option::Some(CmdRogueTournType::CmdRogueTournStartScRsp),
            6032 => ::std::option::Option::Some(CmdRogueTournType::CmdRogueTournGetAllArchiveCsReq),
            6033 => ::std::option::Option::Some(CmdRogueTournType::CmdRogueTournQueryScRsp),
            6082 => ::std::option::Option::Some(CmdRogueTournType::CmdRogueTournDifficultyCompNotify),
            6052 => ::std::option::Option::Some(CmdRogueTournType::CmdRogueTournTakeExpRewardScRsp),
            6055 => ::std::option::Option::Some(CmdRogueTournType::CmdRogueTournGetCurRogueCocoonInfoCsReq),
            6081 => ::std::option::Option::Some(CmdRogueTournType::CmdRogueTournGetAllArchiveScRsp),
            6037 => ::std::option::Option::Some(CmdRogueTournType::CmdRogueTournReviveAvatarScRsp),
            6020 => ::std::option::Option::Some(CmdRogueTournType::CmdRogueTournAreaUpdateScNotify),
            6088 => ::std::option::Option::Some(CmdRogueTournType::CmdRogueTournExpNotify),
            6070 => ::std::option::Option::Some(CmdRogueTournType::CmdRogueTournHandBookNotify),
            6075 => ::std::option::Option::Some(CmdRogueTournType::CmdRogueTournGetSettleInfoCsReq),
            6029 => ::std::option::Option::Some(CmdRogueTournType::CmdRogueTournLeaveRogueCocoonSceneCsReq),
            6077 => ::std::option::Option::Some(CmdRogueTournType::CmdRogueTournReviveCostUpdateScNotify),
            6059 => ::std::option::Option::Some(CmdRogueTournType::CmdRogueTournEnterScRsp),
            6079 => ::std::option::Option::Some(CmdRogueTournType::CmdRogueTournEnterRoomCsReq),
            6072 => ::std::option::Option::Some(CmdRogueTournType::CmdRogueTournStartCsReq),
            6038 => ::std::option::Option::Some(CmdRogueTournType::CmdRogueTournEnterLayerCsReq),
            6092 => ::std::option::Option::Some(CmdRogueTournType::CmdRogueTournReEnterRogueCocoonStageCsReq),
            _ => ::std::option::Option::None
        }
    }

    fn from_str(str: &str) -> ::std::option::Option<CmdRogueTournType> {
        match str {
            "CmdRogueTournTypeNone" => ::std::option::Option::Some(CmdRogueTournType::CmdRogueTournTypeNone),
            "CmdRogueTournRenameArchiveCsReq" => ::std::option::Option::Some(CmdRogueTournType::CmdRogueTournRenameArchiveCsReq),
            "CmdRogueTournGetMiscRealTimeDataCsReq" => ::std::option::Option::Some(CmdRogueTournType::CmdRogueTournGetMiscRealTimeDataCsReq),
            "CmdRogueTournGetArchiveRepositoryCsReq" => ::std::option::Option::Some(CmdRogueTournType::CmdRogueTournGetArchiveRepositoryCsReq),
            "CmdRogueTournBattleFailSettleInfoScNotify" => ::std::option::Option::Some(CmdRogueTournType::CmdRogueTournBattleFailSettleInfoScNotify),
            "CmdRogueTournEnterLayerScRsp" => ::std::option::Option::Some(CmdRogueTournType::CmdRogueTournEnterLayerScRsp),
            "CmdRogueTournGetPermanentTalentInfoScRsp" => ::std::option::Option::Some(CmdRogueTournType::CmdRogueTournGetPermanentTalentInfoScRsp),
            "CmdRogueTournEnablePermanentTalentScRsp" => ::std::option::Option::Some(CmdRogueTournType::CmdRogueTournEnablePermanentTalentScRsp),
            "CmdRogueTournReviveAvatarCsReq" => ::std::option::Option::Some(CmdRogueTournType::CmdRogueTournReviveAvatarCsReq),
            "CmdRogueTournTakeExpRewardCsReq" => ::std::option::Option::Some(CmdRogueTournType::CmdRogueTournTakeExpRewardCsReq),
            "CmdRogueTournLeaveCsReq" => ::std::option::Option::Some(CmdRogueTournType::CmdRogueTournLeaveCsReq),
            "CmdRogueTournSettleScRsp" => ::std::option::Option::Some(CmdRogueTournType::CmdRogueTournSettleScRsp),
            "CmdRogueTournEnterRoomScRsp" => ::std::option::Option::Some(CmdRogueTournType::CmdRogueTournEnterRoomScRsp),
            "CmdRogueTournWeekChallengeUpdateScNotify" => ::std::option::Option::Some(CmdRogueTournType::CmdRogueTournWeekChallengeUpdateScNotify),
            "CmdRogueTournEnterCsReq" => ::std::option::Option::Some(CmdRogueTournType::CmdRogueTournEnterCsReq),
            "CmdRogueTournQueryCsReq" => ::std::option::Option::Some(CmdRogueTournType::CmdRogueTournQueryCsReq),
            "CmdRogueTournLevelInfoUpdateScNotify" => ::std::option::Option::Some(CmdRogueTournType::CmdRogueTournLevelInfoUpdateScNotify),
            "CmdRogueTournDeleteArchiveScRsp" => ::std::option::Option::Some(CmdRogueTournType::CmdRogueTournDeleteArchiveScRsp),
            "CmdRogueTournEnterRogueCocoonSceneScRsp" => ::std::option::Option::Some(CmdRogueTournType::CmdRogueTournEnterRogueCocoonSceneScRsp),
            "CmdRogueTournGetPermanentTalentInfoCsReq" => ::std::option::Option::Some(CmdRogueTournType::CmdRogueTournGetPermanentTalentInfoCsReq),
            "CmdRogueTournEnablePermanentTalentCsReq" => ::std::option::Option::Some(CmdRogueTournType::CmdRogueTournEnablePermanentTalentCsReq),
            "CmdRogueTournClearArchiveNameScNotify" => ::std::option::Option::Some(CmdRogueTournType::CmdRogueTournClearArchiveNameScNotify),
            "CmdRogueTournConfirmSettleScRsp" => ::std::option::Option::Some(CmdRogueTournType::CmdRogueTournConfirmSettleScRsp),
            "CmdRogueTournResetPermanentTalentCsReq" => ::std::option::Option::Some(CmdRogueTournType::CmdRogueTournResetPermanentTalentCsReq),
            "CmdRogueTournDeleteArchiveCsReq" => ::std::option::Option::Some(CmdRogueTournType::CmdRogueTournDeleteArchiveCsReq),
            "CmdRogueTournGetSettleInfoScRsp" => ::std::option::Option::Some(CmdRogueTournType::CmdRogueTournGetSettleInfoScRsp),
            "CmdRogueTournRenameArchiveScRsp" => ::std::option::Option::Some(CmdRogueTournType::CmdRogueTournRenameArchiveScRsp),
            "CmdRogueTournGetArchiveRepositoryScRsp" => ::std::option::Option::Some(CmdRogueTournType::CmdRogueTournGetArchiveRepositoryScRsp),
            "CmdRogueTournGetMiscRealTimeDataScRsp" => ::std::option::Option::Some(CmdRogueTournType::CmdRogueTournGetMiscRealTimeDataScRsp),
            "CmdRogueTournResetPermanentTalentScRsp" => ::std::option::Option::Some(CmdRogueTournType::CmdRogueTournResetPermanentTalentScRsp),
            "CmdRogueTournGetCurRogueCocoonInfoScRsp" => ::std::option::Option::Some(CmdRogueTournType::CmdRogueTournGetCurRogueCocoonInfoScRsp),
            "CmdRogueTournLeaveRogueCocoonSceneScRsp" => ::std::option::Option::Some(CmdRogueTournType::CmdRogueTournLeaveRogueCocoonSceneScRsp),
            "CmdRogueTournConfirmSettleCsReq" => ::std::option::Option::Some(CmdRogueTournType::CmdRogueTournConfirmSettleCsReq),
            "CmdRogueTournLeaveScRsp" => ::std::option::Option::Some(CmdRogueTournType::CmdRogueTournLeaveScRsp),
            "CmdRogueTournSettleCsReq" => ::std::option::Option::Some(CmdRogueTournType::CmdRogueTournSettleCsReq),
            "CmdRogueTournReEnterRogueCocoonStageScRsp" => ::std::option::Option::Some(CmdRogueTournType::CmdRogueTournReEnterRogueCocoonStageScRsp),
            "CmdRogueTournEnterRogueCocoonSceneCsReq" => ::std::option::Option::Some(CmdRogueTournType::CmdRogueTournEnterRogueCocoonSceneCsReq),
            "CmdRogueTournStartScRsp" => ::std::option::Option::Some(CmdRogueTournType::CmdRogueTournStartScRsp),
            "CmdRogueTournGetAllArchiveCsReq" => ::std::option::Option::Some(CmdRogueTournType::CmdRogueTournGetAllArchiveCsReq),
            "CmdRogueTournQueryScRsp" => ::std::option::Option::Some(CmdRogueTournType::CmdRogueTournQueryScRsp),
            "CmdRogueTournDifficultyCompNotify" => ::std::option::Option::Some(CmdRogueTournType::CmdRogueTournDifficultyCompNotify),
            "CmdRogueTournTakeExpRewardScRsp" => ::std::option::Option::Some(CmdRogueTournType::CmdRogueTournTakeExpRewardScRsp),
            "CmdRogueTournGetCurRogueCocoonInfoCsReq" => ::std::option::Option::Some(CmdRogueTournType::CmdRogueTournGetCurRogueCocoonInfoCsReq),
            "CmdRogueTournGetAllArchiveScRsp" => ::std::option::Option::Some(CmdRogueTournType::CmdRogueTournGetAllArchiveScRsp),
            "CmdRogueTournReviveAvatarScRsp" => ::std::option::Option::Some(CmdRogueTournType::CmdRogueTournReviveAvatarScRsp),
            "CmdRogueTournAreaUpdateScNotify" => ::std::option::Option::Some(CmdRogueTournType::CmdRogueTournAreaUpdateScNotify),
            "CmdRogueTournExpNotify" => ::std::option::Option::Some(CmdRogueTournType::CmdRogueTournExpNotify),
            "CmdRogueTournHandBookNotify" => ::std::option::Option::Some(CmdRogueTournType::CmdRogueTournHandBookNotify),
            "CmdRogueTournGetSettleInfoCsReq" => ::std::option::Option::Some(CmdRogueTournType::CmdRogueTournGetSettleInfoCsReq),
            "CmdRogueTournLeaveRogueCocoonSceneCsReq" => ::std::option::Option::Some(CmdRogueTournType::CmdRogueTournLeaveRogueCocoonSceneCsReq),
            "CmdRogueTournReviveCostUpdateScNotify" => ::std::option::Option::Some(CmdRogueTournType::CmdRogueTournReviveCostUpdateScNotify),
            "CmdRogueTournEnterScRsp" => ::std::option::Option::Some(CmdRogueTournType::CmdRogueTournEnterScRsp),
            "CmdRogueTournEnterRoomCsReq" => ::std::option::Option::Some(CmdRogueTournType::CmdRogueTournEnterRoomCsReq),
            "CmdRogueTournStartCsReq" => ::std::option::Option::Some(CmdRogueTournType::CmdRogueTournStartCsReq),
            "CmdRogueTournEnterLayerCsReq" => ::std::option::Option::Some(CmdRogueTournType::CmdRogueTournEnterLayerCsReq),
            "CmdRogueTournReEnterRogueCocoonStageCsReq" => ::std::option::Option::Some(CmdRogueTournType::CmdRogueTournReEnterRogueCocoonStageCsReq),
            _ => ::std::option::Option::None
        }
    }

    const VALUES: &'static [CmdRogueTournType] = &[
        CmdRogueTournType::CmdRogueTournTypeNone,
        CmdRogueTournType::CmdRogueTournRenameArchiveCsReq,
        CmdRogueTournType::CmdRogueTournGetMiscRealTimeDataCsReq,
        CmdRogueTournType::CmdRogueTournGetArchiveRepositoryCsReq,
        CmdRogueTournType::CmdRogueTournBattleFailSettleInfoScNotify,
        CmdRogueTournType::CmdRogueTournEnterLayerScRsp,
        CmdRogueTournType::CmdRogueTournGetPermanentTalentInfoScRsp,
        CmdRogueTournType::CmdRogueTournEnablePermanentTalentScRsp,
        CmdRogueTournType::CmdRogueTournReviveAvatarCsReq,
        CmdRogueTournType::CmdRogueTournTakeExpRewardCsReq,
        CmdRogueTournType::CmdRogueTournLeaveCsReq,
        CmdRogueTournType::CmdRogueTournSettleScRsp,
        CmdRogueTournType::CmdRogueTournEnterRoomScRsp,
        CmdRogueTournType::CmdRogueTournWeekChallengeUpdateScNotify,
        CmdRogueTournType::CmdRogueTournEnterCsReq,
        CmdRogueTournType::CmdRogueTournQueryCsReq,
        CmdRogueTournType::CmdRogueTournLevelInfoUpdateScNotify,
        CmdRogueTournType::CmdRogueTournDeleteArchiveScRsp,
        CmdRogueTournType::CmdRogueTournEnterRogueCocoonSceneScRsp,
        CmdRogueTournType::CmdRogueTournGetPermanentTalentInfoCsReq,
        CmdRogueTournType::CmdRogueTournEnablePermanentTalentCsReq,
        CmdRogueTournType::CmdRogueTournClearArchiveNameScNotify,
        CmdRogueTournType::CmdRogueTournConfirmSettleScRsp,
        CmdRogueTournType::CmdRogueTournResetPermanentTalentCsReq,
        CmdRogueTournType::CmdRogueTournDeleteArchiveCsReq,
        CmdRogueTournType::CmdRogueTournGetSettleInfoScRsp,
        CmdRogueTournType::CmdRogueTournRenameArchiveScRsp,
        CmdRogueTournType::CmdRogueTournGetArchiveRepositoryScRsp,
        CmdRogueTournType::CmdRogueTournGetMiscRealTimeDataScRsp,
        CmdRogueTournType::CmdRogueTournResetPermanentTalentScRsp,
        CmdRogueTournType::CmdRogueTournGetCurRogueCocoonInfoScRsp,
        CmdRogueTournType::CmdRogueTournLeaveRogueCocoonSceneScRsp,
        CmdRogueTournType::CmdRogueTournConfirmSettleCsReq,
        CmdRogueTournType::CmdRogueTournLeaveScRsp,
        CmdRogueTournType::CmdRogueTournSettleCsReq,
        CmdRogueTournType::CmdRogueTournReEnterRogueCocoonStageScRsp,
        CmdRogueTournType::CmdRogueTournEnterRogueCocoonSceneCsReq,
        CmdRogueTournType::CmdRogueTournStartScRsp,
        CmdRogueTournType::CmdRogueTournGetAllArchiveCsReq,
        CmdRogueTournType::CmdRogueTournQueryScRsp,
        CmdRogueTournType::CmdRogueTournDifficultyCompNotify,
        CmdRogueTournType::CmdRogueTournTakeExpRewardScRsp,
        CmdRogueTournType::CmdRogueTournGetCurRogueCocoonInfoCsReq,
        CmdRogueTournType::CmdRogueTournGetAllArchiveScRsp,
        CmdRogueTournType::CmdRogueTournReviveAvatarScRsp,
        CmdRogueTournType::CmdRogueTournAreaUpdateScNotify,
        CmdRogueTournType::CmdRogueTournExpNotify,
        CmdRogueTournType::CmdRogueTournHandBookNotify,
        CmdRogueTournType::CmdRogueTournGetSettleInfoCsReq,
        CmdRogueTournType::CmdRogueTournLeaveRogueCocoonSceneCsReq,
        CmdRogueTournType::CmdRogueTournReviveCostUpdateScNotify,
        CmdRogueTournType::CmdRogueTournEnterScRsp,
        CmdRogueTournType::CmdRogueTournEnterRoomCsReq,
        CmdRogueTournType::CmdRogueTournStartCsReq,
        CmdRogueTournType::CmdRogueTournEnterLayerCsReq,
        CmdRogueTournType::CmdRogueTournReEnterRogueCocoonStageCsReq,
    ];
}

impl ::protobuf::EnumFull for CmdRogueTournType {
    fn enum_descriptor() -> ::protobuf::reflect::EnumDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().enum_by_package_relative_name("CmdRogueTournType").unwrap()).clone()
    }

    fn descriptor(&self) -> ::protobuf::reflect::EnumValueDescriptor {
        let index = match self {
            CmdRogueTournType::CmdRogueTournTypeNone => 0,
            CmdRogueTournType::CmdRogueTournRenameArchiveCsReq => 1,
            CmdRogueTournType::CmdRogueTournGetMiscRealTimeDataCsReq => 2,
            CmdRogueTournType::CmdRogueTournGetArchiveRepositoryCsReq => 3,
            CmdRogueTournType::CmdRogueTournBattleFailSettleInfoScNotify => 4,
            CmdRogueTournType::CmdRogueTournEnterLayerScRsp => 5,
            CmdRogueTournType::CmdRogueTournGetPermanentTalentInfoScRsp => 6,
            CmdRogueTournType::CmdRogueTournEnablePermanentTalentScRsp => 7,
            CmdRogueTournType::CmdRogueTournReviveAvatarCsReq => 8,
            CmdRogueTournType::CmdRogueTournTakeExpRewardCsReq => 9,
            CmdRogueTournType::CmdRogueTournLeaveCsReq => 10,
            CmdRogueTournType::CmdRogueTournSettleScRsp => 11,
            CmdRogueTournType::CmdRogueTournEnterRoomScRsp => 12,
            CmdRogueTournType::CmdRogueTournWeekChallengeUpdateScNotify => 13,
            CmdRogueTournType::CmdRogueTournEnterCsReq => 14,
            CmdRogueTournType::CmdRogueTournQueryCsReq => 15,
            CmdRogueTournType::CmdRogueTournLevelInfoUpdateScNotify => 16,
            CmdRogueTournType::CmdRogueTournDeleteArchiveScRsp => 17,
            CmdRogueTournType::CmdRogueTournEnterRogueCocoonSceneScRsp => 18,
            CmdRogueTournType::CmdRogueTournGetPermanentTalentInfoCsReq => 19,
            CmdRogueTournType::CmdRogueTournEnablePermanentTalentCsReq => 20,
            CmdRogueTournType::CmdRogueTournClearArchiveNameScNotify => 21,
            CmdRogueTournType::CmdRogueTournConfirmSettleScRsp => 22,
            CmdRogueTournType::CmdRogueTournResetPermanentTalentCsReq => 23,
            CmdRogueTournType::CmdRogueTournDeleteArchiveCsReq => 24,
            CmdRogueTournType::CmdRogueTournGetSettleInfoScRsp => 25,
            CmdRogueTournType::CmdRogueTournRenameArchiveScRsp => 26,
            CmdRogueTournType::CmdRogueTournGetArchiveRepositoryScRsp => 27,
            CmdRogueTournType::CmdRogueTournGetMiscRealTimeDataScRsp => 28,
            CmdRogueTournType::CmdRogueTournResetPermanentTalentScRsp => 29,
            CmdRogueTournType::CmdRogueTournGetCurRogueCocoonInfoScRsp => 30,
            CmdRogueTournType::CmdRogueTournLeaveRogueCocoonSceneScRsp => 31,
            CmdRogueTournType::CmdRogueTournConfirmSettleCsReq => 32,
            CmdRogueTournType::CmdRogueTournLeaveScRsp => 33,
            CmdRogueTournType::CmdRogueTournSettleCsReq => 34,
            CmdRogueTournType::CmdRogueTournReEnterRogueCocoonStageScRsp => 35,
            CmdRogueTournType::CmdRogueTournEnterRogueCocoonSceneCsReq => 36,
            CmdRogueTournType::CmdRogueTournStartScRsp => 37,
            CmdRogueTournType::CmdRogueTournGetAllArchiveCsReq => 38,
            CmdRogueTournType::CmdRogueTournQueryScRsp => 39,
            CmdRogueTournType::CmdRogueTournDifficultyCompNotify => 40,
            CmdRogueTournType::CmdRogueTournTakeExpRewardScRsp => 41,
            CmdRogueTournType::CmdRogueTournGetCurRogueCocoonInfoCsReq => 42,
            CmdRogueTournType::CmdRogueTournGetAllArchiveScRsp => 43,
            CmdRogueTournType::CmdRogueTournReviveAvatarScRsp => 44,
            CmdRogueTournType::CmdRogueTournAreaUpdateScNotify => 45,
            CmdRogueTournType::CmdRogueTournExpNotify => 46,
            CmdRogueTournType::CmdRogueTournHandBookNotify => 47,
            CmdRogueTournType::CmdRogueTournGetSettleInfoCsReq => 48,
            CmdRogueTournType::CmdRogueTournLeaveRogueCocoonSceneCsReq => 49,
            CmdRogueTournType::CmdRogueTournReviveCostUpdateScNotify => 50,
            CmdRogueTournType::CmdRogueTournEnterScRsp => 51,
            CmdRogueTournType::CmdRogueTournEnterRoomCsReq => 52,
            CmdRogueTournType::CmdRogueTournStartCsReq => 53,
            CmdRogueTournType::CmdRogueTournEnterLayerCsReq => 54,
            CmdRogueTournType::CmdRogueTournReEnterRogueCocoonStageCsReq => 55,
        };
        Self::enum_descriptor().value_by_index(index)
    }
}

impl ::std::default::Default for CmdRogueTournType {
    fn default() -> Self {
        CmdRogueTournType::CmdRogueTournTypeNone
    }
}

impl CmdRogueTournType {
    fn generated_enum_descriptor_data() -> ::protobuf::reflect::GeneratedEnumDescriptorData {
        ::protobuf::reflect::GeneratedEnumDescriptorData::new::<CmdRogueTournType>("CmdRogueTournType")
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x17CmdRogueTournType.proto*\xa1\x11\n\x11CmdRogueTournType\x12\x19\n\
    \x15CmdRogueTournTypeNone\x10\0\x12$\n\x1fCmdRogueTournRenameArchiveCsRe\
    q\x10\xaf/\x12*\n%CmdRogueTournGetMiscRealTimeDataCsReq\x10\xd1/\x12+\n&\
    CmdRogueTournGetArchiveRepositoryCsReq\x10\xc7/\x12.\n)CmdRogueTournBatt\
    leFailSettleInfoScNotify\x10\xb4/\x12!\n\x1cCmdRogueTournEnterLayerScRsp\
    \x10\xae/\x12-\n(CmdRogueTournGetPermanentTalentInfoScRsp\x10\x80/\x12,\
    \n'CmdRogueTournEnablePermanentTalentScRsp\x10\x8a/\x12#\n\x1eCmdRogueTo\
    urnReviveAvatarCsReq\x10\x99/\x12$\n\x1fCmdRogueTournTakeExpRewardCsReq\
    \x10\xfc.\x12\x1c\n\x17CmdRogueTournLeaveCsReq\x10\x9b/\x12\x1d\n\x18Cmd\
    RogueTournSettleScRsp\x10\x8e/\x12\x20\n\x1bCmdRogueTournEnterRoomScRsp\
    \x10\xb3/\x12-\n(CmdRogueTournWeekChallengeUpdateScNotify\x10\xfd.\x12\
    \x1c\n\x17CmdRogueTournEnterCsReq\x10\xb2/\x12\x1c\n\x17CmdRogueTournQue\
    ryCsReq\x10\xfe.\x12)\n$CmdRogueTournLevelInfoUpdateScNotify\x10\xff.\
    \x12$\n\x1fCmdRogueTournDeleteArchiveScRsp\x10\xc9/\x12,\n'CmdRogueTourn\
    EnterRogueCocoonSceneScRsp\x10\x89/\x12-\n(CmdRogueTournGetPermanentTale\
    ntInfoCsReq\x10\xd4/\x12,\n'CmdRogueTournEnablePermanentTalentCsReq\x10\
    \xa0/\x12*\n%CmdRogueTournClearArchiveNameScNotify\x10\xfb.\x12$\n\x1fCm\
    dRogueTournConfirmSettleScRsp\x10\xa1/\x12+\n&CmdRogueTournResetPermanen\
    tTalentCsReq\x10\xc4/\x12$\n\x1fCmdRogueTournDeleteArchiveCsReq\x10\xac/\
    \x12$\n\x1fCmdRogueTournGetSettleInfoScRsp\x10\xb1/\x12$\n\x1fCmdRogueTo\
    urnRenameArchiveScRsp\x10\xa9/\x12+\n&CmdRogueTournGetArchiveRepositoryS\
    cRsp\x10\x97/\x12*\n%CmdRogueTournGetMiscRealTimeDataScRsp\x10\x9a/\x12+\
    \n&CmdRogueTournResetPermanentTalentScRsp\x10\x85/\x12,\n'CmdRogueTournG\
    etCurRogueCocoonInfoScRsp\x10\x94/\x12,\n'CmdRogueTournLeaveRogueCocoonS\
    ceneScRsp\x10\xba/\x12$\n\x1fCmdRogueTournConfirmSettleCsReq\x10\xb9/\
    \x12\x1c\n\x17CmdRogueTournLeaveScRsp\x10\xa2/\x12\x1d\n\x18CmdRogueTour\
    nSettleCsReq\x10\xce/\x12.\n)CmdRogueTournReEnterRogueCocoonStageScRsp\
    \x10\xd2/\x12,\n'CmdRogueTournEnterRogueCocoonSceneCsReq\x10\x93/\x12\
    \x1c\n\x17CmdRogueTournStartScRsp\x10\x9e/\x12$\n\x1fCmdRogueTournGetAll\
    ArchiveCsReq\x10\x90/\x12\x1c\n\x17CmdRogueTournQueryScRsp\x10\x91/\x12&\
    \n!CmdRogueTournDifficultyCompNotify\x10\xc2/\x12$\n\x1fCmdRogueTournTak\
    eExpRewardScRsp\x10\xa4/\x12,\n'CmdRogueTournGetCurRogueCocoonInfoCsReq\
    \x10\xa7/\x12$\n\x1fCmdRogueTournGetAllArchiveScRsp\x10\xc1/\x12#\n\x1eC\
    mdRogueTournReviveAvatarScRsp\x10\x95/\x12$\n\x1fCmdRogueTournAreaUpdate\
    ScNotify\x10\x84/\x12\x1b\n\x16CmdRogueTournExpNotify\x10\xc8/\x12\x20\n\
    \x1bCmdRogueTournHandBookNotify\x10\xb6/\x12$\n\x1fCmdRogueTournGetSettl\
    eInfoCsReq\x10\xbb/\x12,\n'CmdRogueTournLeaveRogueCocoonSceneCsReq\x10\
    \x8d/\x12*\n%CmdRogueTournReviveCostUpdateScNotify\x10\xbd/\x12\x1c\n\
    \x17CmdRogueTournEnterScRsp\x10\xab/\x12\x20\n\x1bCmdRogueTournEnterRoom\
    CsReq\x10\xbf/\x12\x1c\n\x17CmdRogueTournStartCsReq\x10\xb8/\x12!\n\x1cC\
    mdRogueTournEnterLayerCsReq\x10\x96/\x12.\n)CmdRogueTournReEnterRogueCoc\
    oonStageCsReq\x10\xcc/b\x06proto3\
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
            let mut messages = ::std::vec::Vec::with_capacity(0);
            let mut enums = ::std::vec::Vec::with_capacity(1);
            enums.push(CmdRogueTournType::generated_enum_descriptor_data());
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
