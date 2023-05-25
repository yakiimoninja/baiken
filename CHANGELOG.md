# Version 0.13.0 Patch Notes.

### General
- Updated framedata.

### Nicknames
- `Asuka R`:
    - Added nicknames `Asuka`, `That Man`, `Oscar`.

### Fixes
- Fixed and issue where using `/aliases` for Jacko wouldn't display anything.

&#x200B;
&#x200B;
&#x200B;
&#x200B;

# Version 0.12.0 Patch Notes.

### General
- Optimized alias searching loops to break when alias is found (i had forgor 💀).

### Fixes
- Fixed an issue where `/hitboxes` would not display moves when using aliases that had a `.` in them.
- Fixed Bedman?'s aliases file move input: 
    - From: `13C (214H) 236P` to: `13C (236H) 236P`.
    - From: `13C (214P) 236P` to: `13C (236P) 236P`.
    - From: `13C (214S) 236P` to: `13C (236S) 236P`.

&#x200B;
&#x200B;
&#x200B;
&#x200B;

# Version 0.11.0 Patch Notes.

### General
- Updated framedata.

### Fixes
- Fixed an issue where `Bedman` was displaying different moves than what user request. \
    e.g. `/fmeter bedman 214P` displaying info for `(236H) 214P`. \
    Bedman's specials broke bot's program logic (error 6E lmao).

- Fixed and issue where `/fmeter` wasn't displaying the right amount of startup frames inside `[ brackets ]`.

### Move Aliases
- **`Bedman?`**:
    - `!H` added aliases `error 6E H`, `6EH`, `EH`.
    - `!{H}` added aliases `Partial error 6E {H}`, `6E!H`, `P6EH`, `E!H`, `PEH`.
    - `![H]` added aliases `Charged error 6E [H]`, `6E!!H`, `C6EH`, `E!!H`, `CEH`.
    - `!P` added aliases `error 6E P`, `6EP`, `EP`.
    - `!S` added aliases `error 6E S`, `6ES`, `ES`.
    - `(236H) 214P` added aliases `call 0x$0.20 236H`, `Instant H`, `IH`, `C2H`.
    - `(236P) 214P` added aliases `call 0x$0.20 236P`, `Instant P`, `IP`, `C2P`.
    - `(236S) 214P` added aliases `call 0x$0.20 236S`, `Instant S`, `IS`, `C2S`.
    - `13C !H` added aliases `13C error 6E H`, `13C6EH`, `13CEH`.
    - `13C !P` added aliases `13C error 6E P`, `13C6EP`, `13CEP`.
    - `13C !S` added aliases `13C error 6E S`, `13C6ES`, `13CES`.
    - `13C (214H) 236P` added aliases `13C call 0x$0.20 H`, `13CC2H`.
    - `13C (214P) 236P` added aliases `13C call 0x$0.20 P`, `13CC2P`.
    - `13C (214S) 236P` added aliases `13C call 0x$0.20 S`, `13CC2S`.
    - `214K` added aliases `call 0x$1.00`, `C1`, `Delay`.
    - `214P` added aliases `call 0x$0.20`, `C2`, `Instant`.
    - `236P` added aliases `call 4BA`, `4BA`, `Fireball`, `FB`.
    - `236S` added aliases `call 4B3`, `4B3`, `Beyblade`, `BB`.
    - `236H` added aliases `call 4B9 (Malfunction)`, `4B9`, `Arm Swing`, `Swing`, `AS`, `236H`.
    - `236{H}` added aliases `Partial call 4B9 (Malfunction)`, `P4B9`, `Partial Arm Swing`, `Partial Swing`, `PAS`, `236!H`
    - `236[H]` added aliases `Charged call 4B9 (Malfunction)`, `C4B9`, `Charged Arm Swing`, `Charged Swing`, `CAS`, `236!!H`.

