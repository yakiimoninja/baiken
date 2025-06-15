# Changelog

All notable changes to this project will be documented in this file.
The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).


## [Unreleased]
- Find a non blocking way to update the db.

## [2.0.0] - 2025-06-05

### Changed
- Make `/hitboxes` command display an hitbox image caption.
- Make embed titles display move inputs and or names dynamically to avoid clutter.
- Replace `rand` dependency with `nanorand`.
- Update dependencies.
- Replace `.replace()` with `aho-corasick` for text replacement.
- Rework data storage to use a sqlite database instead of json files.
- Rework error handling. Error responses are now ephemeral.
- Rework CHANGELOG.

- **`Goldlewis Dickinson`**:
    - `236S Level 1` change to `236S`.
    - `236S` change alias from `MG1` to `MG`.

### Added
- Add new command `/list`.
- Add new character **`Venom`**.
- Add new character **`Unika`**.

- **`Asuka R`**:
    - `214X (Discard)` add new alias `Discard`.
    - `214X (Draw)` add new aliases `Draw`.
    - `j.214X (Discard)` add new alias `Air Discard`, `ADI`.
    - `j.214X (Draw)` add new aliases `Air Draw`, `ADR`.

- **`Queen Dizzy`**:
    - Add new nickname `Queef Dizzy`.

- **`Ramlethal Valentine`**:
    - `214P` add new alias `Rekka`.
    - `214P 4P` add new alias `214P4P`.
    - `214P 4P 4P` add new alias `214P4P4P`.

- **`Venom`**:
	- `214X` add new aliases `Ball Set`, `Ball`, `214P`, `214K`,`214S`, `214H`.
	- `41236K` add new aliases `Ball Set (Multiple)`, `Balls`, `BSM`, `426K`.
	- `623X` add new aliases `QV`, `Q5`, `623P`, `623K`, `623S`, `623H`.
	- `[4]6S` add new aliases `Stinger Aim S`, `Stinger Aim`, `SA`, `SAS`, `46S`, `46S!`.
	- `[4]6H` add new aliases `Stinger Aim H`, `SAH`, `46H`, `46H!`.
	- `[2]8S` add new aliases `Carcass Raid S`, `Carcass Raid`, `CRS`, `CR`, `28S`, `28S!`.
	- `[2]8H` add new aliases `Carcass Raid H`, `CRH`, `28H`, `28H!`.
	- `j.236S` add new aliases `Carcass Raid (mid-air) S`, `Air Carcass Raid S`, `Air Carcass Raid`, `ACR`, `ACRS`, `ACR!`, `ACRS!`, `j.236S!`.
	- `j.236H` add new aliases `Carcass Raid (mid-air) H`, `Air Carcass Raid H`, `ACRH`, `ACRH!`, `j.236H!`.
	- `632146K` add new aliases `Tryambaka`,`Command Grab`, `Ball Grab`, `BG`, `TB`, `CG`, `6246K`.
	- `632146S` add new aliases `Dark Angel`, `Big Ball`, `Ball Super`, `DA`, `BB`, `BS`, `6246S`.
	- `632146H` add new aliases `Navaratna Runout`, `Black Hole`, `Super`, `BH`, `NR`, `6246H`.

- **`Unika`**:
    - `S~2S` add new aliases `S2S`, `2S2S`, `fS2S`,
    - `S~5S` add new aliases `SS`, `2SS`, `fSS`,
    - `H~2H` add new aliases `H2H`, `2H2H`,
    - `H~5H` add new aliases `HH`, `2HH`,
    - `j.236H` add new aliases `Blitz`, `Fireball`, `Air Fireball`, `FB`, `AF`,
    - `en j.236H` add new aliases `MA Blitz`, `MA Fireball`, `MA Air Fireball`, `MAB`, `MAFB`, `MAAF`,
    - `214K` add new aliases `Top Attack`, `Greed Sever`, `Overhead`, `GS`, `TA`,
    - `214S` add new aliases `Blaze A Trail`, `Michael Sword`, `Sword`, `BAT`, `MS`,
    - `236H` add new aliases `Streak`, `Laser`,
    - `236K` add new aliases `Penetrate`, `Stun Dipper`, `Low`, `SD`,
    - `623S` add new aliases `Blast Off`, `DP`, `BO`,
    - `en.214K` add new aliases `MA Top Attack`, `MA Greed Sever`, `MA Overhead`, `MAO`, `MAGS`, `MATA`,
    - `en.214S` add new aliases `MA Blaze A Trail`, `MA Michael Sword`, `MA Sword`, `MABAT`, `MAMS`,
    - `en.236H` add new aliases `MA Streak`, `MA Laser`, `MAS`, `MALA`,
    - `en.236K` add new aliases `MA Penetrate`, `MA Stun Dipper`, `MA Low`, `MAL` ,`MASD`, `MAP`,
    - `en.623S` add new aliases `MA Blast Off`, `MA DP`, `MABO`, `MADP`,
    - `632146S` add new aliases `Super`, `Gun`,
    - `en.632146S` add new aliases `MA Super`, `MA Gun`,
    - `214214H` add new aliases `Weapons Free`, `WF`, `Install`,
    - `S+H` add new aliases `Annihilate`, `SH`,
    - `Weapons Free Deactivation` add new alias `WFDE`,

### Removed
- **`Asuka R`**:
    - `j.214X (Discard)` remove alias `Air Discard`, `ADI`.
    - `j.214X (Draw)` remove aliases `Air Draw`, `ADR`.

- **`Goldlewis Dickinson`**:
    - Remove `236S Level 2` and aliases.
    - Remove `236S Level 3` and aliases.

### Fixed
- Fix weird character strings in detailed move information, hitbox captions and character info.
- Fix **`Ramlethal Valentine`** alias typo `Sabrobato`.
- Fix **`Millia Rage`** `Iron Savior` embed footer caption containing weird characters.
- Fix **`Faust`** all aliases for items not working.
- Fix **`Bridget`** `f.SS` aliases not working cause of new input.
- Fix **`Goldlewis Dickinson`** `236S` aliases not working cause of new input.
- Fix **`Ramlethal Valentine`** `Erarlumo 2` and `Erarlumo 3` aliases not working cause of new input.

## [1.0.0] - 2024-11-25

### Changed
- Change all embed title to contain name and input for move called.
- Change `/frames` command to a command group.
- Change `/help` command response to an embed.
- Change `/hitboxes` command response to a single embed with all images attached.
- Change `/moves` command response to an embed.
- Change `/nicknames` command response to an embed.
- Change `/info` command response to an embed.
- Change `/stats` command response to an embed.
- Change `/update` command responses to ephemeral.
- Change `/xx` command to ephemeral.
- Change `/report` (previously `/feedback`) command to have and extra `subject` argument field.
- Improve codebase when getting character data.
- Move help pictures for commands inside `data/images/commands`.
- Update README and affected images.
- Revert recalculation of previous version releases and start from [1.0.0].

### Added
- Add `/frames simple`, `/frames advanced` and `/frames meter` as subcommands for `/frames`.
- Add `/help` entries for new `/frames` subcommands.
- Add the ability to display moves by catergory when using `/moves` command.
- Add more data fields for `/info` command.

### Removed
- Remove `/fmeter` command. Its now a `/frames` subcommand.
- Remove`/feedback` command. Its now renamed to `/report`.

