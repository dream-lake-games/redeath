use super::decl_convo;

use crate::prelude::*;

decl_convo!(
    ConvoLake,
    (
        Hello,
        [
            (
                ConvoSpeaker::Lenny,
                ConvoEmotion::Default,
                "wow I remade the conversation system...",
            ),
            (ConvoSpeaker::Lenny, ConvoEmotion::Default, "again..."),
        ],
    ),
);
