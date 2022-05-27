# Baiken
**Your favorite one-handed samurai lady helps you learn Guilty Gear Strive \
by providing you with each character's moves, frame data and their hitboxes.**


## Table of contents
  - [Patch notes.](patch-notes/)
  - [Invite Baiken to your server.](#invite-the-bot-to-your-server)
  - [Host Baiken yourself.](#host-the-bot-yourself)
  - [Commands.](#commands)
  - [Usage notes.](#usage-notes)

## Patch notes
- You can view the latest patch notes by pressing [**here**](patch-notes/).


## Invite the bot to your server
- You can **invite** Baiken to your server by pressing [**here**](https://discord.com/api/oauth2/authorize?client_id=919027797429727272&permissions=517544070208&scope=bot).


## Host the bot yourself
- You will need a bot token.
- You will need to **download** and **compile** the source code.
- You will need to create a `.env` file and put that token inside that.
- You will also need to have the `data` folder and the `.env` file in the same folder as `baiken.exe`.
- Lastly execute the compiled code.


## Commands

- **Command**: `b.f` or `b.frames`. \
Displays the frame data of a move along with an image.\
Example: `b.f baiken 236K` or `b.frames baiken 236K`.
  
- **Command**: `b.h` or `b.hitbox`. \
Displays the hitbox images of a move. \
Example: `b.h goldlewis 426H` or `b.hitbox goldlewis 426H`.
  
- **Command**: `b.m` or `b.moves`.\
Displays all the moves and inputs of a character.\
Example: `b.m sol` or `b.moves sol`.

- **Command**: `b.a` or `b.aliases`.\
Displays all the aliases for each special/super move of a character.\
Example: `b.a leo` or `b.moves leo`.

- **Command**: `b.u` or `b.update`.\
Downloads and updates new frame data from [**dustloop**](https://dustloop.com).

- **Command**: `b.?` or `b.help`.\
Displays a help message.  

## Usage notes

- **Searching is case insensitive.**
  - This works: `b.hitbox Ky DP`.
  - This also works: `b.h ky dp`.

- **You don't have to search using the full name of a character.**
  - This works: `b.frames giovanna 236K`.
  - This will also work: `b.f gio 236K`.

- **You can use character nicknames when searching.**
  - This works: `b.moves hc`.
  - This also works: `b.m Chaos`.  

- **You can also use aliases when searching for moves.**
  - This works: `b.frames Anji 236HP`.
  - This also works: `b.f Anji Needles`.

- **You can omit the `[]` when searching for a charged move.**
  - This works: `b.frames May [4]6S`.
  - This also works: `b.f may 46S`.

- **For Testament's different Grave Reaper versions use as shown.**
  - Regular version: `b.f testament 236S`.
  - Partially charged version: `b.f testament 236S!`.
  - Fully charged version: `b.f testament 236S!!`.

- **When searching for charged dust attack use the alias `5D!`.**
  - Example: `b.f chipp 5D!`.

#

### Credits for the Baiken artwork: [@gogalking](https://twitter.com/gogalking).