- **`Nagoriyuki`**:
	- `j.S Level 1` added alias `j.S`.
	- `j.S Level 2` added alias`j.S2`.
	- `j.S Level 3` added alias`j.S3`.
	- `j.S Level BR` added alias`j.SB`.

&#x200B;
&#x200B;
&#x200B;
&#x200B;

# Version 0.10.6 Patch Notes.

### General
- Updated framedata.

### Fixes
- Fixed dots in `nicknames` command.
- Fixed some typos.

### Move Aliases
- **`Anji`**:
    - `236P` added alias `Bird`.

### Nicknames
- `Jack-O`:
    - Added nickname `Jack-O Valentine`.
- `Bedman?`:
    - Added nickname `Delilah`.

&#x200B;
&#x200B;
&#x200B;
&#x200B;

# Version 0.10.5 Patch Notes.

### Fixes
- Fixed `Bedman` not displaying stuff properly.

&#x200B;
&#x200B;
&#x200B;
&#x200B;

# Version 0.10.4 Patch Notes.

### General
- Updated framedata.
- Added character Bedman?.

### Nicknames
- **`Bedman?`**:
    - Added nicknames `Bedwoman`, `Bed`.

### Move Aliases
- **`Bedman?`**:
    - `(236H) 214P`added alias `214PH`.
    - `(236P) 214P` added alias `214PP`.
    - `(236S) 214P` added alias `214PS`.
    - `214K` added aliases `call 0x$1.00`, `C1`, `Delay`.
    - `214P` added aliases `call 0x$0.20`, `C2`, `Instant`.
    - `236H` added aliases `call 4B9 (Malfunction)`,`4B9`,`Arm Swing`,`Swing`,`AS`.
    - `236P` added aliases `call 4BA`,`4BA`,`Fireball`,`FB`.
    - `236S` added aliases `call 4B3`,`4B3`,`Beyblade`,`BB`.
    - `j.214K`added aliases `Air call 0x$1.00`, `Air C1`, `Air Delay`, `AC1`, `AD`.
    - `j.214P`added aliases `Air call 0x$0.20`, `Air C2`, `Air Instant`, `AC2`, `AI`.
    - `j.236P` added aliases `Air call 4BA`,`Air 4BA`,`A4BA`,`Air Fireball`,`Air FB`,`AFB`.
    - `j.236S` added aliases `Air call 4B3`,`Air 4B3`,`A4B3`,`Air Beyblade`,`Air BB`,`ABB`.
    - `632146H` added aliases `call 4CC`,`4CC`,`Super`.
    - `632146S` added aliases `call 13C`,`13C`,`Install`.

### Fixes
- **`Leo Whitefang`**:
    - `632146S` replaced aliases `Fan Super` and `FS` with `Projectile Super` and `PS`. \
    Due to overlap couldn't access data for `fS` before this change.

&#x200B;
&#x200B;
&#x200B;
&#x200B;

# Version 0.10.3 Patch Notes.

### General
- Updated framedata. 

### Move Aliases
- **`Chipp Zanuff`**:
    - `236S` added alias `Rekka`.

- **`Ramlethal Valentine`**:
    - `236236S` added alias `Motorboat`.

&#x200B;
&#x200B;
&#x200B;
&#x200B;


# Version 0.10.2 Patch Notes.

### Fixes
- Fixed `the application did not respond` error when using the `/moves` command for Nagoriyuki.

### General
- Refactored code to exhaustively search through the character nicknames first.
- Updated framedata. 

&#x200B;
&#x200B;
&#x200B;
&#x200B;

# Version 0.10.1 Patch Notes.

### Fixes
- Fixed missing moves for `Nagoriyuki` and `Ky`.

### General
- Updated framedata.

&#x200B;
&#x200B;
&#x200B;
&#x200B;

# Version 0.10.0 Patch Notes.

### General
- Added new command `fmeter` that displays visually the startup, active and recovery frames of a character's move.
- Reworked the command `help` to display specific help messages if a command name, `notes` or `specifics` is given in `cmd_arg` option.
- Updated the github README. Now has pictures to help explain each command.
- Updated framedata.

