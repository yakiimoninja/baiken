# Baiken
**Your favorite one-handed samurai lady helps you learn Guilty Gear Strive \
by providing you with each character's moves, frame data and their hitboxes.**


## Table of contents
  - [Patch notes.](https://github.com/yakiimoninja/baiken/releases)
  - [Invite Baiken to your server.](#invite-the-bot-to-your-server)
  - [Host Baiken yourself.](#host-the-bot-yourself)
  - [Commands.](#commands)
    - [Usage notes.](#usage-notes)
    - [Nicknames.](#you-can-use-character-nicknames-when-searching)
    - [Character specifics.](#character-specifics)

## Patch notes
- You can view the latest patch notes by pressing [**here**](https://github.com/yakiimoninja/baiken/releases).


## Invite the bot to your server
- You can **invite** Baiken to your server by pressing [**here**](https://discord.com/api/oauth2/authorize?client_id=919027797429727272&permissions=52224&scope=bot).
- Or scan the **QR Code** with your Camera or Discord application.

<img src="data/images/baiken_qr.png" width="250" height="250" />


## Host the bot yourself
- You will need a bot token.
- You will need to **download** and **compile** the source code.
- You will need to create a `.env` file and put that token inside that.
- You will also need to have the `data` folder and the `.env` file in the same folder as `baiken.exe`.
- Lastly execute the compiled code.


## Commands
### Both **`b.`** and **`!`** can be used as prefixes.
- **Command**: `b.f` or `b.frames`. \
Displays the frame data of a move along with an image.\
Example: `b.f baiken 236K` or `b.frames baiken 236K`.
  
- **Command**: `b.h` or `b.hitboxes`. \
Displays the hitbox images of a move. \
Example: `b.h goldlewis 426H` or `b.hitboxes goldlewis 426H`.
  
- **Command**: `b.m` or `b.moves`.\
Displays all the moves and inputs of a character.\
Example: `b.m sol` or `b.moves sol`.

- **Command**: `b.a` or `b.aliases`.\
Displays all the aliases for each normal/special/super move of a character.\
Example: `b.a leo` or `b.moves leo`.

- **Command**: `b.u` or `b.update`.\
**This command only works for owners.** \
Meaning that you need to run your own instance of the source code to use it. \
Updates the frame data according to [**dustloop**](https://dustloop.com).

- **Command**: `b.?` or `b.help`.\
Displays a help message.  

## Usage notes

 - ### You can substitute the prefix **`b.`** with **`!`** if you prefer.
   - Example: `!f baiken 2k`.

- **Searching is case insensitive.**
  - Example: `b.h ky dp`.

- **You don't have to search using the full name of a character.**
  - Example: `b.f gio 236K`.

- #### **You can use character nicknames when searching.**
  - Example: `b.moves hc`.
  - [List of nicknames.](data/nicknames.json)

- **You can also use aliases instead of inputs when searching for moves.**
  - Example: `b.f Anji Needles`.

- **You can omit the `[]` when searching for a charged move.**
  - Example: `b.frames May [4]6S`.
  - Example: `b.f may 46S`.

- **When searching for charged dust attack use the alias `5D!`.**
  - Example: `b.f chipp 5D!`.

## Character specifics
- **For normals that have levels like Nagoriyuki.**
  - Add the level number next to the normal.
  - For Level 1 `fS`: `b.f nago fs`. 
  - For Level 2 `fS`: `b.f nago fs2`.
  - For Level 3 `fS`: `b.f nago fs3`.
  - If it's a level 1 normal nothing needs to be added since it's the default state.

- **For specials that have levels like Goldlewis.**
  - Add the level number next to the special.
  - For Level 1 `Thunderbird`: `b.f gold Drone`.
  - For Level 2 `Thunderbird`: `b.f gold Drone 2`.
  - For Level 3 `Thunderbird`: `b.f gold Drone 3`.
  - The above is not always the case depending on the special move and alias you use.
  - For Level 1 `Thunderbird`: `b.f gold D1`.
  - For Level 2 `Thunderbird`: `b.f gold D2`.
  - For Level 3 `Thunderbird`: `b.f gold D3`.
  - See `b.a gold` for more info on his aliases.

- **For Testament's different Grave Reaper versions use as shown.**
  - Regular version: `b.f testament 236S`.
  - Partially charged version: `b.f testament 236S!`.
  - Fully charged version: `b.f testament 236S!!`.

#

### Credits for the Baiken artwork: [@gogalking](https://twitter.com/gogalking/status/1307199393607553024).


