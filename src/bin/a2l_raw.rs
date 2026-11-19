//! `a2l-raw <input.a2l> <output.json>`: write the first MODULE as parser-neutral
//! JSON for Python-side normalization. Input must be UTF-8 (see `utf8_check`);
//! failures exit non-zero with a `path:line:` message on stderr.
#![allow(non_snake_case)]

#[path = "a2l_raw/utf8_check.rs"]
mod utf8_check;

use std::path::Path;
use std::time::Instant;

use a2lfile_core::{
    A2lObjectName, AxisDescrAttribute, CalibrationAccessEnum, CharacteristicType, DataType,
    DepositMode, IndexMode, IndexOrder, MemoryAttribute, MemoryType, Module, PrgType, RecordLayout,
};
use serde::Serialize;

#[derive(Serialize, Default)]
struct RawModel {
    name: String,
    longIdentifier: String,
    mod_common: Option<RawCommon>,
    mod_par: Option<RawPar>,
    compu_method: Vec<RawCompuMethod>,
    compu_vtab: Vec<RawCompuVtab>,
    record_layout: Vec<RawLayout>,
    axis_pts: Vec<RawAxisPts>,
    measurement: Vec<RawMeasurement>,
    characteristic: Vec<RawCharacteristic>,
    group: Vec<RawGroup>,
    function: Vec<RawFunction>,
}

#[derive(Serialize, Default)]
struct RawCommon {
    byte_order: Option<RawByteOrder>,
    deposit: Option<RawDeposit>,
    alignment_byte: Option<RawBorder>,
    alignment_word: Option<RawBorder>,
    alignment_long: Option<RawBorder>,
    alignment_int64: Option<RawBorder>,
    alignment_float16_ieee: Option<RawBorder>,
    alignment_float32_ieee: Option<RawBorder>,
    alignment_float64_ieee: Option<RawBorder>,
}

#[derive(Serialize)]
struct RawByteOrder {
    byteOrder: String,
}

#[derive(Serialize)]
struct RawDeposit {
    mode: String,
}

#[derive(Serialize)]
struct RawBorder {
    alignmentBorder: u16,
}

#[derive(Serialize, Default)]
struct RawPar {
    epk: Option<RawEpk>,
    addr_epk: Vec<RawAddrEpk>,
    ecu_calibration_offset: Option<RawOffset>,
    memory_segment: Vec<RawSegment>,
}

#[derive(Serialize)]
struct RawEpk {
    identifier: String,
}

#[derive(Serialize)]
struct RawAddrEpk {
    address: u32,
}

#[derive(Serialize)]
struct RawOffset {
    offset: i32,
}

#[derive(Serialize)]
struct RawSegment {
    name: String,
    longIdentifier: String,
    prgType: String,
    memoryType: String,
    attribute: String,
    address: u32,
    size: u32,
    offset_0: i32,
    offset_1: i32,
    offset_2: i32,
    offset_3: i32,
    offset_4: i32,
}

#[derive(Serialize, Default)]
struct RawCompuMethod {
    name: String,
    conversionType: String,
    format: Option<String>,
    unit: Option<String>,
    coeffs_linear: Option<RawCoeffsLinear>,
    coeffs: Option<RawCoeffs>,
    compu_tab_ref: Option<RawTabRef>,
}

#[derive(Serialize)]
struct RawCoeffsLinear {
    a: f64,
    b: f64,
}

#[derive(Serialize)]
struct RawCoeffs {
    a: f64,
    b: f64,
    c: f64,
    d: f64,
    e: f64,
    f: f64,
}

#[derive(Serialize)]
struct RawTabRef {
    conversionTable: String,
}

#[derive(Serialize, Default)]
struct RawCompuVtab {
    name: String,
    pairs: Vec<RawPair>,
}

#[derive(Serialize)]
struct RawPair {
    inVal: f64,
    outVal: String,
}