&#x200B;
&#x200B;
&#x200B;
&#x200B;

# Version 0.9.0 Patch Notes.

### General
- Added new `Attack Level`, `Scaling` and `Risc Gain` information fields for the `frames` commands.
- Added new `Attack Level`, `Scaling` and `Risc Gain` information fields for the character json files.
- Completely reworked how framedata is being parsed from dustloop which cut the `update` time to less than half.
- Added auto parsing of images and hitboxes instead of having to grab them manually.
- Reworked the `update` command to update frames and or data for all or specific characters.
- Updated frame data.

### Nicknames
- **`Ramlethal Valentine`**:
    - Added nicknames `Borgar`.
  
- **`Happy Chaos`**:
    - Added nicknames `Asshole`.

### Move Aliases
- **`Potemkin`**:
    - `632146P` added alias `Hug`.

- **`Potemkin`**:
    - `632146P` added alias `Super Hug`.
  
- **`Happy Chaos`**:
    - `632146P` added alias `Balls`.

&#x200B;
&#x200B;
&#x200B;
&#x200B;

# Version 0.8.5 Patch Notes.

### General
- Updated frame data.

### Move Aliases
- **`All Characters`**:
    - `2D` added alias `Sweep`.
 
- **`Sin Kiske`**:
    - `632S` added alias `DP`.
    - `632SS` added aliases `DP Followup`, `DPF`.

&#x200B;
&#x200B;
&#x200B;
&#x200B;

# Version 0.8.4 Patch Notes.

### Fixes
- Fixed `aliases` command spitting `The application did not respond` error.
- Fixed `moves` command not showing anything for `Goldlewis`.

&#x200B;
&#x200B;
&#x200B;
&#x200B;

# Version 0.8.3 Patch Notes.

### General
- Updated frame data.

### Move Aliases
- **`Nagoriyuki`**:
    - `f.S Level 1` added alias `fs`.

&#x200B;
&#x200B;
&#x200B;
&#x200B;

# Version 0.8.2 Patch Notes.

### General
- Updated frame data.
- Updated command usage instructions.

### Nicknames
- **`Axl Low`**:
    - Added nicknames `Brit`,`British`.
 
- **`Zato-1`**:
    - Added nickname `Eddie`.

### Move Aliases
- **`Zato-1`**:
    - `236K` added alias `Clap`.
    - `]K[` added aliases `NE Clap`, `NEC`.

&#x200B;
&#x200B;
&#x200B;
&#x200B;

# Version 0.8.1 Patch Notes.

### General
- Added `Sin Kiske` along with his framedata, hitboxes, aliases.

### Nicknames
- **`Sin_Kiske`**:
    - Added nickname `SK`.

### Move Aliases
- **`Sol Badguy`**:
    - `236P` added alias `Gunflame`.
    - `214P` added alias `Feint Gunflame`.

- **`Sin Kiske`**:
    - `c.S` added alias `5S`.
    - `5[D]` added alias `5D!`.
    - `236H` added alias `Beak Driver`, `BD`.
    - `236HH` added alias `Beak Driver Followup`, `BDF`.
    - `623S` added alias `Hawk Baker`, `HB`.
    - `623SS` added alias `Hawk Baker Followup`, `HBF`.
    - `214S` added alias `Hoof Stomp`, `HS`.
    - `214SS` added alias `Hoof Stomp Followup`, `HSF`.
    - `236K` added alias `Elk Hunt`, `EH`.
    - `236KK` added alias `Elk Hunt Followup`, `EHF`.
    - `66` added alias `Gazelle Step`, `GS`.
    - `63214P` added alias `Still Growing`, `Eat`, `Food`, `SG`.
    - `632146H` added alias `R.T.L.`, `RTL`.
    - `632146HH` added alias `R.T.L. Followup`, `RTLF`.
    - `236236P` added alias `Tyrant Barrel`, `Tyrant Rave`, `TB`, `TR`.
    - `236236PP` added alias `Tyrant Barrel Followup`, `Tyrant Rave Followup`, `TBF`, `TRF`.