### Fixed
- Fix [0.30.2] tag.
- Update framedata (new fields).

## [0.30.2] - 2024-11-12

### Fixed
- Rewrite CHANGELOG to better follow keepachangelog and semantic versioning.
- Bump `Cargo.toml` to recalcuted version.

## [0.30.1] - 2024-11-10

### Changed
- Rewrite CHANGELOG.md

### Added
- Add new github actions to check compilation errors.
- Add new github action to create new releases.
- Add new github action to creat release notes according to CHANGELOG.md.

### Removed
- **`Potemkin`**:
    - Remove `Kara Mega Fist` and aliases.

- **`Potemkin`**:
    - `214K` add new alias `Flick`.
    - `214[K]` add new aliases `F.D.B. (Charged)`, `Flick!`, `FDB!`, `214K!`

### Fixed
- Update framedata
- Update `Potemkin` inputs.

## [0.30.0] - 2024-10-31

### Changed
- Rework serialization for `frames`, `images` and `info` data when updating.
- Replace file deletion with truncate when updating.

### Added
- **`Queen Dizzy`**:
    - `214S` add new aliases `Michael Sword`, `S Michael Sword`, `Sword`, `S Sword`, `MS`, `SMS`, `SS`.
    - `214H` add new aliases `H Michael Sword`, `H Sword`, `HMS`, `HS`.
    - `214P/K` add new aliases `We talked a lot together`, `Fish`, `WTALT`, `214P`, `214K`.
    - `22H` add new aliases `Wings of Light`, `Wings`, `WOL`.
    - `236K` add new aliases `For roasting chestnuts`, `Fireball`, `Deez nuts`, `FRCN`, `FB`, `DN`.
    - `236S/H` add new aliases `I used this to catch fish`, `Spike`, `Pillar`, `IUTTCF`, `CF`, `SP`, `PI`, `236S`, `236H`.
    - `236S~6S/236H~6H` add new aliases `Ice Field`, `Floor is lava`, `Floor is ice`, `IF`, `FIL`, `FII`, `236S~6S`, `236H~6H`, `236SS`, `236HH`.
    - `632146S` add new aliases `Imperial Ray`, `Super`, `Ice Super`, `IS`, `IR`.
    - `632146H` add new aliases `Gamma Ray`, `Lazer`, `Lazer Super`, `Kamehameha`, `LS`, `GR`.

- **`Chipp Zanuff`**:
    - `214H` add new aliases `Tiighrope`, `Rope`, `TR`.

## [0.29.0] - 2024-10-30

### Changed
- Make `/register` and `/feedback` command responses, ephemeral.
- Update command descriptions.
- Update `/help` help messages.
- Update `README`.

### Added
- Add new command `/xx` that disables easter eggs in current server. (Admin only)
- Add new character **`Queen Dizzy`**.

### Removed
- Remove aliases for prefix commands.

### Fixed
- Fix `/update` failing to update **`Asuka`** data. (Sorry Asuka bros)
- Update framedata.

## [0.28.2] - 2024-07-30

### Changed
- Clippy code clean up.

### Added
- **`Zato-1`**:
    - `22 During 214K(HOLD)` add new aliases `Eddie Teleport`, `Swap`, `ET`, `TP`.

### Fixed
- Update framedata.

## [0.28.1] - 2024-07-24

### Added
- **`Bridget`**:
    - `236S/H` add new aliases `236S`, `236H`.
    - `214S/H` add new aliases `236S`, `236H`.
    - `63214P` add new alias `624P`.

## [0.28.0] - 2024-07-24

### Changed
- Update github README to display current server count.
- Hyperlink titles for `/frames`, `/fmeter`, `/hitboxes` and `moves` now redirect to character overview pages.
- Hyperlink titles for `/info` now redirect to corresponding character page.

### Added
- Add new command `/stats` that displays Baiken's server and member count.

### Fixed
- Fix the parsing of `unique moment options` to not contain `[[GGST/CHAR/UMO|UMO]]` template in info.
- Update framedata.

## [0.27.1] - 2024-07-05

### Added
- **`Slayer`**:
    - Add new nickname `Dandy`. 

- **`Johnny`**:
    - Add new nickname `Lean`.

- **`Johnny`**:
    - `214K` add new alias `KMF`.
    - `214P` add new alias `PMF`.
    - `214S` add new alias `SMF`.
    - `j.214[P/K/S]` add new aliases `j.214K!`, `j.214P!`, `j.214S!`, `JMF`.
    - `j.214]K[` add new aliases `AKMF`, `JKMF`.
    - `j.214]P[` add new aliases `APMF`, `JPMF`.
    - `j.214]S[` add new aliases `ASMF`, `JSMF`.
    - `Mist Finer 44` add new alias `MF44`.
    - `Mist Finer 66` add new alias `MF66`.
    - `632146H` add new alias `TMN`.

### Fixed
- Update framedata.

## [0.27.0] - 2024-06-15

### Added
- **`Slayer`**:
    - `214K` add new aliases `Dandy Step K`, `DSK`.
	- `214P` add new aliases `Dandy Step P`, `DSP`.
	- `214P/K~H` add new aliases `Master's Hammer`, `214KH`, `214PH`, `MH`, `Guard Crush`.
	- `214P/K~K` add new aliases `Bump Ahead`, `214KK`, `214PK`, `BA`, `Low`.
	- `214P/K~P` add new aliases `Pilebunker`, `214KP`, `214PP`, `PB`.
	- `214P/K~S` add new aliases `It's Late`, `214KS`, `214PS`, `IL`, `Overhead`.
	- `236K` add new aliases `Mappa Hunch K`, `MHK`.
	- `236P` add new aliases `Mappa Hunch P`, `MHP`.
	- `44~6S` add new aliases `Hand of Doom`, `446S`, `6S`, `Backdash S`, `BDS`, `HOD`.
	- `63214H` add new aliases `Bloodsucking Universe`, `Suck`, `BU`.
	- `236236H` add new aliases `Last Horizon`, `LH`, `Suck Super`.
	- `632146S` add new aliases `Super Mappa Hunch`, `Super`, `Naked`, `SMH`.

### Fixed
- Fix message formatting when calling for Negative Edge moves.
- Update framedata.

## [0.26.0] - 2024-05-29

### Added
- Add new character **`Slayer`**.

### Fixed
- Update framedata.

## [0.25.1] - 2024-04-17

### Added
- **`A.B.A`**:
    - `63214P` add new alias `624P`.

- **`Leo Whitefang`**:
    - `[s/h] h/s` add new aliases `s/h`, `h/s`.

### Fixed
- Updated framedata.

## [0.25.0] - 2024-04-02

### Changed
- Refactor `/frames`, `/fmeter`, `hitboxes` to show the name and move of the character as a title.
- Rework some hyperlink formatting.

### Added
- **`A.B.A`**:
    - Add new nickname `A.B.A.`.

### Fixed
- Update framedata.

## [0.24.0] - 2024-03-29

### Added
- Add new character **`A.B.A`**.

- **`A.B.A`**:
    - Add new nicknames `AB`, `ABA`, `Paracelsus`.

