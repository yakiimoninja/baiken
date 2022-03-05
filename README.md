# Baiken
**Your favorite one-handed samurai lady helps you learn Guilty Gear Strive \
by providing you with each character's moves, frame data and their hitboxes.**

## Requirements
- You will need a bot token.
- You will need to download the exe from [here](https://github.com/yakiimoninja/baiken/releases/latest).
- You will put it that token inside the `.env` file.
- You will also need to have `data` folder and `.env` file in the same folder as `baiken-bot.exe`.

## Usage
- Either open the `baiken-bot.exe` itself or run it in the command prompt.
- ### Lastly the bot **commands**:
  
  - **Command**: `b.f` or `b.frames`. \
  Displays the frame data of a move.\
  Use `b.f` or `b.frames` followed by the `character` and the `move` you want.\
  Example: `b.f baiken 236K` or `b.frames baiken 236K`.
  
  - **Command**: `b.h` or `b.hitbox`. \
  Displays the hitbox images of a move. \
  Use `b.h` or `b.hitbox` followed by the `character` and the `move` you want. \
  Example: `b.h goldlewis 4126H` or `b.hitbox goldlewis 4126H`.
  
  - **Command**: `b.m` or `b.moves`.\
  Displays all the moves and their inputs of a character.\
  Use `b.m` or `b.moves` followed by the `character` you want.\
  Example: `b.m sol` or `b.moves sol`.

  - **Command**: `b.a` or `b.aliases`.\
  Displays all the aliases for each special/super move of a character.\
  Use `b.a` or `b.aliases` followed by the `character` you want.\
  Example: `b.a leo` or `b.moves leo`.

  - **Command**: `b.u` or `b.update`.\
  Updates the frame data according to [dustloop](https://dustloop.com).

  - **Command**: `b.h` or `b.help` or `b.?`.\
  Displays this help message.
  

- ### **Notes**

- **You don't have to write the full name of a character.**
  - This works: `b.frames giovanna 236K`.
  - This will also work: `b.f gio 236K`.

- **You can also use aliases when searching for moves.**
  - This works: `b.frames nagoriyuki 214H`.
  - This also works: `b.f nagoriyuki beyblade`.

- **When searching for charged dust attack use the alias `5D!`.**
  - Example: `b.f chipp 5D!`.

- **Searching is case insensitive.**
  - This works: `b.hitbox Ky DP`.
  - This also works: `b.h ky dp`.
  # 
 ### That's all folks!