#[derive(Serialize, Default)]
struct RawLayout {
    name: String,
    static_address_offsets: bool,
    static_record_layout: bool,
    alignment_byte: Option<RawBorder>,
    alignment_word: Option<RawBorder>,
    alignment_long: Option<RawBorder>,
    alignment_int64: Option<RawBorder>,
    alignment_float16_ieee: Option<RawBorder>,
    alignment_float32_ieee: Option<RawBorder>,
    alignment_float64_ieee: Option<RawBorder>,
    fnc_values: Option<RawFncValues>,
    axis_pts_x: Option<RawAxisDim>,
    axis_pts_y: Option<RawAxisDim>,
    axis_pts_z: Option<RawAxisDim>,
    axis_pts_4: Option<RawAxisDim>,
    axis_pts_5: Option<RawAxisDim>,
    no_axis_pts_x: Option<RawCountDim>,
    no_axis_pts_y: Option<RawCountDim>,
    no_axis_pts_z: Option<RawCountDim>,
    no_axis_pts_4: Option<RawCountDim>,
    no_axis_pts_5: Option<RawCountDim>,
    fix_no_axis_pts_x: Option<RawFixCountDim>,
    fix_no_axis_pts_y: Option<RawFixCountDim>,
    fix_no_axis_pts_z: Option<RawFixCountDim>,
    fix_no_axis_pts_4: Option<RawFixCountDim>,
    fix_no_axis_pts_5: Option<RawFixCountDim>,
    axis_rescale_x: Option<RawRescaleDim>,
    axis_rescale_y: Option<RawRescaleDim>,
    axis_rescale_z: Option<RawRescaleDim>,
    axis_rescale_4: Option<RawRescaleDim>,
    axis_rescale_5: Option<RawRescaleDim>,
    dist_op_x: Option<RawRescaleDim>,
    dist_op_y: Option<RawRescaleDim>,
    dist_op_z: Option<RawRescaleDim>,
    dist_op_4: Option<RawRescaleDim>,
    dist_op_5: Option<RawRescaleDim>,
    no_rescale_x: Option<RawCountDim>,
    no_rescale_y: Option<RawCountDim>,
    no_rescale_z: Option<RawCountDim>,
    no_rescale_4: Option<RawCountDim>,
    no_rescale_5: Option<RawCountDim>,
    offset_x: Option<RawRescaleDim>,
    offset_y: Option<RawRescaleDim>,
    offset_z: Option<RawRescaleDim>,
    offset_4: Option<RawRescaleDim>,
    offset_5: Option<RawRescaleDim>,
    src_addr_x: Option<RawRescaleDim>,
    src_addr_y: Option<RawRescaleDim>,
    src_addr_z: Option<RawRescaleDim>,
    src_addr_4: Option<RawRescaleDim>,
    src_addr_5: Option<RawRescaleDim>,
    shift_op_x: Option<RawRescaleDim>,
    shift_op_y: Option<RawRescaleDim>,
    shift_op_z: Option<RawRescaleDim>,
    shift_op_4: Option<RawRescaleDim>,
    shift_op_5: Option<RawRescaleDim>,
    rip_addr_w: Option<RawRescaleDim>,
    rip_addr_x: Option<RawRescaleDim>,
    rip_addr_y: Option<RawRescaleDim>,
    rip_addr_z: Option<RawRescaleDim>,
    rip_addr_4: Option<RawRescaleDim>,
    rip_addr_5: Option<RawRescaleDim>,
    reserved: Option<RawRescaleDim>,
    identification: Option<RawRescaleDim>,
}

#[derive(Serialize)]
struct RawFncValues {
    position: u16,
    datatype: String,
    indexMode: String,
    addresstype: String,
}

#[derive(Serialize)]
struct RawAxisDim {
    position: u16,
    datatype: String,
    indexIncr: String,
    addressing: String,
}

#[derive(Serialize)]
struct RawCountDim {
    position: u16,
    datatype: String,
}

#[derive(Serialize)]
struct RawFixCountDim {
    numberOfAxisPoints: u16,
}

#[derive(Serialize)]
struct RawRescaleDim {
    position: u16,
    datatype: String,
}

#[derive(Serialize, Default)]
struct RawAxisPts {
    name: String,
    longIdentifier: String,
    address: u32,
    depositAttr: String,
    deposit: Option<RawDeposit>,
    conversion: String,
    maxAxisPoints: u16,
    lowerLimit: f64,
    upperLimit: f64,
    byte_order: Option<RawByteOrder>,
    ecu_address_extension: Option<RawExtension>,
    read_only: bool,
}

#[derive(Serialize)]
struct RawExtension {
    extension: i16,
}

#[derive(Serialize, Default)]
struct RawAxisDescr {
    attribute: String,
    inputQuantity: String,
    conversion: String,
    maxAxisPoints: u16,
    lowerLimit: f64,
    upperLimit: f64,
    axis_pts_ref: Option<RawAxisPtsRef>,
    byte_order: Option<RawByteOrder>,
    deposit: Option<RawDeposit>,
    fix_axis_par: Option<RawFixAxisPar>,
    fix_axis_par_dist: Option<RawFixAxisParDist>,
    fix_axis_par_list: Option<RawFixAxisParList>,
}

#[derive(Serialize)]
struct RawAxisPtsRef {
    axisPoints: String,
}

#[derive(Serialize)]
struct RawFixAxisPar {
    offset: f64,
    shift: f64,
    numberapo: u16,
}

