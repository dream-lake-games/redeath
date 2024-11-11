use super::decl_convo;

use crate::prelude::*;

decl_convo!(
    ConvoCanyon,
    (
        Hello,
        [
            (
                ConvoSpeaker::Friend,
                ConvoEmotion::Default,
                "You've been coming here a lot lately.",
            ),
            (
                ConvoSpeaker::Friend,
                ConvoEmotion::Default,
                "Everything alright?",
            ),
            (
                ConvoSpeaker::Lenny,
                ConvoEmotion::Default,
                "You know, back before I died, I used to live like 30 minutes from here."
            ),
            (
                ConvoSpeaker::Lenny,
                ConvoEmotion::Default,
                "And my whole life, I only visited once."
            ),
            (ConvoSpeaker::Silence(1.5), ConvoEmotion::Default, ""),
            (
                ConvoSpeaker::Lenny,
                ConvoEmotion::Default,
                "It's beautiful."
            ),
            (
                ConvoSpeaker::Friend,
                ConvoEmotion::Default,
                "We all have regrets. It's just part of death.",
            ),
            (
                ConvoSpeaker::Friend,
                ConvoEmotion::Default,
                "Nothing like purgatory to get you thinking about all the things you wish you could've done differently.",
            ),
            (
                ConvoSpeaker::Lenny,
                ConvoEmotion::Default,
                "I don't know. I feel like I spent my whole life worrying about what I should've done differently."
            ),
            (
                ConvoSpeaker::Lenny,
                ConvoEmotion::Default,
                "There's got to be more to death than that."
            ),
            (
                ConvoSpeaker::Friend,
                ConvoEmotion::Default,
                "Maybe some day. Once it's our turn for heaven.",
            ),
            (
                ConvoSpeaker::Lenny,
                ConvoEmotion::Default,
                "I'm tired of waiting."
            ),
            (
                ConvoSpeaker::Friend,
                ConvoEmotion::Default,
                "So what? You're going to go ask God to let you in early?",
            ),
            (
                ConvoSpeaker::Lenny,
                ConvoEmotion::Default,
                "Yeah. I mean, I don't know. I'll figure it out."
            ),
            (
                ConvoSpeaker::Lenny,
                ConvoEmotion::Default,
                "All I know is I'm getting out of here."
            ),
        ],
    ),
);
