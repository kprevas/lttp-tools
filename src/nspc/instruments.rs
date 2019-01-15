//0. Unknown (00)
const _UNKNOWN: u8 = 0;
//1. Rain (01)
const RAIN: u8 = 1;
//2. Tympani   (02)
const TIMPANI: u8 = 2;
//3. Square wave (03)
const SQUARE_WAVE: u8 = 3;
//4. Saw wave (04)
const SAW_WAVE: u8 = 4;
//5. Sine wave (05)
const SINE_WAVE: u8 = 5;
//6. Double saw wave 1 (06)
const DOUBLE_SAW_WAVE_1: u8 = 6;
//7. Double saw wave 2 (07)
const DOUBLE_SAW_WAVE_2: u8 = 7;
//8. Tweet (08)
const TWEET: u8 = 8;
//9. Strings (09)
const STRINGS: u8 = 9;
//10. Same as 9 (0A)
//11. Trombone (0B)
const TROMBONE: u8 = 11;
//12. Cymbal (0C)
pub const CYMBAL: u8 = 12;
//13. Ocarina (0D)
const OCARINA: u8 = 13;
//14. Chime (0E)
const CHIME: u8 = 14;
//15. Harp (0F)
const HARP: u8 = 15;
//16. Splash (10)
const SPLASH: u8 = 16;
//17. Trumpet (11)
const TRUMPET: u8 = 17;
//18. Horn (12)
const HORN: u8 = 18;
//19. Snare (13)
pub const SNARE: u8 = 19;
//20. Same as 19 (14)
//21. Choir (15)
const CHOIR: u8 = 21;
//22. Flute (16)
const FLUTE: u8 = 22;
//23. "Oof" (17)
const OOF: u8 = 23;
//24. Guitar (18)  we think this sounds more like a piano
const PIANO: u8 = 24;