#[derive(Serialize)]
struct RawFixAxisParDist {
    offset: f64,
    distance: f64,
    numberapo: u16,
}

#[derive(Serialize)]
struct RawFixAxisParList {
    axisPts_Value: Vec<f64>,
}

#[derive(Serialize, Default)]
struct RawMeasurement {
    name: String,
    longIdentifier: String,
    datatype: String,
    conversion: String,
    lowerLimit: f64,
    upperLimit: f64,
    ecu_address: Option<RawAddress>,
    ecu_address_extension: Option<RawExtension>,
    array_size: Option<RawSize>,
    bit_mask: bool,
    bit_operation: bool,
    byte_order: Option<RawByteOrder>,
    matrix_dim: Option<RawMatrixDim>,
    address_type: Option<RawAddrType>,
    #[serde(rename = "virtual")]
    r#virtual: bool,
    phys_unit: Option<RawUnit>,
    step_size: Option<RawStep>,
    format: Option<RawFormat>,
    read_write: bool,
    function_list: Option<RawFunctionList>,
}

#[derive(Serialize)]
struct RawAddress {
    address: u32,
}

#[derive(Serialize)]
struct RawSize {
    number: u16,
}

#[derive(Serialize)]
struct RawMatrixDim {
    numbers: Vec<u16>,
}

#[derive(Serialize)]
struct RawAddrType {
    addressType: String,
}

#[derive(Serialize)]
struct RawUnit {
    unit: String,
}

#[derive(Serialize)]
struct RawStep {
    stepSize: f64,
}

#[derive(Serialize)]
struct RawFormat {
    formatString: String,
}

#[derive(Serialize)]
struct RawFunctionList {
    name: Vec<String>,
}

#[derive(Serialize, Default)]
struct RawCharacteristic {
    name: String,
    longIdentifier: String,
    r#type: String,
    address: u32,
    deposit: String,
    conversion: String,
    lowerLimit: f64,
    upperLimit: f64,
    axis_descr: Vec<RawAxisDescr>,
    bit_mask: bool,
    bit_operation: bool,
    byte_order: Option<RawByteOrder>,
    matrix_dim: Option<RawMatrixDim>,
    number: Option<RawNumber>,
    address_type: Option<RawAddrType>,
    #[serde(rename = "virtual")]
    r#virtual: bool,
    virtual_characteristic: bool,
    dependent_characteristic: bool,
    calibration_access: Option<RawAccess>,
    read_only: bool,
    ecu_address_extension: Option<RawExtension>,
    phys_unit: Option<RawUnit>,
    step_size: Option<RawStep>,
    format: Option<RawFormat>,
    function_list: Option<RawFunctionList>,
}

#[derive(Serialize)]
struct RawNumber {
    number: u16,
}

#[derive(Serialize)]
struct RawAccess {
    r#type: String,
}

#[derive(Serialize, Default)]
struct RawGroup {
    groupName: String,
    ref_measurement: Option<RawIdentifiers>,
    ref_characteristic: Option<RawIdentifiers>,
    sub_group: Option<RawIdentifiers>,
    function_list: Option<RawFunctionList>,
}

#[derive(Serialize)]
struct RawIdentifiers {
    identifier: Vec<String>,
}

#[derive(Serialize, Default)]
struct RawFunction {
    name: String,
    def_characteristic: Option<RawIdentifiers>,
    ref_characteristic: Option<RawIdentifiers>,
    in_measurement: Option<RawIdentifiers>,
    out_measurement: Option<RawIdentifiers>,
    loc_measurement: Option<RawIdentifiers>,
    sub_function: Option<RawIdentifiers>,
}

fn datatype(value: &DataType) -> &'static str {
    match value {
        DataType::Ubyte => "UBYTE",
        DataType::Sbyte => "SBYTE",
        DataType::Uword => "UWORD",
        DataType::Sword => "SWORD",
        DataType::Ulong => "ULONG",
        DataType::Slong => "SLONG",
        DataType::AUint64 => "A_UINT64",
        DataType::AInt64 => "A_INT64",
        DataType::Float16Ieee => "FLOAT16_IEEE",
        DataType::Float32Ieee => "FLOAT32_IEEE",
        DataType::Float64Ieee => "FLOAT64_IEEE",
    }
}

fn index_mode(value: &IndexMode) -> &'static str {
    match value {
        IndexMode::RowDir => "ROW_DIR",
        IndexMode::ColumnDir => "COLUMN_DIR",
        IndexMode::AlternateCurves => "ALTERNATE_CURVES",
        IndexMode::AlternateWithX => "ALTERNATE_WITH_X",
        IndexMode::AlternateWithY => "ALTERNATE_WITH_Y",
    }
}

