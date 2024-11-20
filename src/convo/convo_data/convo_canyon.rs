use super::decl_convo;

use crate::prelude::*;

decl_convo! {
    ConvoCanyon,
    (
        Intro,
        [
            {
                speaker: ConvoSpeaker::Friend,
                portrait: "default",
                sound: "TalkingSynth_5",
                text: "You've been coming here a lot lately.",
            },
            {
                speaker: ConvoSpeaker::Friend,
                portrait: "default",
                sound: "TalkingSynth_6",
                text: "Everything alright?",
            },
            {
                speaker: ConvoSpeaker::Lenny,
                portrait: "default",
                sound: "TalkingSynth_2",
                text: "You know, back before I died, I used to live like 30 minutes from here."
            },
            {
                speaker: ConvoSpeaker::Lenny,
                portrait: "default",
                sound: "TalkingSynth_9",
                text: "And my whole life, I only visited once."
            },
            {
                speaker: ConvoSpeaker::Silence(1.5),
                portrait: "default",
                sound: "default",
                text: ""
            },
            {
                speaker: ConvoSpeaker::Lenny,
                portrait: "default",
                sound: "TalkingSynth_18",
                text: "It's beautiful."
            },
            {
                speaker: ConvoSpeaker::Friend,
                portrait: "default",
                sound: "TalkingSynth_4",
                text: "We all have regrets. It's just part of death.",
            },
            {
                speaker: ConvoSpeaker::Friend,
                portrait: "default",
                sound: "TalkingSynth_8",
                text: "Nothing like purgatory to get you thinking about all the things you wish you could've done differently.",
            },
            {
                speaker: ConvoSpeaker::Lenny,
                portrait: "default",
                sound: "TalkingSynth_3",
                text: "I don't know. I feel like I spent my whole life worrying about what I should've done differently."
            },
            {
                speaker: ConvoSpeaker::Lenny,
                portrait: "default",
                sound: "TalkingSynth_20",
                text: "There's got to be more to death than that."
            },
            {
                speaker: ConvoSpeaker::Friend,
                portrait: "default",
                sound: "TalkingSynth_6",
                text: "Maybe some day. Once it's our turn for heaven.",
            },
            {
                speaker: ConvoSpeaker::Lenny,
                portrait: "default",
                sound: "TalkingSynth_19",
                text: "I'm tired of waiting."
            },
            {
                speaker: ConvoSpeaker::Friend,
                portrait: "default",
                sound: "TalkingSynth_5prime",
                text: "So what? You're going to go ask God to let you in early?",
            },
            {
                speaker: ConvoSpeaker::Lenny,
                portrait: "default",
                sound: "TalkingSynth_2",
                text: "Yeah. I mean, I don't know. I'll figure it out."
            },
            {
                speaker: ConvoSpeaker::Lenny,
                portrait: "default",
                sound: "TalkingSynth_9",
                text: "All I know is I'm getting out of here."
            },
        ],
    ),
}
