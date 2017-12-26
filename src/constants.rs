pub const HCI_COMMAND_PKT: u8 = 0x01;

// hci.h
pub const HCI_FILTER: i32 = 2;
pub const HCI_EVENT_PKT: i32 = 0x04;
pub const HCI_ACLDATA_PKT: i32 = 0x02;
pub const HCI_LE_META_EVENT: i32 = 0x3E;

// bluetooth.h
pub const SOL_HCI: i32 = 0;

pub const ATT_CID: u16 = 4;
pub const ATT_OP_READ_BY_GROUP_REQ: u8 = 0x10;
pub const GATT_CHARAC_UUID: u16 = 0x2803;

pub const EVT_DISCONN_COMPLETE: u8 = 0x05;
pub const EVT_ENCRYPT_CHANGE: u8 = 0x08;
pub const EVT_CMD_COMPLETE: u8 = 0x0e;
pub const EVT_CMD_STATUS: u8 = 0x0f;
pub const EVT_LE_META_EVENT: u8 = 0x3e;

pub const EVT_LE_CONN_COMPLETE: u8 = 0x01;
pub const EVT_LE_ADVERTISING_REPORT: u8 = 0x02;
pub const EVT_LE_CONN_UPDATE_COMPLETE: u8 = 0x03;


pub const OGF_LE_CTL: u8 = 0x08;
pub const OCF_LE_SET_EVENT_MASK: u16 = 0x0001;
pub const OCF_LE_SET_SCAN_PARAMETERS: u16 = 0x000b;
pub const OCF_LE_SET_SCAN_ENABLE: u16 = 0x000c;
pub const OCF_LE_CREATE_CONN: u16 = 0x000d;
pub const OCF_LE_CONN_UPDATE: u16 = 0x0013;
pub const OCF_LE_START_ENCRYPTION: u16 = 0x0019;

pub const LE_SET_SCAN_PARAMETERS_CMD: u16 =
    OCF_LE_SET_SCAN_PARAMETERS | (OGF_LE_CTL as u16) << 10;
pub const LE_SET_SCAN_ENABLE_CMD: u16 = OCF_LE_SET_SCAN_ENABLE |
    (OGF_LE_CTL as u16) << 10;
pub const LE_CREATE_CONN_CMD: u16 = OCF_LE_CREATE_CONN | ((OGF_LE_CTL as u16) << 10);

pub const BTPROTO_HCI: i32 = 1;

// #define HCIGETDEVLIST	_IOR('H', 210, int)
pub static HCI_GET_DEV_LIST_MAGIC: usize = (2u32 << 0i32 + 8i32 + 8i32 + 14i32 |
    (b'H' as (i32) << 0i32 + 8i32) as (u32) | (210i32 << 0i32) as (u32)) as
    (usize) | 4 /* (sizeof(i32)) */ << 0i32 + 8i32 + 8i32;
