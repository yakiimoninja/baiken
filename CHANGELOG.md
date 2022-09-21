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
 - Added additional prefix for Baiken. Now both **`b.`** and **`!`** work as prefixes.
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
