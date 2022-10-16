mod callback;
mod checks;
mod ibc_msg;

use cosmwasm_std::IbcOrder;

pub use crate::callback::ReceiveIcaResponseMsg;
pub use crate::checks::{check_order, check_version, SimpleIcaError};
pub use crate::ibc_msg::{
    BalancesResponse, DispatchResponse, IbcQueryResponse, PacketMsg, StdAck, WhoAmIResponse,
};

pub const IBC_APP_VERSION: &str = "ica-host-v1";
pub const APP_ORDER: IbcOrder = IbcOrder::Unordered;
pub const BAD_APP_ORDER: IbcOrder = IbcOrder::Ordered;