fn index_order(value: &IndexOrder) -> &'static str {
    match value {
        IndexOrder::IndexIncr => "INDEX_INCR",
        IndexOrder::IndexDecr => "INDEX_DECR",
    }
}

fn addr_type(value: &a2lfile_core::AddrType) -> &'static str {
    use a2lfile_core::AddrType::*;
    match value {
        Direct => "DIRECT",
        Pbyte => "PBYTE",
        Pword => "PWORD",
        Plong => "PLONG",
        Plonglong => "PLONGLONG",
    }
}

fn byte_order(value: &a2lfile_core::ByteOrderEnum) -> &'static str {
    use a2lfile_core::ByteOrderEnum::*;
    match value {
        MsbLast => "MSB_LAST",
        MsbFirst => "MSB_FIRST",
        LittleEndian => "LITTLE_ENDIAN",
        BigEndian => "BIG_ENDIAN",
        MsbFirstMswLast => "MSB_FIRST_MSW_LAST",
        MsbLastMswFirst => "MSB_LAST_MSW_FIRST",
    }
}

fn deposit_mode(value: &DepositMode) -> &'static str {
    match value {
        DepositMode::Absolute => "ABSOLUTE",
        DepositMode::Difference => "DIFFERENCE",
    }
}

fn characteristic_type(value: &CharacteristicType) -> &'static str {
    use CharacteristicType::*;
    match value {
        Value => "VALUE",
        Curve => "CURVE",
        Map => "MAP",
        Cuboid => "CUBOID",
        Cube4 => "CUBE_4",
        Cube5 => "CUBE_5",
        ValBlk => "VAL_BLK",
        Ascii => "ASCII",
    }
}

fn axis_attribute(value: &AxisDescrAttribute) -> &'static str {
    use AxisDescrAttribute::*;
    match value {
        FixAxis => "FIX_AXIS",
        StdAxis => "STD_AXIS",
        ComAxis => "COM_AXIS",
        CurveAxis => "CURVE_AXIS",
        ResAxis => "RES_AXIS",
    }
}

fn access(value: &CalibrationAccessEnum) -> &'static str {
    use CalibrationAccessEnum::*;
    match value {
        Calibration => "CALIBRATION",
        NoCalibration => "NO_CALIBRATION",
        NotInMcdSystem => "NOT_IN_MCD_SYSTEM",
        OfflineCalibration => "OFFLINE_CALIBRATION",
    }
}

fn prg_type(value: &PrgType) -> &'static str {
    use PrgType::*;
    match value {
        CalibrationVariables => "CALIBRATION_VARIABLES",
        Code => "CODE",
        Data => "DATA",
        ExcludeFromFlash => "EXCLUDE_FROM_FLASH",
        OfflineData => "OFFLINE_DATA",
        Reserved => "RESERVED",
        Seram => "SERAM",
        Variables => "VARIABLES",
    }
}

fn memory_type(value: &MemoryType) -> &'static str {
    use MemoryType::*;
    match value {
        Eeprom => "EEPROM",
        Eprom => "EPROM",
        Flash => "FLASH",
        Ram => "RAM",
        Rom => "ROM",
        Register => "REGISTER",
        NotInEcu => "NOT_IN_ECU",
    }
}

fn memory_attribute(value: &MemoryAttribute) -> &'static str {
    match value {
        MemoryAttribute::Intern => "INTERN",
        MemoryAttribute::Extern => "EXTERN",
    }
}

fn border(value: &Option<impl Border>) -> Option<RawBorder> {
    value.as_ref().map(|b| RawBorder {
        alignmentBorder: b.get_border(),
    })
}

trait Border {
    fn get_border(&self) -> u16;
}

macro_rules! border_impl {
    ($($t:ty),*) => {$(
        impl Border for $t {
            fn get_border(&self) -> u16 { self.alignment_border }
        }
    )*};
}
border_impl!(
    a2lfile_core::AlignmentByte,
    a2lfile_core::AlignmentWord,
    a2lfile_core::AlignmentLong,
    a2lfile_core::AlignmentInt64,
    a2lfile_core::AlignmentFloat16Ieee,
    a2lfile_core::AlignmentFloat32Ieee,
    a2lfile_core::AlignmentFloat64Ieee
);