- **`A.B.A`**:
    - `c.S` add new alias `5S`.
	- `2D` add new alias `Sweep`.
	- `5[D]` add new alias `5D!`.
	- `JR c.S` add new alias `JR5S`.
	- `JR 5[D]` add new alias `JR5D!`.    
    - `214H` add new aliases `Bonding and Dissolving`, `BND`.
	- `214K` add new aliases `Haul and Heed`, `HNH`.
    - `214S` add new aliases `Frenzy and Astonishment`, `FNA`.
	- `236K` add new aliases `Intertwine and Tilt`, `INT`.
	- `236S` add new aliases `Menace and Groan`, `MNG`.
	- `236S~6S` add new aliases `Restriction and Constraint`, `236S 6S`, `236S6S`, `RNC`.
	- `623H` add new aliases `Judgment and Sentiment`, `JNS`.
	- `63214P` add new aliases `Changing and Swaying`, `CNS`.
	- `JR 214H` add new aliases `JR Bonding and Dissolving`, `JR214H`, `JRBND`.
	- `JR 214K` add new aliases `JR Haul and Heed`, `JR214K`, `JRHNH`.
	- `JR 236K` add new aliases `JR Intertwine and Tilt`, `JR236K`, `JRINT`.
	- `JR 236S` add new aliases `JR Menace and Groan`, `JR236S`, `JRMNG`.
	- `JR 236S~6S` add new aliases `JR Restriction and Constraint`, `JR236S6S`, `JR236S 6S`, `JRRNC`.
	- `JR 63214P` add new aliases `JR Changing and Swaying`, `JR63214P`, `JRCNS`.
	- `632146H` add new aliases `The Law is Key, Key is King.`, `Super`.
	- `632146K` add new aliases `Keeper of the Key`, `Door Super`.
	- `JR 632146H` add new aliases `JR The Law is Key, Key is King.`, `JR Super`, `Install Super`.
	- `JR 632146K` add new aliases `JR Keeper of the Key`, `JR Door Super`, `Install Door Super`.
    - `6D or 4D` add new aliases `Throw`, `6D`, `4D`.
	- `j.6D or j.4D` add new aliases `Air Throw`, `J6D`, `J4D`.
	- `236D` add new aliases `Wild Assault`, `WA`.
    - `236[D]` add new aliases `Wild Assault Hold`, `WAH`, `WA!`, `236D!`.

- **`Millia Rage`**:
    - `214H` add new alias `Claw`.

### Fixed
- Update framedata.

## [0.23.0] - 2024-03-22

### Added
- Add the ability to update data automatically at set intervals.

### Fixed
- Update framedata.

## [0.22.0] - 2024-02-10

### Changed
- Rework the text formatting of `/moves`, `/nicknames` and `/help notes` once more.

### Fixed
- Update framedata.

## [0.21.0] - 2024-02-2

### Changed
- Rework text formatting of `/fmeter`, `/moves`, `/nicknames` commands.
- For commands `/frames`, `/fmeter`, `/hitboxes` and `/moves` Dustloop links are now masked. \
Clicking the provided link redirects to the move in the character page, instead of the top of the page.

### Added
- Add new command `/info` that displays character information like `Backdash`, `Guts` etc.

### Fixed
- Update framedata.

## [0.20.1] - 2024-01-29

### Changed
- Update command descriptions.
- Update README.

### Fixed
- Fix mislabed `j.214k` aliases for all `Air Mist Finers`.
- Update framedata.

## [0.20.0] - 2024-01-24

### Changed
- Codebase and dependency update (poise 0.6.1).

### Fixed
- Fix the `/update` command not fetching the hitbox images for `Wild Assault` moves.
- Update framedata.

## [0.19.2] - 2024-01-22

### Added
- **`Bridget`**:
    - `236K P` add new aliases `236KP`.
	- `236K K` add new aliases `236KK`.

### Fixed
- Fix **`Bridget`** `Move 236KP was not found!` when using `Brake` as an alias.
- Fix **`Bridget`** `Move 236KK was not found!` when using `Shoot` as an alias.
- Update framedata.

## [0.19.1] - 2024-01-01

### Added
- **`Axl Low`**:
    - `214K` add new aliases `Whistling Wind`, `Tornado`, `WW`.
	- `214[K]` add new aliases `Whistling Wind (Charged)`, `Charged Tornado`, `CWW`, `WWC`.

- **`Elphelt`**:
    - `236S/H` add new aliases `Miss Charlotte`, `Fireball`, `FB`, `MC`.
    - `j.236S/H` add new aliases `Air Miss Charlotte`, `Air Fireball`, `JF`, `AF`, `JMC`, `AMC`.

- **`Potemkin`**:
	- `41236H` add new aliases `Heat Tackle`, `Jet Thrusters`, `HT`, `JT`.

### Fixed
- **`Johnny`**:
	- `236236S` remove alias `CS` cause overlap with `close slash`.

- **`Elphelt`**:
	- `236S` and `236H` merge into `236S/H` therefore all `236H` aliases are removed.
    - `j.236S` and `j.236H` merge into `j.236S/H` therefore all `j.236H` aliases are removed.

- Update framedata.

## [0.19.0] - 2023-12-19

### Added
- **`Elphelt Valentine`**:
    - `214S~P` add new aliases `Up High!`, `Overhead`, `Rekka 2`, `R2`, `UH`, `214SP`.
    - `214S~K` add new aliases `Down Low!`, `Low`, `Rekka 3`, `R3`, `DL`, `214SK`.
    - `214S~P/K~P` add new aliases `Up High! (after Up High! or Down Low!)`, `Overhead Finisher`, `Rekka 22`, `Rekka 32`, `R22`, `R32`, `UHF`, `214SPP`, `214SKP`.
    - `214S~P/K~K` add new aliases `Down Low! (after Up High! or Down Low!)`, `Low Finisher`, `Rekka 33`, `Rekka 23`, `R23`, `R33`, `DLF`, `214SPK`, `214SKK`.
    - `214S~H` add new aliases `Nailed It!`, `Rekka 4`, `Shotgun`, `R4`, `SG`, `NI`, `214SH`.

### Removed
- Remove autocomplete in most commands because it caused images to not load properly.

### Fixed
- Update framedata.

## [0.18.0] - 2023-12-08

### Added
- Add new character **`Elphelt Valentine`**.

- **`Elphelt Valentine`**:
    - Add new nicknames `Elphelt Valentine`, `Die`, `EV`.

- **`Elphelt Valentine`**:
    -`236S` add new aliases `Miss Charlotte`, `Fireball`, `FB`, `MC`.
	-`236H` add new aliases `Heavy Miss Charlotte`, `Heavy Fireball`, `HFB`, `HMC`.
	-`214K` add new aliases `Miss Charlotte (Out of Repair)`, `Buttslam`, `BS`, `MCOR`.
	-`214H` add new aliases `Bomb-Bom Chocolat`, `Grenade`, `BBC` (lol).
	-`214S` add new aliases `Here I Go!`, `Rekka`, `Rekka 1`, `R1`, `HIG`.
	-`214S~S` add new aliases `Here I Go!`, `Rekka`, `Rekka 1`, `R1`, `HIG`.
	-`214S~P` add new aliases `Up High!`, `Overhead`, `Rekka 2`, `R2`, `UH`.
	-`214S~K` add new aliases `Down Low!`, `Low`, `Rekka 3`, `R3`, `DL`.
	-`214S~H` add new aliases `Nailed It!`, `Rekka 4`, `Shotgun`, `R4`, `SG`, `NI`.
	-`236236K` add new aliases `Bomb-Bombnnière`, `Grenade Super`, `GS`, `BB`.
	-`236236K Explosion` add new aliases `Bomb-Bombnnière Explosion`, `Grenade Explosion`, `GE`, `BBE`.
	-`632146H` add new aliases `Juganto Da Parfeo`, `Super`, `Die`, `JDP`.
    