pub const INSTRUMENT_MAP: [u8; 128] = [
    PIANO,   //    0 Acoustic Grand Piano
    PIANO,   //    1 Bright Acoustic Piano
    PIANO,   //    2 Electric Grand Piano
    PIANO,   //    3 Honky-tonk Piano
    PIANO,   //    4 Electric Piano 1
    PIANO,   //    5 Electric Piano 2
    PIANO,   //    6 Harpsichord
    PIANO,   //    7 Clavinet
    CHIME,   //    8 Celesta
    CHIME,   //    9 Glockenspiel
    CHIME,   //    10 Music Box
    CHIME,   //    11 Vibraphone
    CHIME,   //    12 Marimba
    CHIME,   //    13 Xylophone
    CHIME,   //    14 Tubular Bells
    CHIME,   //    15 Dulcimer
    CHOIR,   //    16 Drawbar Organ
    CHOIR,   //    17 Percussive Organ
    CHOIR,   //    18 Rock Organ
    CHOIR,   //    19 Church Organ
    CHOIR,   //    20 Reed Organ
    STRINGS, //    21 Accordion
    OCARINA, //    22 Harmonica
    OOF,
    PIANO,             //    24 Acoustic Guitar (nylon)
    PIANO,             //    25 Acoustic Guitar (steel)
    PIANO,             //    26 Electric Guitar (jazz)
    PIANO,             //    27 Electric Guitar (clean)
    PIANO,             //    28 Electric Guitar (muted)
    PIANO,             //    29 Overdriven Guitar
    PIANO,             //    30 Distortion Guitar
    PIANO,             //    31 Guitar Harmonics
    PIANO,             //    32 Acoustic Bass
    PIANO,             //    33 Electric Bass (finger)
    PIANO,             //    34 Electric Bass (pick)
    PIANO,             //    35 Fretless Bass
    PIANO,             //    36 Slap Bass 1
    PIANO,             //    37 Slap Bass 2
    PIANO,             //    38 Synth Bass 1
    PIANO,             //    39 Synth Bass 2
    STRINGS,           //    40 Violin
    STRINGS,           //    41 Viola
    STRINGS,           //    42 Cello
    STRINGS,           //    43 Contrabass
    STRINGS,           //    44 Tremolo Strings
    STRINGS,           //    45 Pizzicato Strings
    HARP,              //    46 Orchestral Harp
    TIMPANI,           //    47 Timpani
    STRINGS,           //    48 String Ensemble 1
    STRINGS,           //    49 String Ensemble 2
    STRINGS,           //    50 Synth Strings 1
    STRINGS,           //    51 Synth Strings 2
    CHOIR,             //    52 Choir Aahs
    CHOIR,             //    53 Voice Oohs
    CHOIR,             //    54 Synth Choir
    STRINGS,           //    55 Orchestra Hit
    TRUMPET,           //    56 Trumpet
    TROMBONE,          //    57 Trombone
    HORN,              //    58 Tuba
    TRUMPET,           //    59 Muted Trumpet
    HORN,              //    60 French Horn
    HORN,              //    61 Brass Section
    HORN,              //    62 Synth Brass 1
    HORN,              //    63 Synth Brass 2
    TRUMPET,           //    64 Soprano Sax
    TRUMPET,           //    65 Alto Sax
    TROMBONE,          //    66 Tenor Sax
    TROMBONE,          //    67 Baritone Sax
    HORN,              //    68 Oboe
    HORN,              //    69 English Horn
    HORN,              //    70 Bassoon
    FLUTE,             //    71 Clarinet
    FLUTE,             //    72 Piccolo
    FLUTE,             //    73 Flute
    OCARINA,           //    74 Recorder
    OCARINA,           //    75 Pan Flute
    OCARINA,           //    76 Blown bottle
    OCARINA,           //    77 Shakuhachi
    TWEET,             //    78 Whistle
    OCARINA,           //    79 Ocarina
    SQUARE_WAVE,       //    80 Lead 1 (square)
    SAW_WAVE,          //    81 Lead 2 (sawtooth)
    OCARINA,           //    82 Lead 3 (calliope)
    DOUBLE_SAW_WAVE_1, //    83 Lead 4 (chiff)
    DOUBLE_SAW_WAVE_1, //    84 Lead 5 (charang)
    DOUBLE_SAW_WAVE_1, //    85 Lead 6 (voice)
    DOUBLE_SAW_WAVE_2, //    86 Lead 7 (fifths)
    DOUBLE_SAW_WAVE_2, //    87 Lead 8 (bass + lead)
    SINE_WAVE,         //    88 Pad 1 (new age)
    SINE_WAVE,         //    89 Pad 2 (warm)
    SINE_WAVE,         //    90 Pad 3 (polysynth)
    CHOIR,             //    91 Pad 4 (choir)
    SINE_WAVE,         //    92 Pad 5 (bowed)
    SINE_WAVE,         //    93 Pad 6 (metallic)
    SINE_WAVE,         //    94 Pad 7 (halo)
    SINE_WAVE,         //    95 Pad 8 (sweep)
    RAIN,              //    96 FX 1 (rain)
    RAIN,              //    97 FX 2 (soundtrack)
    RAIN,              //    98 FX 3 (crystal)
    RAIN,              //    99 FX 4 (atmosphere)
    RAIN,              //    100 FX 5 (brightness)
    RAIN,              //    101 FX 6 (goblins)
    RAIN,              //    102 FX 7 (echoes)
    RAIN,              //    103 FX 8 (sci-fi)
    PIANO,             //    104 Sitar
    PIANO,             //    105 Banjo
    PIANO,             //    106 Shamisen
    PIANO,             //    107 Koto
    PIANO,             //    108 Kalimba
    HORN,              //    109 Bagpipe
    STRINGS,           //    110 Fiddle
    HORN,              //    111 Shanai
    CHIME,             //    112 Tinkle Bell
    CHIME,             //    113 Agogo
    TIMPANI,           //    114 Steel Drums
    SNARE,             //    115 Woodblock
    TIMPANI,           //    116 Taiko Drum
    TIMPANI,           //    117 Melodic Tom
    TIMPANI,           //    118 Synth Drum
    CYMBAL,            //    119 Reverse Cymbal
    OOF,               //    120 Guitar Fret Noise
    OOF,               //    121 Breath Noise
    SPLASH,            //    122 Seashore
    TWEET,             //    123 Bird Tweet
    CHIME,             //    124 Telephone Ring
    OOF,               //    125 Helicopter
    RAIN,              //    126 Applause
    OOF,               //    127 Gunshot
];