fn common(module: &Module) -> Option<RawCommon> {
    let c = module.mod_common.as_ref()?;
    Some(RawCommon {
        byte_order: c.byte_order.as_ref().map(|b| RawByteOrder {
            byteOrder: byte_order(&b.byte_order).into(),
        }),
        deposit: c.deposit.as_ref().map(|d| RawDeposit {
            mode: deposit_mode(&d.mode).into(),
        }),
        alignment_byte: border(&c.alignment_byte),
        alignment_word: border(&c.alignment_word),
        alignment_long: border(&c.alignment_long),
        alignment_int64: border(&c.alignment_int64),
        alignment_float16_ieee: border(&c.alignment_float16_ieee),
        alignment_float32_ieee: border(&c.alignment_float32_ieee),
        alignment_float64_ieee: border(&c.alignment_float64_ieee),
    })
}

fn par(module: &Module) -> Option<RawPar> {
    let p = module.mod_par.as_ref()?;
    Some(RawPar {
        epk: p.epk.as_ref().map(|e| RawEpk {
            identifier: e.identifier.clone(),
        }),
        addr_epk: p
            .addr_epk
            .iter()
            .map(|a| RawAddrEpk { address: a.address })
            .collect(),
        ecu_calibration_offset: p
            .ecu_calibration_offset
            .as_ref()
            .map(|o| RawOffset { offset: o.offset }),
        memory_segment: p
            .memory_segment
            .iter()
            .map(|s| RawSegment {
                name: s.get_name().to_string(),
                longIdentifier: s.long_identifier.clone(),
                prgType: prg_type(&s.prg_type).into(),
                memoryType: memory_type(&s.memory_type).into(),
                attribute: memory_attribute(&s.attribute).into(),
                address: s.address,
                size: s.size,
                offset_0: s.offset[0],
                offset_1: s.offset[1],
                offset_2: s.offset[2],
                offset_3: s.offset[3],
                offset_4: s.offset[4],
            })
            .collect(),
    })
}

fn layout(record: &RecordLayout) -> RawLayout {
    let fnc = record.fnc_values.as_ref().map(|f| RawFncValues {
        position: f.position,
        datatype: datatype(&f.datatype).into(),
        indexMode: index_mode(&f.index_mode).into(),
        addresstype: addr_type(&f.address_type).into(),
    });
    let axis = |d: &Option<a2lfile_core::AxisPtsDim>| {
        d.as_ref().map(|a| RawAxisDim {
            position: a.position,
            datatype: datatype(&a.datatype).into(),
            indexIncr: index_order(&a.index_incr).into(),
            addressing: "DIRECT".into(),
        })
    };
    let count = |d: &Option<a2lfile_core::NoAxisPtsDim>| {
        d.as_ref().map(|a| RawCountDim {
            position: a.position,
            datatype: datatype(&a.datatype).into(),
        })
    };
    let fix = |d: &Option<a2lfile_core::FixNoAxisPtsDim>| {
        d.as_ref().map(|a| RawFixCountDim {
            numberOfAxisPoints: a.number_of_axis_points,
        })
    };
    let count_present = |d: &Option<a2lfile_core::NoRescaleDim>| {
        d.as_ref().map(|_| RawCountDim {
            position: 0,
            datatype: String::new(),
        })
    };

    RawLayout {
        name: record.get_name().to_string(),
        static_address_offsets: record.static_address_offsets.is_some(),
        static_record_layout: record.static_record_layout.is_some(),
        alignment_byte: border(&record.alignment_byte),
        alignment_word: border(&record.alignment_word),
        alignment_long: border(&record.alignment_long),
        alignment_int64: border(&record.alignment_int64),
        alignment_float16_ieee: border(&record.alignment_float16_ieee),
        alignment_float32_ieee: border(&record.alignment_float32_ieee),
        alignment_float64_ieee: border(&record.alignment_float64_ieee),
        fnc_values: fnc,
        axis_pts_x: axis(&record.axis_pts_x),
        axis_pts_y: axis(&record.axis_pts_y),
        axis_pts_z: axis(&record.axis_pts_z),
        axis_pts_4: axis(&record.axis_pts_4),
        axis_pts_5: axis(&record.axis_pts_5),
        no_axis_pts_x: count(&record.no_axis_pts_x),
        no_axis_pts_y: count(&record.no_axis_pts_y),
        no_axis_pts_z: count(&record.no_axis_pts_z),
        no_axis_pts_4: count(&record.no_axis_pts_4),
        no_axis_pts_5: count(&record.no_axis_pts_5),
        fix_no_axis_pts_x: fix(&record.fix_no_axis_pts_x),
        fix_no_axis_pts_y: fix(&record.fix_no_axis_pts_y),
        fix_no_axis_pts_z: fix(&record.fix_no_axis_pts_z),
        fix_no_axis_pts_4: fix(&record.fix_no_axis_pts_4),
        fix_no_axis_pts_5: fix(&record.fix_no_axis_pts_5),
        axis_rescale_x: other_present(record.axis_rescale_x.is_some()),
        axis_rescale_y: other_present(record.axis_rescale_y.is_some()),
        axis_rescale_z: other_present(record.axis_rescale_z.is_some()),
        axis_rescale_4: other_present(record.axis_rescale_4.is_some()),
        axis_rescale_5: other_present(record.axis_rescale_5.is_some()),
        dist_op_x: other_present(record.dist_op_x.is_some()),
        dist_op_y: other_present(record.dist_op_y.is_some()),
        dist_op_z: other_present(record.dist_op_z.is_some()),
        dist_op_4: other_present(record.dist_op_4.is_some()),
        dist_op_5: other_present(record.dist_op_5.is_some()),
        no_rescale_x: count_present(&record.no_rescale_x),
        no_rescale_y: count_present(&record.no_rescale_y),
        no_rescale_z: count_present(&record.no_rescale_z),
        no_rescale_4: count_present(&record.no_rescale_4),
        no_rescale_5: count_present(&record.no_rescale_5),
        offset_x: other_present(record.offset_x.is_some()),
        offset_y: other_present(record.offset_y.is_some()),
        offset_z: other_present(record.offset_z.is_some()),
        offset_4: other_present(record.offset_4.is_some()),
        offset_5: other_present(record.offset_5.is_some()),
        src_addr_x: other_present(record.src_addr_x.is_some()),
        src_addr_y: other_present(record.src_addr_y.is_some()),
        src_addr_z: other_present(record.src_addr_z.is_some()),
        src_addr_4: other_present(record.src_addr_4.is_some()),
        src_addr_5: other_present(record.src_addr_5.is_some()),
        shift_op_x: other_present(record.shift_op_x.is_some()),
        shift_op_y: other_present(record.shift_op_y.is_some()),
        shift_op_z: other_present(record.shift_op_z.is_some()),
        shift_op_4: other_present(record.shift_op_4.is_some()),
        shift_op_5: other_present(record.shift_op_5.is_some()),
        rip_addr_w: other_present(record.rip_addr_w.is_some()),
        rip_addr_x: other_present(record.rip_addr_x.is_some()),
        rip_addr_y: other_present(record.rip_addr_y.is_some()),
        rip_addr_z: other_present(record.rip_addr_z.is_some()),
        rip_addr_4: other_present(record.rip_addr_4.is_some()),
        rip_addr_5: other_present(record.rip_addr_5.is_some()),
        reserved: other_present(!record.reserved.is_empty()),
        identification: other_present(record.identification.is_some()),
    }
}