### Fixed
- Update framedata.

## [0.17.0] - 2023-11-25

### Added
- Add new easter egg.

### Removed
- Remove `/aliases` command.

### Changed
- Even more code cleanup.

### Fixed
- Update framedata.

## [0.16.0] - 2023-11-24

### Deprecated
- Deprecate `/aliases` and integrate its functionality to `/moves` command. 

### Changed
- Some more codebase cleanup.

### Fixed
- Fix `megafist` not showing anything when used as an alias.
- Fix prompts that include the old `/request` command instead of the new `/feedback` command.

## [0.15.0] - 2023-11-21

### Changed
- Rename command arguments.
- Rework `/help` command.
- Major codebase cleanup.

### Added
- Add new command `/feedback` replacement of `/request`.
- Autocompletion of character names and options in commands.

### Removed
- Remove `/request` and replace with `/feedback`.

### Fixed
- Update framedata.

## [0.14.0] - 2023-09-15

### Added
- Add new character **`Johnny`**.

- **`All Characters`**:
    -`236D` add new aliases `Wild Assault`, `WA`.
    -`236[D]` add new aliases `Wild Assault Hold`, `WAH`, `WA!`, `236D!`.

- **`Anji Mito`**:
    - `236K~214P` add new aliases `Midare`, `Fish`.

- **`Faust`**:
    - `236P` add new aliases `22P`, `Spit`.
    - `236[P]` add new aliases `What Could This Be?`,`Item Eat`,`Eat`,`IE`,`236P!`

- **`Giovanna`**:
    - `214H` add new aliases `Chave`, `Dash`.

- **`I-No`**:
    - `214S` add new aliases `Mad Love Agitato`, `MLA`.

- **`Ramlethal Valentine`**:
    - `236K` add new aliases `Ondo`, `Rock`.

- **`Zato-1`**:
    -`22 During 214K(HOLD)` add new aliases `Eddie Swap`, `ES`.

- **`Johnny`**:
    - `214H` add new alias `Ensenga`.
    - `214[P/K/S]` add new aliases `Mist Finer Stance`, `214K!`, `214P!`, `214S!`, `MF`.
    - `214]K[` add new aliases `Mist Finer (Horizontal)`, `214K`, `Mist Finer`, `MFH`, `MFK`.
    - `214]P[` add new aliases `Mist Finer (Upward)`, `214P`, `Mist Finer Up`, `MFU`, `MFP`.
    - `214]S[` add new aliases `Mist Finer (Downward)`, `214S`, `Mist Finer Down`, `MFD`, `MFS`.
    - `236H` add new alias `Vault`.
    - `236HH` add new aliases `Vault Deal`, `Flip Card`, `VD`, `FC`.
    - `236P/K/S` add new aliases `Deal`, `Card`, `236P`, `236K`, `236S`.
    - `Card hit by Mist Finer` add new aliases `Turn Up`, `Card Cut, `, `TU`, `CC`.
    - `H or D` add new aliases `Mist Finer Cancel`, `MFC`.
    - `j.214[P/K/S]` add new aliases `Mist Finer Stance`, `Air Mist Finer Stance`, `AMF`.
    - `j.214]K[` add new aliases `Air Mist Finer (Horizontal)`, `j.214K`, `AMFH`, `AMFK`.
    - `j.214]P[` add new aliases `Air Mist Finer (Upward)`, `j.214k`, `AMFU`, `AMFP`.
    - `j.214]S[` add new aliases `Air Mist Finer (Downward)`, `j.214k`, `AMFD`, `AMFK`, `Overhead`.
    - `j.236H` add new aliases `Air Deal`, `AD`, `Air Card`, `AC`.
    - `Mist Finer 44` add new aliases `Mist Finer Dash (Backward)`, `MFBD`.
    - `Mist Finer 66` add new aliases `Mist Finer Dash (Forward)`, `MFFD`.
    - `236236S` add new aliases `Joker Trick`, `Card Super`, `JT`, `CS`.
    - `632146H` add new aliases `That's My Name`, `Super`.

### Fixed
- Update framedata.

## [0.13.0] - 2023-05-25

### Added
- `Asuka R`:
    - Add new nicknames `Asuka`, `That Man`, `Oscar`.

### Fixed
- Fix and issue where using `/aliases` command for **`Jacko`** wouldn't display anything.
- Update framedata.

## [0.12.0] - 2023-05-21

### Fixed
- Fix alias searching loops to break when alias is found (i had forgor 💀).
- Fix an issue where `/hitboxes` command would not display moves when using aliases that had a `.` in them.
- Fix **`Bedman`** aliases file move input: 
    - From `13C (214H) 236P` to `13C (236H) 236P`.
    - From `13C (214P) 236P` to `13C (236P) 236P`.
    - From `13C (214S) 236P` to `13C (236S) 236P`.

## [0.11.0] - 2023-05-18

### Added
- **`Bedman`**:
    - `!H` add new aliases `error 6E H`, `6EH`, `EH`.
    - `!{H}` add new aliases `Partial error 6E {H}`, `6E!H`, `P6EH`, `E!H`, `PEH`.
    - `![H]` add new aliases `Charged error 6E [H]`, `6E!!H`, `C6EH`, `E!!H`, `CEH`.
    - `!P` add new aliases `error 6E P`, `6EP`, `EP`.
    - `!S` add new aliases `error 6E S`, `6ES`, `ES`.
    - `(236H) 214P` add new aliases `call 0x$0.20 236H`, `Instant H`, `IH`, `C2H`.
    - `(236P) 214P` add new aliases `call 0x$0.20 236P`, `Instant P`, `IP`, `C2P`.
    - `(236S) 214P` add new aliases `call 0x$0.20 236S`, `Instant S`, `IS`, `C2S`.
    - `13C !H` add new aliases `13C error 6E H`, `13C6EH`, `13CEH`.
    - `13C !P` add new aliases `13C error 6E P`, `13C6EP`, `13CEP`.
    - `13C !S` add new aliases `13C error 6E S`, `13C6ES`, `13CES`.
    - `13C (214H) 236P` add new aliases `13C call 0x$0.20 H`, `13CC2H`.
    - `13C (214P) 236P` add new aliases `13C call 0x$0.20 P`, `13CC2P`.
    - `13C (214S) 236P` add new aliases `13C call 0x$0.20 S`, `13CC2S`.
    - `214K` add new aliases `call 0x$1.00`, `C1`, `Delay`.
    - `214P` add new aliases `call 0x$0.20`, `C2`, `Instant`.
    - `236P` add new aliases `call 4BA`, `4BA`, `Fireball`, `FB`.
    - `236S` add new aliases `call 4B3`, `4B3`, `Beyblade`, `BB`.
    - `236H` add new aliases `call 4B9 (Malfunction)`, `4B9`, `Arm Swing`, `Swing`, `AS`, `236H`.
    - `236{H}` add new aliases `Partial call 4B9 (Malfunction)`, `P4B9`, `Partial Arm Swing`, `Partial Swing`, `PAS`, `236!H`
    - `236[H]` add new aliases `Charged call 4B9 (Malfunction)`, `C4B9`, `Charged Arm Swing`, `Charged Swing`, `CAS`, `236!!H`.

