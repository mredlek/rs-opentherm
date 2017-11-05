use super::{DataValue, SimpleTypeEnum, Message};
use ::serde_derive;
use ::message::{DataId};
use ::{Error, ErrorKind};

macro_rules! complextype {
    ( $( $mat:tt => $id:ident($token:ident) ),* ) =>
        (
        /// An enumeration type which represents the dataid and payload of the message
        #[derive(Copy, Clone, Debug, Deserialize, Serialize)]
        pub enum ComplexType { $( /// Payload for dataid $id
            $id(super::$token::DataIdType) ),* }

        $(
            impl From<super::$token::DataIdType> for ComplexType {
                fn from(input: super::$token::DataIdType) -> ComplexType
                {
                    ComplexType::$id(input)
                }
            }
        )*

        pub(crate) fn to_complex_type<'dv>(dataid: u8, datavalue: DataValue<'dv>) -> Result<ComplexType, Error>
        {
            match dataid {
                $( $mat => {
                    if let Some(check) = super::$token::DATAID_DEFINITION.check {
                        if (check)(super::$token::DATAID_DEFINITION.simple_data(datavalue)).is_err() {
                            return Err(ErrorKind::InvalidApplicationData.into());
                        }
                    }

                    Ok(super::$token::DATAID_DEFINITION.complex_data(datavalue ).into())
                } ),* ,
                _ => Err(ErrorKind::UnknownDataId(dataid).into())
            }
        }

        pub(crate) fn to_simple_type<'dv>(dataid: u8, datavalue: DataValue<'dv>) -> Result<SimpleTypeEnum, Error>
        {
            match dataid {
                $( $mat => {
                    Ok(super::$token::DATAID_DEFINITION.simple_data(datavalue ).into())
                } ),* ,
                _ => Err(ErrorKind::UnknownDataId(dataid).into())
            }
        }

        impl DataId for ComplexType
        {
            fn data_id(&self) -> u8
            {
                match self {
                    $( &ComplexType::$id(_) => $mat ),*
                }
            }
        }

        impl Into<SimpleTypeEnum> for ComplexType
        {
            fn into(self) -> SimpleTypeEnum
            {
                match self
                {
                    $( ComplexType::$id(data) => SimpleTypeEnum::from(super::$token::DATAID_DEFINITION.complex_to_simple(data)) ),*
                }
            }
        }
    )
}

complextype!(
    0 => Status(dataid0),
    1 => TempSet(dataid1),
    2 => MasterConfig(dataid2),
    3 => SlaveConfig(dataid3),
    4 => Command(dataid4),
    5 => ApplicationSpecificFaults(dataid5),
    6 => RemoteBoilerParameters(dataid6),
    7 => Coolingcontrol(dataid7),
    8 => TsetCentralHeating2(dataid8),
    9 => TempRoomOverride(dataid9),
    10 => TransparantSlaveParameters(dataid10),
    11 => TransparantSlaveParameterIndexValue(dataid11),
    12 => FaultHistoryBufferSize(dataid12),
    13 => FaultHistoryBufferIndexValue(dataid13),
    14 => MaxRelModLevelSetting(dataid14),
    15 => MaxCapicityMinModLevel(dataid15),
    16 => TempRoomSet(dataid16),
    17 => RelModLevel(dataid17),
    18 => CentralHeatingPressure(dataid18),
    19 => DomesticHotWaterFlowRate(dataid19),
    20 => DayTime(dataid20),
    21 => Date(dataid21),
    22 => Year(dataid22),
    23 => TempRoomSetCentralHeating2(dataid23),
    24 => TempRoom(dataid24),
    25 => TempBoiler(dataid25),
    26 => TempDomesticHotWater(dataid26),
    27 => TempOutside(dataid27),
    28 => TempReturn(dataid28),
    29 => TempStorage(dataid29),
    30 => TempCollector(dataid30),
    31 => TempFlowCentralHeating2(dataid31),
    32 => TempFlowDomesticHotWater2(dataid32),
    33 => TempBoilerExhaust(dataid33),
    48 => TempDomesticHotWaterBounds(dataid48),
    49 => MaxTempSetBounds(dataid49),
    //50 => OtcHeatCurveRatioBounds(dataid50),
    56 => TempDomesticHotWaterSet(dataid56),
    57 => MaxTempSet(dataid57),
    //58 => OtcHeatCurveRatio(dataid58),
    100 => RemoteOverrideFunction(dataid100),
    115 => OemDiagnosticsCode(dataid115),
    116 => BurnerStarts(dataid116),
    117 => CentralHeatingPumpStarts(dataid117),
    118 => DomesticHotWaterStarts(dataid118),
    119 => DomesticHotWaterBurnerStarts(dataid119),
    120 => BurnerOperationHours(dataid120),
    121 => CentralHeatingPumpOperationHours(dataid121),
    122 => DomesticHotWaterOperationHours(dataid122),
    123 => DomesticHotWaterBurnerOperationHours(dataid123),
    124 => OpenthermVersionMaster(dataid124),
    125 => OpenthermVersionSlave(dataid125),
    126 => MasterVersion(dataid126),
    127 => SlaveVersion(dataid127)
);

impl ComplexType
{
    pub(crate) fn new(msg: &(Message + 'static)) -> Result<ComplexType, Error>
    {
        to_complex_type(msg.data_id(), msg.data_value())
    }
}

/*pub enum ComplexType
{
    Status(DataId0Type)
}*/