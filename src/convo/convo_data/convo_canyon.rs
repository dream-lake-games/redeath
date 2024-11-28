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

    (
        ConfrontReaper,
        [
            {
                speaker: ConvoSpeaker::Lenny,
                portrait: "default",
                sound: "TalkingSynth_9",
                text: "Why do you keep running away?",
            },
            {
                speaker: ConvoSpeaker::Reaper,
                portrait: "default",
                sound: "output (1)",
                text: "I could ask you the same thing.",
            },
            {
                speaker: ConvoSpeaker::Lenny,
                portrait: "default",
                sound: "TalkingSynth_3",
                text: "I'm tired of waiting. I'm tired of being scared.",
            },
            {
                speaker: ConvoSpeaker::Reaper,
                portrait: "default",
                sound: "output (2)",
                text: "And what? You think that means you deserve to get into heaven early?",
            },
            {
                speaker: ConvoSpeaker::Lenny,
                portrait: "default",
                sound: "TalkingSynth_2",
                text: "Well, no. I just want to do something. I can't keep feeling sorry for myself.",
            },
            {
                speaker: ConvoSpeaker::Reaper,
                portrait: "default",
                sound: "output (3)",
                text: "Oh child, I don't think you understand how purgatory works.",
            },
            {
                speaker: ConvoSpeaker::Reaper,
                portrait: "default",
                sound: "output (4)",
                text: "You see, you were an okay person. Like most people. Not good, not bad. Just okay.",
            },
            {
                speaker: ConvoSpeaker::Lenny,
                portrait: "default",
                sound: "TalkingSynth_18",
                text: "...thanks.",
            },
            {
                speaker: ConvoSpeaker::Reaper,
                portrait: "default",
                sound: "output (5)",
                text: "We have pity on you, so you're on the proverbial waitlist so to speak.",
            },
            {
                speaker: ConvoSpeaker::Reaper,
                portrait: "default",
                sound: "output (6)",
                text: "But until your day comes you must sit and think about what you did — and more importantly didn't — do.",
            },
            {
                speaker: ConvoSpeaker::Lenny,
                portrait: "default",
                sound: "TalkingSynth_20",
                text: "I'm sorry, I... I can't accept that. Now please, you're in my way.",
            },
            {
                speaker: ConvoSpeaker::Reaper,
                portrait: "default",
                sound: "output (1)",
                text: "In that case, I too am sorry, child. You have a nice death going for you.",
            },
            {
                speaker: ConvoSpeaker::Reaper,
                portrait: "default",
                sound: "output (2)",
                text: "It'd be a shame if something happened to it.",
            },
        ],
    ),
}