- **`Nagoriyuki`**:
	- `j.S Level 1` add new alias `j.S`.
	- `j.S Level 2` add new alias`j.S2`.
	- `j.S Level 3` add new alias`j.S3`.
	- `j.S Level BR` add new alias`j.SB`.

### Fixed
- Fix issue where **`Bedman`** was displaying different moves than what requested. \
    e.g. `/fmeter bedman 214P` displaying info for `(236H) 214P`. \
    Bedman's specials broke bot's program logic (error 6E lmao).

- Fixed and issue where `/fmeter` command wasn't displaying the right amount of startup frames inside `[ brackets ]`.
- Updated framedata.

## [0.10.6] - 2023-05-07

### Added
- **`Jack-O`**:
    - Add new nickname `Jack-O Valentine`.

- **`Bedman?`**:
    - Add new nickname `Delilah`.

- **`Anji`**:
    - `236P` add new alias `Bird`.

### Fixed
- Fix dots in `nicknames` command.
- Fix some typos.
- Update framedata.

## [0.10.5] - 2023-04-11

### Fixed
- Fix **`Bedman`** not displaying stuff properly.

## [0.10.4] - 2023-04-06

### Added
- Add new character **`Bedman`**.

- **`Bedman`**:
    - Add new nicknames `Bedwoman`, `Bed`.

- **`Bedman`**:
    - `(236H) 214P` add new alias `214PH`.
    - `(236P) 214P` add new alias `214PP`.
    - `(236S) 214P` add new alias `214PS`.
    - `214K` add new aliases `call 0x$1.00`, `C1`, `Delay`.
    - `214P` add new aliases `call 0x$0.20`, `C2`, `Instant`.
    - `236H` add new aliases `call 4B9 (Malfunction)`,`4B9`,`Arm Swing`,`Swing`,`AS`.
    - `236P` add new aliases `call 4BA`,`4BA`,`Fireball`,`FB`.
    - `236S` add new aliases `call 4B3`,`4B3`,`Beyblade`,`BB`.
    - `j.214K` add new aliases `Air call 0x$1.00`, `Air C1`, `Air Delay`, `AC1`, `AD`.
    - `j.214P` add new aliases `Air call 0x$0.20`, `Air C2`, `Air Instant`, `AC2`, `AI`.
    - `j.236P` add new aliases `Air call 4BA`,`Air 4BA`,`A4BA`,`Air Fireball`,`Air FB`,`AFB`.
    - `j.236S` add new aliases `Air call 4B3`,`Air 4B3`,`A4B3`,`Air Beyblade`,`Air BB`,`ABB`.
    - `632146H` add new aliases `call 4CC`,`4CC`,`Super`.
    - `632146S` add new aliases `call 13C`,`13C`,`Install`.

### Fixed
- **`Leo Whitefang`**:
    - `632146S` replace aliases `Fan Super` and `FS` with `Projectile Super` and `PS`. Due to overlap couldn't access `fS`.

- Update framedata.

## [0.10.3] - 2023-04-01

### Added
- **`Chipp Zanuff`**:
    - `236S` add new alias `Rekka`.

- **`Ramlethal Valentine`**:
    - `236236S` add new alias `Motorboat`.

### Fixed
- Update framedata. 

## [0.10.2] - 2023-02-25

### Changed
- Refactor code to exhaustively search through the character nicknames first.

### Fixed
- Fix `the application did not respond` error when using the `/moves` command for **`Nagoriyuki`**.
- Update framedata. 

## [0.10.1] - 2023-01-25

### Fixed
- Fix missing moves for **`Nagoriyuki`** and **`Ky`**.
- Update framedata.

## [0.10.0] - 2023-01-20

### Changed
- Update the github README. Now has pictures to help explain each command.

### Added
- Add new command `/fmeter` that displays visually the startup, active and recovery frames of a character's move.
- Rework the command `/help` to display specific help messages if a command name, `notes` or `specifics` is given as an option.

### Fixed
- Update framedata.

## [0.9.0] - 2022-12-25

### Changed
- Improve data parsing from dustloop which cut the `/update` command time to less than half.
- Rework the `/update` command to update frames and or data for all or specific characters.

### Added
- Add new `Attack Level`, `Scaling` and `Risc Gain` information fields for the `/frames` command.
- Add new `Attack Level`, `Scaling` and `Risc Gain` information fields for the character json files.
- Auto parsing of images and hitboxes instead of having to grab them manually.

- **`Ramlethal Valentine`**:
    - Add new nickname `Borgar`.
  
- **`Happy Chaos`**:
    - Add new nickname `Asshole`.

- **`Potemkin`**:
    - `632146P` add new alias `Hug`.

- **`Potemkin`**:
    - `632146P` add new alias `Super Hug`.
  
- **`Happy Chaos`**:
    - `632146P` add new alias `Balls`.

### Fixed
- Update framedata.

## [0.8.5] - 2022-12-20

### Added
- **`All Characters`**:
    - `2D` add new alias `Sweep`.
 
- **`Sin Kiske`**:
    - `632S` add new alias `DP`.
    - `632SS` add new aliases `DP Followup`, `DPF`.

### Fixed
- Update framedata.

## [0.8.4] - 2022-12-09

### Fixed
- Fix `/aliases` command spitting `The application did not respond` error.
- Fix `/moves` command not showing anything for **`Goldlewis`**.

## [0.8.3] - 2022-12-09

### Added
- **`Nagoriyuki`**:
    - `f.S Level 1` add new alias `fs`.

### Fixed
- Update framedata.

## [0.8.2] - 2022-12-02

### Changed
- Update command usage instructions.

### Added
- **`Axl Low`**:
    - Add new nicknames `Brit`,`British`.
 
- **`Zato-1`**:
    - Add new nickname `Eddie`.

- **`Zato-1`**:
    - `236K` add new alias `Clap`.
    - `]K[` add new aliases `NE Clap`, `NEC`.

### Fixed
- Update framedata.

## [0.8.1] - 2022-11-25

### Added
- Add new character **`Sin Kiske`**.

- **`Sin_Kiske`**:
    - Add new nickname `SK`.

- **`Sol Badguy`**:
    - `236P` add new alias `Gunflame`.
    - `214P` add new alias `Feint Gunflame`.

- **`Sin Kiske`**:
    - `c.S` add new alias `5S`.
    - `5[D]` add new alias `5D!`.
    - `236H` add new aliases `Beak Driver`, `BD`.
    - `236HH` add new aliases `Beak Driver Followup`, `BDF`.
    - `623S` add new aliases `Hawk Baker`, `HB`.
    - `623SS` add new aliases `Hawk Baker Followup`, `HBF`.
    - `214S` add new aliases `Hoof Stomp`, `HS`.
    - `214SS` add new aliases `Hoof Stomp Followup`, `HSF`.
    - `236K` add new aliases `Elk Hunt`, `EH`.
    - `236KK` add new aliases `Elk Hunt Followup`, `EHF`.
    - `66` add new aliases `Gazelle Step`, `GS`.
    - `63214P` add new aliases `Still Growing`, `Eat`, `Food`, `SG`.
    - `632146H` add new aliases `R.T.L.`, `RTL`.
    - `632146HH` add new aliases `R.T.L. Followup`, `RTLF`.
    - `236236P` add new aliases `Tyrant Barrel`, `Tyrant Rave`, `TB`, `TR`.
    - `236236PP` add new aliases `Tyrant Barrel Followup`, `Tyrant Rave Followup`, `TBF`, `TRF`.

