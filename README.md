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
- You can **invite** Baiken to your server by pressing [**here**](https://discord.com/api/oauth2/authorize?client_id=919027797429727272&permissions=2147535872&scope=bot%20applications.commands).
- Or scan the **QR Code** with your Camera or Discord application.

<img src="data/images/baiken_qr.png" width="250" height="250" />


## Host the bot yourself
- You will need a bot token.
- You will need to **download** and **compile** the source code.
- You will need to create a `.env` file and put that token inside that.
- You will also need to have the `data` folder and the `.env` file in the same folder as `baiken.exe`.
- Lastly execute the compiled code.


## Commands
### Both **`/`** and **`Mentioning Baiken`** can be used to invoke commands.
- **Command**: `/frames`. \
Displays the frame data of a move along with an image.\
Example: `/frames baiken 236K`.
  
- **Command**: `/hitboxes`. \
Displays the hitbox images of a character's move. \
Example: `/hitboxes goldlewis 426H`.
  
- **Command**: `/moves`.\
Displays all the moves and inputs of a character.\
Example: `/moves sol`.

- **Command**: `/aliases`.\
Displays all the aliases for each normal/special/super move of a character.\
Example: `/aliases leo`.

- **Command**: `/update`.\
**This command only works for owners.** \
Meaning that you need to run your own instance of the source code to use it. \
Updates the frame data according to [**dustloop**](https://dustloop.com).

- **Command**: `/request`.\
Sends a request or feedback to the dev.

- **Command**: `/help`.\
Displays a help message.  

## Usage notes

 - ### You can substitute **`/`** commands with direct mentions if you prefer.
   - Doing so will enable you to use shorthand commands.
      - Example: `@Baiken f sol 2k` same as `/frames sol 2k`.
      - Example: `@Baiken h ky 6p` same as `/hitboxes ky 6p`.
      - Example: `@Baiken m leo` same as `/moves leo`.
      - Example: `@Baiken a chipp` same as `/aliases chipp`.


- **Searching is case insensitive.**
  - Example: `/hitboxes ky dp`.

- **You don't have to search using the full name of a character.**
  - Example: `/frames gio 236K`.

- #### **You can use character nicknames when searching.**
  - Example: `/moves hc`.
  - [List of nicknames.](data/nicknames.json)

- **You can also use aliases instead of inputs when searching for moves.**
  - Example: `/frames Anji Needles`.

- **You can omit the `[]` when searching for a charged move.**
  - Example: `/frames May [4]6S`.
  - Example: `/frames may 46S`.

- **When searching for charged dust attack use the alias `5D!`.**
  - Example: `/frames chipp 5D!`.

## Character specifics
- **For normals that have levels like Nagoriyuki.**
  - Add the level number next to the normal.
  - For Level 1 `fS`: `/frames nago fs`. 
  - For Level 2 `fS`: `/frames nago fs2`.
  - For Level 3 `fS`: `/frames nago fs3`.
  - If it's a level 1 normal nothing needs to be added since it's the default state.

- **For specials that have levels like Goldlewis.**
  - Add the level number next to the special.
  - For Level 1 `Thunderbird`: `/frames gold Drone`.
  - For Level 2 `Thunderbird`: `/frames gold Drone 2`.
  - For Level 3 `Thunderbird`: `/frames gold Drone 3`.
  - The above is not always the case depending on the special move and alias you use.
  - For Level 1 `Thunderbird`: `/frames gold D1`.
  - For Level 2 `Thunderbird`: `/frames gold D2`.
  - For Level 3 `Thunderbird`: `/frames gold D3`.
  - See `/aliases gold` for more info on his aliases.

- **For Testament's different Grave Reaper versions use as shown.**
  - Regular version: `/frames testament 236S`.
  - Partially charged version: `/frames testament 236S!`.
  - Fully charged version: `/frames testament 236S!!`.

#

### Credits for the Baiken artwork: [@gogalking](https://twitter.com/gogalking/status/1307199393607553024).