fn other_present(present: bool) -> Option<RawRescaleDim> {
    present.then(|| RawRescaleDim {
        position: 0,
        datatype: String::new(),
    })
}

fn function_list(value: &Option<a2lfile_core::FunctionList>) -> Option<RawFunctionList> {
    value.as_ref().map(|f| RawFunctionList {
        name: f.name_list.clone(),
    })
}

fn identifiers(value: &Option<impl Identifiers>) -> Option<RawIdentifiers> {
    value.as_ref().map(|v| RawIdentifiers {
        identifier: v.get_identifier_list().clone(),
    })
}

trait Identifiers {
    fn get_identifier_list(&self) -> &Vec<String>;
}

macro_rules! identifiers_impl {
    ($($t:ty),*) => {$(
        impl Identifiers for $t {
            fn get_identifier_list(&self) -> &Vec<String> { &self.identifier_list }
        }
    )*};
}
identifiers_impl!(
    a2lfile_core::RefMeasurement,
    a2lfile_core::RefCharacteristic,
    a2lfile_core::SubGroup,
    a2lfile_core::DefCharacteristic,
    a2lfile_core::InMeasurement,
    a2lfile_core::OutMeasurement,
    a2lfile_core::LocMeasurement,
    a2lfile_core::SubFunction
);