## [0.8.0] - 2022-11-11

### Added
- Add new command `/nicknames` to print all existing nicknames for characters.

- **`Zato-1`**:
    - Add new nickname `Zato One`.

### Removed
- Remove all 2 letter nicknames for characters without 2 part names, as redundant. Bot can find characters as is.

## [0.7.6] - 2022-11-05
 
### Fixed 
- Fix **`Faust 5K`** broken hitbox image link.
- Fix **`Nagoriyuki`** aliases command not displaying anything.

## [0.7.5] - 2022-10-21

### Changed
- Code clean up. Remove the use of the `init.json` file.
   
### Added
- Add the full name as a nickname for all characters applicable.

- **`Sol Badguy`**:
    - Add new nickname `Sol Goodman`.

- **`All Characters`**:
    - `c.S` add new alias `5S`.

- **`Bridget`**:
    - `SS` add new alias `5SS`.
    - `HH` add new alias `5HH`.
    - `236KP` modified alias from `SB` to `SLB` due to alias overlap.
    - `236KK` modified alias from `SS` to `SLS` due to alias overlap.

- **`Chipp Zanuff`**
    - `236P` add new aliases `P Alpha`, `PA`.
    - `j.236P` add new aliases `Air P Alpha`, `APA`.
    - `236K` add new aliases `K Alpha`, `KA`.
    - `j.236K` add new aliases `Air K Alpha`, `AKA`.
    - `623S` add new alias `Beta`.
    - `j.623S` add new aliases `Air Beta`, `AB`.
    - `236H` add new alias `Gamma`.

## [0.7.4] - 2022-10-14

### Added
- **`Axl Low`**:
    - `j.236H` add new alias `TKB`.

- **`Baiken`**:
    - `j.214214P` add new alias `Air Kenjyu`.

- **`Faust`**:
    - `214P` add new alias `P Scarecrow`.
    - `214K` add new alias `K Scarecrow`.
    - `214S` add new alias `S Scarecrow`.

- **`I-no`**:
    - `j.236K` add new alias `K Sultry Performance`.
    - `236H` add new alias `H Stroke The Big Tree`.

- **`Ky Kiske`**:
    - `DI 236K` add new alias `DI Stun Dipper`.
    - `DI 214K` add new alias `DI Foudre Arc`.
    - `DI 214S` add new alias `DI Dire Eclat`.
    - `DI 236236P` add new alias `DI Sacred Edge`.

- **`Leo Whitefang`**:
    - `bt.P` add new aliases `Bt Punch`,`BTP`.
    - `bt.K` add new aliases `Bt Low`, `BTK`, `BK`.
    - `bt.S` add new aliases `Bt Cross`, `BTS`.
    - `bt.H` add new aliases `Bt Overhead`, `BTH`, `BH`.
    - `bt.D` add new aliases `Back Turn Dust`, `Backturn Dust`, `Back Turn Parry`, `Bt Parry`.
    - `bt.22` add new alias `Bt Cancel`.
    - `bt.214K` add new aliases `Bt Command Grab`,`Bt Grab`,`Bt Throw`,`BTCG`,`BTG`,`BTT`. 
    - `bt.214H` add new alias `Bt Blitz`.

- **`May`**:
    - `[4]6S` add new alias `S Dolphin`.
    - `[2]8S` add new alias `S Up Dolphin`.

- **`Millia Rage`**:
    - `236S` add new alias `S Disk`.
    - `236H` add new alias `H Disk`.

- **`Nagoriyuki`**:
    - `623H` add new aliases `Shizuriyuki 1`, `DP1`.
    - `623HH` add new aliases `Shizuriyuki 2`, `DP2`.

- **`Potemkin`**:
    - `Reflect Projectile` add new alias `RP`.

- **`Ramlethal Valentine`**:
    - `236S` add new alias `S Sword Throw`.

- **`Sol Badguy`**:
    - `214P` add new alias `Feint Fireball`.
    - `623S` add new alias `S Volcanic Viper`.
    - `236K` add new aliases `Bandit Revolver 1`, `BR1`.
    - `236KK` add new aliases `Bandit Revolver 2`, `BR2`.
    - `j.236K` add new aliases `Air Bandit Revolver`, `Air Bandit Revolver 1`, `JBR1`, `ABR`, `ABR1`.
    - `j.236KK` add new aliases `Air Bandit Revolver`, `Air Bandit Revolver 1`, `JBR1`, `ABR`, `ABR1`.

### Removed
- **`Leo Whitefang`**:
    - `bt.214K` remove alias `BTP` as duplicate.

### Fixed 
- Update framedata.
- Fix `The application did not respond` error when invoking the `/update` command.
- Fix `invincibility` and `counter` showing `RISC` and `Proration` values.
- Fix **`Sol`** `Air Volcanic Viper` alias displaying framedata for `H Volcanic Viper` and vice versa.

## [0.7.3] - 2022-10-04

### Added
>add a nickname for may called "totsugeki" so we can search up totsugeki totsugeki
 
Say less my guy.

- **`May`**: new nickname `Totsugeki`.

- **`Nagoriyuki`**:
    - `j.D Level 1` add new alias `j.D`.
    - `j.D Level 2` add new alias `j.D2`.
    - `j.D Level 3` add new alias `j.D3`.
    - `j.D Level BR` add new alias `j.DB`.

### Fixed
- Fix bot message formatting.

## [0.7.2] - 2022-09-26

### Changed
- Clean up code.
- Rework error messages when moves are not found.

### Added
- **`Millia Rage`**:
    - `j.236K` add new alias `Turbofall`.

## [0.7.1] - 2022-09-21

### Changed
- Change move not found case for `/frames` command.
- Change follow up message for `/aliases` command.

### Added
- **`Goldlewis`**: new nickname `GL`.

- **`Chipp Zanuff`**:
    - `63214S` add new alias `Leaf Grab`.

## [0.7.0] - 2022-09-17

### Changed
- Rework how the `request` command works.
- New invite link and QR Code.

### Added
- Refactor code to support slash commands and the new gateway intents.
- Slash commands have context menus to help with command execution.
- Add new easter egg.

