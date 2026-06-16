//! IRC protocol message parsing and serialization.

#![deny(clippy::all)]
#![deny(clippy::pedantic)]
#![deny(unsafe_code)]
#![deny(missing_docs)]

use std::collections::HashMap;
use thiserror::Error;

/// Protocol errors.
#[derive(Debug, Error)]
#[allow(missing_docs)]
pub enum ProtoError {
    #[error("400: ERR_UNKNOWNERROR")]
    UNKNOWNERROR,
    #[error("401: ERR_NOSUCHNICK")]
    NOSUCHNICK,
    #[error("402: ERR_NOSUCHSERVER")]
    NOSUCHSERVER,
    #[error("403: ERR_NOSUCHCHANNEL")]
    NOSUCHCHANNEL,
    #[error("404: ERR_CANNOTSENDTOCHAN")]
    CANNOTSENDTOCHAN,
    #[error("405: ERR_TOOMANYCHANNELS")]
    TOOMANYCHANNELS,
    #[error("406: ERR_WASNOSUCHNICK")]
    WASNOSUCHNICK,
    #[error("409: ERR_NOORIGIN")]
    NOORIGIN,
    #[error("411: ERR_NORECIPIENT")]
    NORECIPIENT,
    #[error("412: ERR_NOTEXTTOSEND")]
    NOTEXTTOSEND,
    #[error("417: ERR_INPUTTOOLONG")]
    INPUTTOOLONG,
    #[error("421: ERR_UNKNOWNCOMMAND")]
    UNKNOWNCOMMAND,
    #[error("422: ERR_NOMOTD")]
    NOMOTD,
    #[error("431: ERR_NONICKNAMEGIVEN")]
    NONICKNAMEGIVEN,
    #[error("432: ERR_ERRONEUSNICKNAME")]
    ERRONEUSNICKNAME,
    #[error("433: ERR_NICKNAMEINUSE")]
    NICKNAMEINUSE,
    #[error("436: ERR_NICKCOLLISION")]
    NICKCOLLISION,
    #[error("441: ERR_USERNOTINCHANNEL")]
    USERNOTINCHANNEL,
    #[error("442: ERR_NOTONCHANNEL")]
    NOTONCHANNEL,
    #[error("443: ERR_USERONCHANNEL")]
    USERONCHANNEL,
    #[error("451: ERR_NOTREGISTERED")]
    NOTREGISTERED,
    #[error("461: ERR_NEEDMOREPARAMS")]
    NEEDMOREPARAMS,
    #[error("462: ERR_ALREADYREGISTERED")]
    ALREADYREGISTERED,
    #[error("464: ERR_PASSWDMISMATCH")]
    PASSWDMISMATCH,
    #[error("465: ERR_YOUREBANNEDCREEP")]
    YOUREBANNEDCREEP,
    #[error("471: ERR_CHANNELISFULL")]
    CHANNELISFULL,
    #[error("472: ERR_UNKNOWNMODE")]
    UNKNOWNMODE,
    #[error("473: ERR_INVITEONLYCHAN")]
    INVITEONLYCHAN,
    #[error("474: ERR_BANNEDFROMCHAN")]
    BANNEDFROMCHAN,
    #[error("475: ERR_BADCHANNELKEY")]
    BADCHANNELKEY,
    #[error("476: ERR_BADCHANMASK")]
    BADCHANMASK,
    #[error("481: ERR_NOPRIVILEGES")]
    NOPRIVILEGES,
    #[error("482: ERR_CHANOPRIVSNEEDED")]
    CHANOPRIVSNEEDED,
    #[error("483: ERR_CANTKILLSERVER")]
    CANTKILLSERVER,
    #[error("491: ERR_NOOPERHOST")]
    NOOPERHOST,
    #[error("501: ERR_UMODEUNKNOWNFLAG")]
    UMODEUNKNOWNFLAG,
    #[error("502: ERR_USERSDONTMATCH")]
    USERSDONTMATCH,
    #[error("524: ERR_HELPNOTFOUND")]
    HELPNOTFOUND,
    #[error("525: ERR_INVALIDKEY")]
    INVALIDKEY,
    #[error("691: ERR_STARTTLS")]
    STARTTLS,
    #[error("696: ERR_INVALIDMODEPARAM")]
    INVALIDMODEPARAM,
    #[error("723: ERR_NOPRIVS")]
    NOPRIVS,
    #[error("902: ERR_NICKLOCKED")]
    NICKLOCKED,
    #[error("904: ERR_SASLFAIL")]
    SASLFAIL,
    #[error("905: ERR_SASLTOOLONG")]
    SASLTOOLONG,
    #[error("906: ERR_SASLABORTED")]
    SASLABORTED,
    #[error("907: ERR_SASLALREADY")]
    SASLALREADY,
}

/// IRC message.
pub struct Message {
    /// Message metadata.
    pub tags: HashMap<String, Option<String>>,
    /// Message origin.
    pub source: Option<String>,
    /// IRC command or numeric.
    pub command: String,
    /// Command arguments.
    pub parameters: Vec<String>,
}

/// Parse the message string into a data model.
///
/// # Errors
///
/// Returns a `ProtoError` in the event that the message
/// could not be parsed.
///
/// # Example
///
/// TODO
pub fn parse_message(message: &str) -> Result<Message, ProtoError> {
    todo!()
}