&#x200B;
&#x200B;
&#x200B;
&#x200B;

# Version 0.8.0 Patch Notes.

### General
- Added new command `nicknames` to print all existing nicknames for characters.

### Nicknames

- Removed all 2 letter nicknames from file for characters without 2 part names, as redundant. Bot can find characters as is.
- **`Zato-1`**:
    - Added nickname `Zato One`.

&#x200B;
&#x200B;
&#x200B;
&#x200B;

# Version 0.7.6 Patch Notes.
 
### Fixes 
- Fixed `Faust 5K` broken hitbox image link.
- Fixed `Nagoriyuki` aliases command not displaying anything.

&#x200B;
&#x200B;
&#x200B;
&#x200B;

# Version 0.7.5 Patch Notes.

### General
- Code clean up. Removed the use of the `init.json` file.

### Nicknames
- Added the full name as a nickname for all characters applicable.
- **`Sol Badguy`**:
    - Added nickname `Sol Goodman`.
   
### Move Aliases

- **`All Characters`**:
    - `c.S` added alias `5S`.

- **`Bridget`**:
    - `SS` added alias `5SS`.
    - `HH` added alias `5HH`.
    - `236KP` modified alias from `SB` to `SLB` due to alias overlap.
    - `236KK` modified alias from `SS` to `SLS` due to alias overlap.

- **`Chipp Zanuff`**
    - `236P` added alias `P Alpha`, `PA`.
    - `j.236P` added alias `Air P Alpha`, `APA`.
    - `236K` added alias `K Alpha`, `KA`.
    - `j.236K` added alias `Air K Alpha`, `AKA`.
    - `623S` added alias `Beta`.
    - `j.623S` added alias `Air Beta`, `AB`.
    - `236H` added alias `Gamma`.

&#x200B;
&#x200B;
&#x200B;
&#x200B;

# Version 0.7.4 Patch Notes.

### General
- Updated frame data.

### Fixes 
- Fixed `The application did not respond` error when invoking the `/update` command.
- Fixed `invincibility` and `counter` showing `RISC` and `Proration` values.
- Fixed `Sol` `Air Volcanic Viper` alias displaying framedata for `H Volcanic Viper` and vice versa.

### Move Aliases
- **`Axl Low`**:
    - `j.236H` Added alias: `TKB`.

- **`Baiken`**:
    - `j.214214P` Added alias: `Air Kenjyu`.

- **`Faust`**:
    - `214P` Added alias: `P Scarecrow`.
    - `214K` Added alias: `K Scarecrow`.
    - `214S` Added alias: `S Scarecrow`.

- **`I-no`**:
    - `j.236K` Added alias: `K Sultry Performance`.
    - `236H` Added alias: `H Stroke The Big Tree`.

- **`Ky Kiske`**:
    - `DI 236K` Added alias: `DI Stun Dipper`.
    - `DI 214K` Added alias: `DI Foudre Arc`.
    - `DI 214S` Added alias: `DI Dire Eclat`.
    - `DI 236236P` Added alias: `DI Sacred Edge`.

- **`Leo Whitefang`**:
    - `bt.P` Added alias: `Bt Punch`,`BTP`.
    - `bt.K` Added alias: `Bt Low`, `BTK`, `BK`.
    - `bt.S` Added alias: `Bt Cross`, `BTS`.
    - `bt.H` Added alias: `Bt Overhead`, `BTH`, `BH`.
    - `bt.D` Added alias: `Back Turn Dust`, `Backturn Dust`, `Back Turn Parry`, `Bt Parry`.
    - `bt.22` Added alias: `Bt Cancel`.
    - `bt.214K` Added alias: `Bt Command Grab`,`Bt Grab`,`Bt Throw`,`BTCG`,`BTG`,`BTT`. 
    - `bt.214H` Added alias: `Bt Blitz`.
    - `bt.214K` Removed alias `BTP` as duplicate.