### Removed
- Replace prefix commands with [slash commands](https://github.com/yakiimoninja/baiken#commands) from `19/September/2022` due to changes to Discord's API.
- Remove the current easter egg.

- **`Baiken`** new nickname `Bacon`.
- **`Bridget`** new nickname `Brisket`.
 
- **`Bridget`**:
    - `236S/H` add new aliases `Stop and Dash`, `Fireball`, `Yoyo Throw`, `SD`, `YYT`.
    - `214S/H` add new aliases `Stop and Dash Return`, `Return Fireball`, `Yoyo Return`, `SDR`, `YYR`.
    - `214K` add new aliases `Rolling Movement`, `Ball`, `Spin`, `Sonic`, `Sonic Spin`, `RM`.
    - `623P` add new aliases `Starship`, `DP`.
    - `236K` add new aliases `Kick Start My Heart`, `Slide`, `KSMH`.
    - `236K P` add new aliases `Brake`, `Slide Brake`, `SB`.
    - `236K K` add new aliases `Shoot`, `Slide Shoot`, `SS`.
    - `j.236K`  new aliases `Roger Dive`, `Dive Kick`, `RD`, `DK`.
    - `63214P` add new aliases `Rock the Baby`, `Command Grab`, `Grab`, `RTB`, `CG`.
    - `632146S` add new aliases `Loop the Loop`, `Teddy Super`, `Roger Super`, `TS`, `RS`, `LTL`.
    - `632146H` add new aliases `Return of the Killing Machine`, `Fire Teddy Super`, `Fire Roger Super`, `FRS`, `FTS`, `ROTKM`.

### Fixed
- Fix **`Bridget`** missing images.
- Fix **`Faust`** `c.S` displaying super instead of `c.S` due to overlap of aliases.


## [0.6.4]

### Added
- Add Bridgets framedata/hitboxes.

### Fixed
- Update framedata.

## [0.6.3]

### Fixed
- Fix **`Chipp`** `Banki Messai` not displaying cause of changed input.
- Update framedata.

## [0.6.2]

### Fixed
- Update framedata.

## [0.6.1]

### Fixed
- Fix **`Testament`** `Greave Reapers` not displaying anything. 
- Fix missing move images and hitboxes for:
    - **`Chipp`**: 
        - `Wall Run P`.
        - `Wall Run K`.
        - `Wall Run S`.
        - `Wall Run SS`.
        - `Wall Run H`.
        - `Wall Run 6H`.

## [0.6.0]

### Changed
- Improve code regarding `init.json`.
- Make all commands case insensitive.
- Update [README.md](https://github.com/yakiimoninja/baiken#readme) with character specific stuff and minor adjustments.

### Added
- Add **`!`** as additional command prefix.
- Add easter egg interaction when `b.f` is used while other frame data bots are present on the same server.

- **`Nagoriyuki`**:
    - `f.S Level 1` add new alias `f.S`.
    - `f.S Level 2` add new alias `f.S2`.
    - `f.S Level 3` add new alias `f.S3`.
    - `f.S Level BR` add new alias `f.SB`.
    - `f.SS Level 1` add new alias `f.SS`.
    - `f.SS Level 2` add new alias `f.SS2`.
    - `f.SS Level 3` add new alias `f.SS3`.
    - `f.SS Level BR` add new alias `f.SSB`.
    - `f.SSS Level 1` add new alias `f.SSS`.
    - `f.SSS Level 2` add new alias `f.SSS2`.
    - `f.SSS Level 3` add new alias `f.SSS3`.
    - `f.SSS Level BR` add new alias `f.SSSB`.
    - `2S Level 1` add new alias `2S`.
    - `2S Level 2` add new alias `2S2`.
    - `2S Level 3` add new alias `2S3`.
    - `2S Level BR` add new alias `2SB`.

### Fixed
- Fix missing move image and hitboxes for:
    - **`Nagoriyuki`**: 
        - `Level 1`.
        - `fS Level 2`.
        - `fS Level 3`.
        - `fS Level BR`.
        - `fSS Level 1`.
        - `fSS Level 2`.
        - `fSS Level 3`.
        - `fSS Level BR`.
        - `fSSS Level 1`.
        - `fSSS Level 2`.
        - `fSSS Level 3`.
        - `fSSS Level BR`.
        - `2S Level 1`.
        - `2S Level 2`.
        - `2S Level 3`.
        - `2S Level BR`.

## [0.5.0]

### Changed
- **`Happy Chaos`**:
    - `Focus 214S H` change alias `Fire` to `Focus Fire`.

- **`Goldlewis Dickinson`**:
    - `236S Level 2` change alias `GL1` to `GL2`.

### Added
- **`Nagoriyuki`**:
    - `5H Level 1` add new alias `5H`.
    - `5H Level 2` add new alias `5H2`.
    - `5H Level 3` add new alias `5H3`.
    - `5H Level BR` add new alias `5HB`.
    - `2H Level 1` add new alias `2H`.
    - `2H Level 2` add new alias `2H2`.
    - `2H Level 3` add new alias `2H3`.
    - `2H Level BR` add new alias `2HB`.
    - `6H Level 1` add new alias `6H`.
    - `6H Level 2` add new alias `6H2`.
    - `6H Level 3` add new alias `6H3`.
    - `6H Level BR` add new alias `6HB`.
    - `j.H Level 1` add new alias `jH`.
    - `j.H Level 2` add new alias `jH2`.
    - `j.H Level 3` add new alias `jH3`.
    - `j.H Level BR` add new alias `jHB`.

- **`Goldlewis Dickinson`**:
    - `214S Level 1` add new alias `D1`.
    - `214S Level 2` add new aliases `Thunderbird 2`, `Drone 2`,`D2`. 
    - `214S Level 3` add new aliases `Thunderbird 3`, `Drone 3`,`D3`.
    - `236S Level 1` add new aliases `Minigun`, `MG1`, `G1`.
    - `236S Level 2` add new aliases `Skyfish 2`, `Minigun 2`, `MG2`, `Gun 2`, `G2`.
    - `236S Level 3` add new aliases `Skyfish 3`, `Minigun 3`, `MG3`, `Gun 3`, `G3`.
    - `720P` add new aliases `Down With The System 2`, `Super 2`.
    - `1080P` add new aliases `Down With The System 3`, `Super 3`.
    - `236236K Level 1` add new alias `L1`.
    - `236236K Level 2` add new aliases `Burn It Down 2`, `Laser 2`, `Laser Super 2`, `L2`.
    - `236236K Level 3` add new aliases `Burn It Down 3`, `Laser 3`, `Laser Super 3`, `L3`.

### Removed
- Remove patch notes directory and moved all the patch notes to the [releases](https://github.com/yakiimoninja/baiken/releases).
- Remove the command aliases `hit` and `hitbox` from `b.h`.

### Fixed 
- Fix `b.f` broken move image links for:
    - **`Nagoriyuki`**: `623H`, `623HH`.

## [0.4.5]

### Changed
- Change the embed line's color to match Baiken's profile picture. 
- Move `images` directory out of `src` and to `data`.

### Added 
- Option to invite Baiken to your server using the following QR Code. 
<img src="../data/images/baiken_qr.png" width="200" height="200"/>

### Fixed
- Fixed broken image links after the `images` directory move.

## [0.4.4]

### Added
- `Leo Whitefang`:
    - Autoguard attack `[s/h] h/s` add new aliases `s!h` and `h!s`.

- `Faust`:
    - `236S` add new alias `MMM`.
    - `j236S` add new aliases `AMMM`, `JMMM`.

### Fixed
- Fix `b.h` missing hitbox images for:
    - **`Axl`**: `2P`, `2S`, `2H`, `jD`.
    - **`Chipp`**: `236236K`.
    - **`Giovanna`**: `2D`.
    - **`Happy Chaos`**: `fS`.
    - **`Jacko`**: `fS`, `2H`, `jS`, `jH`, `2D`, `236K`.
    - **`Ky`**: `236K`.
    - **`May`**: `632146H`, `j632146H`.
    - **`Millia`**: `6P`.
    - **`Potemkin`**: `cS`.

## [0.4.3]

### Changed
- Change `b.f` message when move input is incorrect.

### Fixed
- Fix `b.a` not printing anything for Ky.
- Fix `b.a` command not coloring properly.

## [0.4.2]

### Changed
- Rename `patch_notes` directory to `patch-notes`.

### Added
- **`Sol Badguy`**: 
    - Add new nickname `Helios` (you can finally stop requesting it lol). 

- **`I-no`**:
    - `236H`: new alias `H Stroke`.

## [0.4.1]

### Changed
- Add new `b.?` / `b.help` message.

### Added
- Universal `throw`: new aliases `6D` and `4D`.

### Removed
- Unnecessary data from characters' json files.

- **`May`**:
    - `214P` add new alias `Beachball`.
    - `214K` add new alias `K Beachball`.
    - `623K` add new alias `OHK`.

- **`Testament`**:
    - `236*` : Change version indicator in aliases.
        - For partial charge, change from `@` to `!`.
        - For full charge, change from `!` to `!!`.

### Fixed
- Fix issue that did not allow to use `6D` or `4D`.

## [0.4.0]

### Added 

- Support for [nicknames.](https://github.com/yakiimoninja/baiken/blob/main/data/nicknames.json).
- Add new aliases for various moves.
- Add new patch notes page.


[1.0.0]: https://github.com/yakiimoninja/baiken/releases/tag/1.0.0
[0.30.1]: https://github.com/yakiimoninja/baiken/releases/tag/0.30.1
[0.30.0]: https://github.com/yakiimoninja/baiken/releases/tag/0.30.0
[0.29.0]: https://github.com/yakiimoninja/baiken/releases/tag/0.29.0
[0.28.2]: https://github.com/yakiimoninja/baiken/releases/tag/0.28.2
[0.28.1]: https://github.com/yakiimoninja/baiken/releases/tag/0.28.1
[0.28.0]: https://github.com/yakiimoninja/baiken/releases/tag/0.28.0
[0.27.1]: https://github.com/yakiimoninja/baiken/releases/tag/0.27.1
[0.27.0]: https://github.com/yakiimoninja/baiken/releases/tag/0.27.0
[0.26.0]: https://github.com/yakiimoninja/baiken/releases/tag/0.26.0
[0.25.1]: https://github.com/yakiimoninja/baiken/releases/tag/0.25.1
[0.25.0]: https://github.com/yakiimoninja/baiken/releases/tag/0.25.0
[0.24.0]: https://github.com/yakiimoninja/baiken/releases/tag/0.24.0
[0.23.0]: https://github.com/yakiimoninja/baiken/releases/tag/0.23.0
[0.22.0]: https://github.com/yakiimoninja/baiken/releases/tag/0.22.0
[0.21.0]: https://github.com/yakiimoninja/baiken/releases/tag/0.21.0
[0.20.1]: https://github.com/yakiimoninja/baiken/releases/tag/0.20.1
[0.20.0]: https://github.com/yakiimoninja/baiken/releases/tag/0.20.0
[0.19.2]: https://github.com/yakiimoninja/baiken/releases/tag/0.19.2
[0.19.1]: https://github.com/yakiimoninja/baiken/releases/tag/0.19.1
[0.19.0]: https://github.com/yakiimoninja/baiken/releases/tag/0.19.0
[0.18.0]: https://github.com/yakiimoninja/baiken/releases/tag/0.18.0
[0.17.0]: https://github.com/yakiimoninja/baiken/releases/tag/0.17.0
[0.16.0]: https://github.com/yakiimoninja/baiken/releases/tag/0.16.0
[0.15.0]: https://github.com/yakiimoninja/baiken/releases/tag/0.15.0
[0.14.0]: https://github.com/yakiimoninja/baiken/releases/tag/0.14.0
[0.13.0]: https://github.com/yakiimoninja/baiken/releases/tag/0.13.0
[0.12.0]: https://github.com/yakiimoninja/baiken/releases/tag/0.12.0
[0.11.0]: https://github.com/yakiimoninja/baiken/releases/tag/0.11.0
[0.10.6]: https://github.com/yakiimoninja/baiken/releases/tag/0.10.6
[0.10.5]: https://github.com/yakiimoninja/baiken/releases/tag/0.10.5
[0.10.4]: https://github.com/yakiimoninja/baiken/releases/tag/0.10.4
[0.10.3]: https://github.com/yakiimoninja/baiken/releases/tag/0.10.3
[0.10.2]: https://github.com/yakiimoninja/baiken/releases/tag/0.10.2
[0.10.1]: https://github.com/yakiimoninja/baiken/releases/tag/0.10.1
[0.10.0]: https://github.com/yakiimoninja/baiken/releases/tag/0.10.0
[0.9.0]: https://github.com/yakiimoninja/baiken/releases/tag/0.9.0
[0.8.5]: https://github.com/yakiimoninja/baiken/releases/tag/0.8.5
[0.8.4]: https://github.com/yakiimoninja/baiken/releases/tag/0.8.4
[0.8.3]: https://github.com/yakiimoninja/baiken/releases/tag/0.8.3
[0.8.2]: https://github.com/yakiimoninja/baiken/releases/tag/0.8.2
[0.8.1]: https://github.com/yakiimoninja/baiken/releases/tag/0.8.1
[0.8.0]: https://github.com/yakiimoninja/baiken/releases/tag/0.8.0
[0.7.6]: https://github.com/yakiimoninja/baiken/releases/tag/0.7.6
[0.7.5]: https://github.com/yakiimoninja/baiken/releases/tag/0.7.5
[0.7.4]: https://github.com/yakiimoninja/baiken/releases/tag/0.7.4
[0.7.3]: https://github.com/yakiimoninja/baiken/releases/tag/0.7.3
[0.7.2]: https://github.com/yakiimoninja/baiken/releases/tag/0.7.2
[0.7.1]: https://github.com/yakiimoninja/baiken/releases/tag/0.7.1
[0.7.0]: https://github.com/yakiimoninja/baiken/releases/tag/0.7.0
[0.6.4]: https://github.com/yakiimoninja/baiken/releases/tag/0.6.4
[0.6.3]: https://github.com/yakiimoninja/baiken/releases/tag/0.6.3
[0.6.2]: https://github.com/yakiimoninja/baiken/releases/tag/0.6.2
[0.6.1]: https://github.com/yakiimoninja/baiken/releases/tag/0.6.1
[0.6.0]: https://github.com/yakiimoninja/baiken/releases/tag/0.6.0
[0.5.0]: https://github.com/yakiimoninja/baiken/releases/tag/0.5.0
[0.4.5]: https://github.com/yakiimoninja/baiken/releases/tag/0.4.5
[0.4.4]: https://github.com/yakiimoninja/baiken/releases/tag/0.4.4
[0.4.3]: https://github.com/yakiimoninja/baiken/releases/tag/0.4.3
[0.4.2]: https://github.com/yakiimoninja/baiken/releases/tag/0.4.2
[0.4.1]: https://github.com/yakiimoninja/baiken/releases/tag/0.4.1
[0.4.0]: https://github.com/yakiimoninja/baiken/releases/tag/0.4.0