fn dump(module: &Module) -> RawModel {
    RawModel {
        name: module.get_name().to_string(),
        longIdentifier: module.long_identifier.clone(),
        mod_common: common(module),
        mod_par: par(module),
        compu_method: module
            .compu_method
            .iter()
            .map(|m| RawCompuMethod {
                name: m.get_name().to_string(),
                conversionType: match m.conversion_type {
                    a2lfile_core::ConversionType::Identical => "IDENTICAL",
                    a2lfile_core::ConversionType::Form => "FORM",
                    a2lfile_core::ConversionType::Linear => "LINEAR",
                    a2lfile_core::ConversionType::RatFunc => "RAT_FUNC",
                    a2lfile_core::ConversionType::TabIntp => "TAB_INTP",
                    a2lfile_core::ConversionType::TabNointp => "TAB_NOINTP",
                    a2lfile_core::ConversionType::TabVerb => "TAB_VERB",
                }
                .into(),
                format: Some(m.format.clone()),
                unit: Some(m.unit.clone()),
                coeffs_linear: m
                    .coeffs_linear
                    .as_ref()
                    .map(|c| RawCoeffsLinear { a: c.a, b: c.b }),
                coeffs: m.coeffs.as_ref().map(|c| RawCoeffs {
                    a: c.a,
                    b: c.b,
                    c: c.c,
                    d: c.d,
                    e: c.e,
                    f: c.f,
                }),
                compu_tab_ref: m.compu_tab_ref.as_ref().map(|r| RawTabRef {
                    conversionTable: r.conversion_table.clone(),
                }),
            })
            .collect(),
        compu_vtab: module
            .compu_vtab
            .iter()
            .map(|t| RawCompuVtab {
                name: t.get_name().to_string(),
                pairs: t
                    .value_pairs
                    .iter()
                    .map(|p| RawPair {
                        inVal: p.in_val,
                        outVal: p.out_val.clone(),
                    })
                    .collect(),
            })
            .collect(),
        record_layout: module.record_layout.iter().map(layout).collect(),
        axis_pts: module
            .axis_pts
            .iter()
            .map(|a| RawAxisPts {
                name: a.get_name().to_string(),
                longIdentifier: a.long_identifier.clone(),
                address: a.address,
                depositAttr: a.deposit_record.clone(),
                deposit: a.deposit.as_ref().map(|d| RawDeposit {
                    mode: deposit_mode(&d.mode).into(),
                }),
                conversion: a.conversion.clone(),
                maxAxisPoints: a.max_axis_points,
                lowerLimit: a.lower_limit,
                upperLimit: a.upper_limit,
                byte_order: a.byte_order.as_ref().map(|b| RawByteOrder {
                    byteOrder: byte_order(&b.byte_order).into(),
                }),
                ecu_address_extension: a.ecu_address_extension.as_ref().map(|e| RawExtension {
                    extension: e.extension,
                }),
                read_only: a.read_only.is_some(),
            })
            .collect(),
        measurement: module
            .measurement
            .iter()
            .map(|m| RawMeasurement {
                name: m.get_name().to_string(),
                longIdentifier: m.long_identifier.clone(),
                datatype: datatype(&m.datatype).into(),
                conversion: m.conversion.clone(),
                lowerLimit: m.lower_limit,
                upperLimit: m.upper_limit,
                ecu_address: m
                    .ecu_address
                    .as_ref()
                    .map(|e| RawAddress { address: e.address }),
                ecu_address_extension: m.ecu_address_extension.as_ref().map(|e| RawExtension {
                    extension: e.extension,
                }),
                array_size: m.array_size.as_ref().map(|a| RawSize { number: a.number }),
                bit_mask: m.bit_mask.is_some(),
                bit_operation: m.bit_operation.is_some(),
                byte_order: m.byte_order.as_ref().map(|b| RawByteOrder {
                    byteOrder: byte_order(&b.byte_order).into(),
                }),
                matrix_dim: m.matrix_dim.as_ref().map(|d| RawMatrixDim {
                    numbers: d.dim_list.clone(),
                }),
                address_type: m.address_type.as_ref().map(|a| RawAddrType {
                    addressType: addr_type(&a.address_type).into(),
                }),
                r#virtual: m.var_virtual.is_some(),
                phys_unit: m.phys_unit.as_ref().map(|u| RawUnit {
                    unit: u.unit.clone(),
                }),
                step_size: None,
                format: m.format.as_ref().map(|f| RawFormat {
                    formatString: f.format_string.clone(),
                }),
                read_write: m.read_write.is_some(),
                function_list: function_list(&m.function_list),
            })
            .collect(),
        characteristic: module
            .characteristic
            .iter()
            .map(|c| RawCharacteristic {
                name: c.get_name().to_string(),
                longIdentifier: c.long_identifier.clone(),
                r#type: characteristic_type(&c.characteristic_type).into(),
                address: c.address,
                deposit: c.deposit.clone(),
                conversion: c.conversion.clone(),
                lowerLimit: c.lower_limit,
                upperLimit: c.upper_limit,
                axis_descr: c
                    .axis_descr
                    .iter()
                    .map(|a| RawAxisDescr {
                        attribute: axis_attribute(&a.attribute).into(),
                        inputQuantity: a.input_quantity.clone(),
                        conversion: a.conversion.clone(),
                        maxAxisPoints: a.max_axis_points,
                        lowerLimit: a.lower_limit,
                        upperLimit: a.upper_limit,
                        axis_pts_ref: a.axis_pts_ref.as_ref().map(|r| RawAxisPtsRef {
                            axisPoints: r.axis_points.clone(),
                        }),
                        byte_order: a.byte_order.as_ref().map(|b| RawByteOrder {
                            byteOrder: byte_order(&b.byte_order).into(),
                        }),
                        deposit: a.deposit.as_ref().map(|d| RawDeposit {
                            mode: deposit_mode(&d.mode).into(),
                        }),
                        fix_axis_par: a.fix_axis_par.as_ref().map(|f| RawFixAxisPar {
                            offset: f.offset,
                            shift: f.shift,
                            numberapo: f.number_apo,
                        }),
                        fix_axis_par_dist: a.fix_axis_par_dist.as_ref().map(|f| {
                            RawFixAxisParDist {
                                offset: f.offset,
                                distance: f.distance,
                                numberapo: f.number_apo,
                            }
                        }),
                        fix_axis_par_list: a.fix_axis_par_list.as_ref().map(|f| {
                            RawFixAxisParList {
                                axisPts_Value: f.axis_pts_value_list.clone(),
                            }
                        }),
                    })
                    .collect(),
                bit_mask: c.bit_mask.is_some(),
                bit_operation: false,
                byte_order: c.byte_order.as_ref().map(|b| RawByteOrder {
                    byteOrder: byte_order(&b.byte_order).into(),
                }),
                matrix_dim: c.matrix_dim.as_ref().map(|d| RawMatrixDim {
                    numbers: d.dim_list.clone(),
                }),
                number: c.number.as_ref().map(|n| RawNumber { number: n.number }),
                address_type: None,
                r#virtual: false,
                virtual_characteristic: c.virtual_characteristic.is_some(),
                dependent_characteristic: c.dependent_characteristic.is_some(),
                calibration_access: c.calibration_access.as_ref().map(|a| RawAccess {
                    r#type: access(&a.calibration_access).into(),
                }),
                read_only: c.read_only.is_some(),
                ecu_address_extension: c.ecu_address_extension.as_ref().map(|e| RawExtension {
                    extension: e.extension,
                }),
                phys_unit: c.phys_unit.as_ref().map(|u| RawUnit {
                    unit: u.unit.clone(),
                }),
                step_size: c.step_size.as_ref().map(|s| RawStep {
                    stepSize: s.step_size,
                }),
                format: c.format.as_ref().map(|f| RawFormat {
                    formatString: f.format_string.clone(),
                }),
                function_list: function_list(&c.function_list),
            })
            .collect(),
        group: module
            .group
            .iter()
            .map(|g| RawGroup {
                groupName: g.get_name().to_string(),
                ref_measurement: identifiers(&g.ref_measurement),
                ref_characteristic: identifiers(&g.ref_characteristic),
                sub_group: identifiers(&g.sub_group),
                function_list: function_list(&g.function_list),
            })
            .collect(),
        function: module
            .function
            .iter()
            .map(|f| RawFunction {
                name: f.get_name().to_string(),
                def_characteristic: identifiers(&f.def_characteristic),
                ref_characteristic: identifiers(&f.ref_characteristic),
                in_measurement: identifiers(&f.in_measurement),
                out_measurement: identifiers(&f.out_measurement),
                loc_measurement: identifiers(&f.loc_measurement),
                sub_function: identifiers(&f.sub_function),
            })
            .collect(),
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let path = args.get(1).expect("usage: a2l-raw <file.a2l> <out.json>");
    let output = args.get(2).expect("usage: a2l-raw <file.a2l> <out.json>");

    let t0 = Instant::now();
    if let Err(message) = utf8_check::check_tree(Path::new(path)) {
        eprintln!("load failed: {message}");
        std::process::exit(1);
    }
    let (file, errors) = match a2lfile_core::load(path, None, false) {
        Ok(x) => x,
        Err(e) => {
            eprintln!("load failed: {e}");
            std::process::exit(1);
        }
    };
    let parse_time = t0.elapsed();
    eprintln!("parse: {parse_time:?}; errors: {}", errors.len());

    let t1 = Instant::now();
    let model = file.project.module.first().map(dump).unwrap_or_default();
    let model_time = t1.elapsed();

    let json = serde_json::to_string(&model).expect("serialize");
    let write_start = Instant::now();
    std::fs::write(output, &json).expect("write");
    eprintln!(
        "dump: {model_time:?}; serialize+write: {:?}; size: {:.1} MB",
        write_start.elapsed(),
        json.len() as f64 / 1e6
    );
}