- **`May`**:
    - `[4]6S` Added alias: `S Dolphin`.
    - `[2]8S` Added alias: `S Up Dolphin`.


- **`Millia Rage`**:
    - `236S` Added alias: `S Disk`.
    - `236H` Added alias: `H Disk`.

- **`Nagoriyuki`**:
    - `623H` Added alias: `Shizuriyuki 1`, `DP1`.
    - `623HH` Added alias: `Shizuriyuki 2`, `DP2`.

- **`Potemkin`**:
    - `Reflect Projectile` Added alias: `RP`.

- **`Ramlethal Valentine`**:
    - `236S` Added alias: `S Sword Throw`.

- **`Sol Badguy`**:
    - `214P` Added alias: `Feint Fireball`.
    - `623S` Added alias: `S Volcanic Viper`.
    - `236K` Added alias: `Bandit Revolver 1`, `BR1`.
    - `236KK` Added alias: `Bandit Revolver 2`, `BR2`.
    - `j.236K` Added alias: `Air Bandit Revolver`, `Air Bandit Revolver 1`, `JBR1`, `ABR`, `ABR1`.
    - `j.236KK` Added alias: `Air Bandit Revolver`, `Air Bandit Revolver 1`, `JBR1`, `ABR`, `ABR1`.

&#x200B;
&#x200B;
&#x200B;
&#x200B;

# Version 0.7.3 Patch Notes.

### General
- Cleaned up bot message formatting.

### Nicknames.
>add a nickname for may called "totsugeki" so we can search up totsugeki totsugeki
 
Say less my guy.

- `May` Added nickname: `Totsugeki`.

### Move Aliases
- **`Nagoriyuki`**:
    - `j.D Level 1` Added alias: `j.D`.
    - `j.D Level 2` Added alias: `j.D2`.
    - `j.D Level 3` Added alias: `j.D3`.
    - `j.D Level BR` Added alias: `j.DB`.

&#x200B;
&#x200B;
&#x200B;
&#x200B;

# Version 0.7.2 Patch Notes.

### General
- Cleaned up some code.
- Reworked error messages when moves are not found.

### Move Aliases
- **`Millia Rage`**:
    - `j.236K` Added alias: `Turbofall`.

&#x200B;
&#x200B;
&#x200B;
&#x200B;

# Version 0.7.1 Patch Notes.

### General
- Changed move not found case for `/frames` command.
- Changed follow up message for `/aliases` command.
 
### Move Aliases
- **`Chipp Zanuff`**:
    - `63214S` Added alias: `Leaf Grab`.

### Nicknames
- `Goldlewis` Added nickname: `GL`.

&#x200B;
&#x200B;
&#x200B;
&#x200B;

# Version 0.7.0 Patch Notes.

### General
- Prefix commands will be replaced with [slash commands](https://github.com/yakiimoninja/baiken#commands) from `19/September/2022` due to changes in Discord's API.
- Refactored code to support slash commands and the new gateway intents.
- Slash commands have context menus to help with command execution.
- Added a new easter egg and removed the previous one.
- Reworked how the `request` command works.
- New invite link and QR Code.

### Nicknames
- `Baiken` Added nickname: `Bacon`.
- `Bridget` Added nickname: `Brisket`.
 
### Move Aliases
- **`Bridget`**:
    - `236S/H` Added aliases: `Stop and Dash`, `Fireball`, `Yoyo Throw`, `SD`, `YYT`.
    - `214S/H` Added aliases: `Stop and Dash Return`, `Return Fireball`, `Yoyo Return`, `SDR`, `YYR`.
    - `214K` Added aliases: `Rolling Movement`, `Ball`, `Spin`, `Sonic`, `Sonic Spin`, `RM`.
    - `623P` Added aliases: `Starship`, `DP`.
    - `236K` Added aliases: `Kick Start My Heart`, `Slide`, `KSMH`.
    - `236K P` Added aliases: `Brake`, `Slide Brake`, `SB`.
    - `236K K` Added aliases: `Shoot`, `Slide Shoot`, `SS`.
    - `j.236K`  Added aliases: `Roger Dive`, `Dive Kick`, `RD`, `DK`.
    - `63214P` Added aliases: `Rock the Baby`, `Command Grab`, `Grab`, `RTB`, `CG`.
    - `632146S` Added aliases: `Loop the Loop`, `Teddy Super`, `Roger Super`, `TS`, `RS`, `LTL`.
    - `632146H` Added aliases: `Return of the Killing Machine`, `Fire Teddy Super`, `Fire Roger Super`, `FRS`, `FTS`, `ROTKM`.

### Fixes
- Fixed `Bridget` missing images.
- Fixed `Faust` `c.S` displaying super instead of `c.S` due to overlap of aliases.

&#x200B;
&#x200B;
&#x200B;
&#x200B;

 # Version 0.6.4 Patch Notes.

### General
- Added Bridgets framedata/hitboxes.
- Updated some existing framedata.

&#x200B;
&#x200B;
&#x200B;
&#x200B;

 # Version 0.6.3 Patch Notes.

### General
- Updated some of the frame data.

### Fixes
- Fixed Chipp `Banki Messai` not displaying cause of changed input.

&#x200B;
&#x200B;
&#x200B;
&#x200B;

# Version 0.6.2 Patch Notes.

### General
- Updated some of the frame data.

&#x200B;
&#x200B;
&#x200B;
&#x200B;

# Version 0.6.1 Patch Notes.

### Fixes
- Fixes Testament's `Greave Reapers` not displaying anything cause of dustloop updating them. 
- Fixed missing move images and hitboxes for:
    - `Chipp Wall Run P`.
    - `Chipp Wall Run K`.
    - `Chipp Wall Run S`.
    - `Chipp Wall Run SS`.
    - `Chipp Wall Run H`.
    - `Chipp Wall Run 6H`.

&#x200B;
&#x200B;
&#x200B;
&#x200B;

# Version 0.6.0 Patch Notes.

### General
- Cleaned up some of the code regarding `init.json`.
- Added additional prefix for Baiken. Now both **`b.`** and **`!`** work as preFixes
- Made all commands case insensitive. Now `b.f` and `B.F` work.
- Added some special interactions when `b.f` is used while other frame data bots are present on the same server.
- Updated [README.md](https://github.com/yakiimoninja/baiken#readme) with character specific stuff and minor adjustments.

### Move aliases

- **`Nagoriyuki`**:
    - `f.S Level 1`: Added alias `f.S`.
    - `f.S Level 2`: Added alias `f.S2`.
    - `f.S Level 3`: Added alias `f.S3`.
    - `f.S Level BR`: Added alias `f.SB`.

    - `f.SS Level 1`: Added alias `f.SS`.
    - `f.SS Level 2`: Added alias `f.SS2`.
    - `f.SS Level 3`: Added alias `f.SS3`.
    - `f.SS Level BR`: Added alias `f.SSB`.

    - `f.SSS Level 1`: Added alias `f.SSS`.
    - `f.SSS Level 2`: Added alias `f.SSS2`.
    - `f.SSS Level 3`: Added alias `f.SSS3`.
    - `f.SSS Level BR`: Added alias `f.SSSB`.

    - `2S Level 1`: Added alias `2S`.
    - `2S Level 2`: Added alias `2S2`.
    - `2S Level 3`: Added alias `2S3`.
    - `2S Level BR`: Added alias `2SB`.

### Fixes

- Fixed missing move image and hitboxes for:
    - `nago fS Level 1`.
    - `nago fS Level 2`.
    - `nago fS Level 3`.
    - `nago fS Level BR`.
    - `nago fSS Level 1`.
    - `nago fSS Level 2`.
    - `nago fSS Level 3`.
    - `nago fSS Level BR`.
    - `nago fSSS Level 1`.
    - `nago fSSS Level 2`.
    - `nago fSSS Level 3`.
    - `nago fSSS Level BR`.
    - `nago 2S Level 1`.
    - `nago 2S Level 2`.
    - `nago 2S Level 3`.
    - `nago 2S Level BR`.

&#x200B;
&#x200B;
&#x200B;
&#x200B;

# Version 0.5.0 Patch Notes.

### General
- Season 2 is out but the new frame data will have to be updated gradually. \
 Meaning we'll need to wait for the chads at dustloop to bless us. \
 Support them if you can. They are doing gods work!
 
- Removed patch notes folder but moved all the patch notes to the [releases](https://github.com/yakiimoninja/baiken/releases).

### Commands
- Removed the command aliases `hit` and `hitbox` from `b.h`. \
 Although `b.h` and `b.hitboxes` still work.

### Move Aliases
- `Happy Chaos`:
    - `Focus 214S H`: Removed alias `Fire` as it's a duplicate of the `Regular 236S H`. \
    `Focus Fire` can be used instead.

- `Nagoriyuki`: \
   Since Nagoriyuki has a lot variation in his normals due to Blood Rage, aliases have now been added to distinguish them from each other.
 
    - `5H Level 1`: Added alias `5H`.
    - `5H Level 2`: Added alias `5H2`.
    - `5H Level 3`: Added alias `5H3`.
    - `5H Level BR`: Added alias `5HB`.

    - `2H Level 1`: Added alias `2H`.
    - `2H Level 2`: Added alias `2H2`.
    - `2H Level 3`: Added alias `2H3`.
    - `2H Level BR`: Added alias `2HB`.

    - `6H Level 1`: Added alias `6H`.
    - `6H Level 2`: Added alias `6H2`.
    - `6H Level 3`: Added alias `6H3`.
    - `6H Level BR`: Added alias `6HB`.

    - `j.H Level 1`: Added alias `jH`.
    - `j.H Level 2`: Added alias `jH2`.
    - `j.H Level 3`: Added alias `jH3`.
    - `j.H Level BR`: Added alias `jHB`.

- `Goldlewis Dickinson`: \
   Same principle with Nagoriyuki adding more aliases to distinguish between specials and their leveled up counterparts.

    - `214S Level 1`: Added alias `D1`.
    - `214S Level 2`: Added alias `Thunderbird 2`, `Drone 2`,`D2`. 
    - `214S Level 3`: Added alias `Thunderbird 3`, `Drone 3`,`D3`.

    - `236S Level 1`: Added alias `Minigun`, `MG1`, `G1`.
    - `236S Level 2`: Added alias `Skyfish 2`, `Minigun 2`, `MG2`, `Gun 2`, `G2`.
        - Existing wrong alias `GL1` changed to `GL2`.
    - `236S Level 3`: Added alias `Skyfish 3`, `Minigun 3`, `MG3`, `Gun 3`, `G3`.

    - `720P`: Added alias `Down With The System 2`, `Super 2`.
    - `1080P`: Added alias `Down With The System 3`, `Super 3`.

    - `236236K Level 1`: Added alias `L1`.
    - `236236K Level 2`: Added alias `Burn It Down 2`, `Laser 2`, `Laser Super 2`, `L2`.
    - `236236K Level 3`: Added alias `Burn It Down 3`, `Laser 3`, `Laser Super 3`, `L3`.


### Fixes 
- Fixed `b.f` broken move image links for:
    - `nago 623H`.
    - `nago 623HH`.

&#x200B;
&#x200B;
&#x200B;
&#x200B;

# Version 0.4.5 Patch Notes.

### Added the option to invite Baiken to your server using the following QR Code. 

<img src="../data/images/baiken_qr.png" width="200" height="200"/>

### Miscellaneous
- Changed the embed line's color to match Baiken's profile picture. 
- Moved `images` folder out of `src` and to `data`.

### Fixes
- Fixed broken image links after the `images` folder move.

&#x200B;
&#x200B;
&#x200B;
&#x200B;

# Version 0.4.4 Patch Notes.

### Move Aliases

- `Leo Whitefang`:
    - Autoguard attack `[s/h] h/s`: Added aliases `s!h` and `h!s`.

- `Faust`:
    - `236S`: Added alias `MMM`.
    - `j236S`: Added aliases `AMMM` and `JMMM`.

### Fixes

- Fixed `b.h` missing hitbox images for:
    - `axl 2P`
    - `axl 2S`
    - `axl 2H`
    - `axl jD`
    - `chipp 236236K`
    - `giovanna 2D`
    - `hc fS`
    - `jacko fS`
    - `jacko 2H`
    - `jacko jS`
    - `jacko jH`
    - `jacko 2D`
    - `jacko 236K`
    - `ky 236K`
    - `may 632146H`
    - `may j632146H`
    - `millia 6P`
    - `potemkin cS`

&#x200B;
&#x200B;
&#x200B;
&#x200B;

# Version 0.4.3 Patch Notes.

### Fixes
- Fixed `b.a` not printing anything for Ky.
- Fixed `b.a` command not coloring properly.

### Miscellaneous
- Changed message of `b.f` command when the move given by the user is incorrect and advizing them to look at the moves and aliases.


### Sidenote:
- Refrain from requesting aliases that already exist or are similar to what already exists.
- Read the instructions on how to request before doing so.

&#x200B;
&#x200B;

TL;DR Stop being lazy pierrots and read instructions.

&#x200B;
&#x200B;
&#x200B;
&#x200B;

# Version 0.4.2 Patch Notes.


### Nicknames

- `Sol Badguy`: 
    - Added nickname `Helios` (you can finally stop requesting it lol). 

### Move Aliases

- `I-no`:
    - `236H`: Added alias `H Stroke`.

### Miscellaneous

- Renamed `patch_notes` folder to `patch-notes`.

&#x200B;
&#x200B;
&#x200B;
&#x200B;

# Version 0.4.1 Patch Notes.

### Changes
- Changed `b.?` / `b.help` message.

### Move Aliases
- Universal `throw`: Added aliases `6D` and `4D`.
- May `214P`: Added alias `Beachball`.
- May `214K`: Added alias `K Beachball`.
- May `623K`: Added alias `OHK`.

- Testament `236*` : Changed version indicator in aliases.
    * For partial charge, changed from `@` to `!`.
    * For full charge, changed from `!` to `!!`.

    * Example (before): `b.f testament 236S@` or `b.f testament 236H!`.
    * Example (after): `b.f testament 236S!` or `b.f testament 236H!!`.

### Fixes
- Fixed an issue that did not allow to use `6D` or `4D` to see the throw data.

### Miscellaneous
- Removed some unnecessary data from all the characters' json files.

&#x200B;
&#x200B;
&#x200B;
&#x200B;

# Version 0.4.0 Patch Notes.

&#x200B;

### Added support for nickname searching.

Previously you could only search by using a characters full name or with a part of it.

* Example before update: `b.f happy_chaos` or `b.f happy` or `b.f chaos`.

With this update you can _**also**_ use some predefined nicknames to achieve the same result.

* Example after update: `b.f hc` or all the previously mentioned methods.

List of nicknames can be viewed [here](https://github.com/yakiimoninja/baiken/blob/main/data/nicknames.json).

&#x200B;

### Added a variety of new aliases for various moves.

Unfortunately I wasn't organizing them so keeping track was difficult. However that leads me to the third feature.

&#x200B;

### Added a patch notes page.

As mentioned above, keeping track of the aliases was a bit difficult.

To solve that issue I created a folder inside the github project that will hold each version's patch notes.

The patch notes will include what's new, what's been removed and any new addition of nicknames or aliases.

List of patch notes can be found [here](https://github.com/yakiimoninja/baiken/blob/main/patch_notes/).
