# Baiken

<img src="https://user-images.githubusercontent.com/80072600/213743131-11012730-7bcd-4ab2-a6b4-e7406fdec419.jpg" />

[![](https://img.shields.io/static/v1?label=Sponsor&message=%E2%9D%A4&logo=GitHub&color=%23fe8e86)](https://github.com/sponsors/yakiimoninja)
[![GitHub tag](https://img.shields.io/github/tag/yakiimoninja/baiken.svg)](https://github.com/yakiimoninja/baiken/releases/latest)

### Your favorite one-handed samurai lady helps you learn Guilty Gear Strive by providing you with each character's moves, frame data and their hitboxes.
#

## Table of contents.
- [Patch notes](https://github.com/yakiimoninja/baiken/releases).
- [Inviting Baiken to a server](#inviting-the-bot-to-a-server).
- [Commands](#commands).
    - [Usage notes](#usage-notes).
    - [Nicknames](data/nicknames.json).
    - [Character specifics](#character-specifics).

## Patch notes.
- You can view the latest patch notes by pressing [**here**](https://github.com/yakiimoninja/baiken/releases).

## Inviting the bot to a server.
- Baiken can be **invited** to a server by pressing [**here**](https://discord.com/api/oauth2/authorize?client_id=919027797429727272&permissions=2147535872&scope=bot%20applications.commands).
- Or scanning the **QR Code** with a Camera or Discord application.

<img src="data/images/baiken_qr.png" width="250" height="250" />

## Commands.
### Both **`/`** and **`@Baiken`** can be used to invoke commands.
- **Command**: `/frames`. \
Displays the frame data of a move along with an image.
    <details>
        <summary>Show example.</summary>
            <p>
                <img src="https://user-images.githubusercontent.com/80072600/213763206-277c7615-66f2-4815-8e66-8da6f5fb70a9.png" />
            </p>
    </details>

- **Command**: `/fmeter`.\
Displays visually the startup, active and recovery frames of a character's move.
    <details>
        <summary>Show example.</summary>
        <p>
            <img src="https://user-images.githubusercontent.com/80072600/213763165-e7499889-9aba-4a98-9e3e-6bd12399e40e.png"/>
        </p>
    </details>

- **Command**: `/hitboxes`. \
Displays the hitbox images of a character's move.
    <details>
        <summary>Show example.</summary>
        <p>
            <img src="https://user-images.githubusercontent.com/80072600/213763221-f648e100-be22-4669-9d89-ad01dd0c2736.png"/>
        </p>
    </details>
  
- **Command**: `/moves`.\
Displays all the moves and inputs of a character.
    <details>
        <summary>Show example.</summary>
        <p>
            <img src="https://user-images.githubusercontent.com/80072600/213763223-bd3785a8-2070-44ce-93ab-9c8bd2abbc31.png"/>
        </p>
    </details>

- **Command**: `/aliases`.\
Displays all the aliases for each normal/special/super move of a character.
    <details>
        <summary>Show example.</summary>
        <p>
            <img src="https://user-images.githubusercontent.com/80072600/213761709-c08bda5a-4b0b-4d66-85b3-7bb6ef4b814d.png"/>
        </p>
    </details>

- **Command**: `/nicknames`.\
Displays all the nicknames for each character.
    <details>
        <summary>Show example.</summary>
        <p>
            <img src="https://user-images.githubusercontent.com/80072600/213763225-608c9a50-f8bb-4c6b-bbba-72b24082a6d3.png"/>
        </p>
    </details>

- **Command**: `/update`.\
**This command only works for owners.** \
Meaning that it requires an instance of the source code to use it. \
Updates the frame data and or image links for all or a specific character according to [**dustloop**](https://dustloop.com). \
    <details>
        <summary>Show example.</summary>
        <p>
            <img src="https://user-images.githubusercontent.com/80072600/213763235-1305c8cd-e58f-4200-a008-744f65df32d7.png"/>
        </p>
    </details>

- **Command**: `/register`.\
**This command only works for owners.** \
Registers or removes all slash commands in the current server or every server the bot is in.
    <details>
        <summary>Show example.</summary>
        <p>
            <img src="https://user-images.githubusercontent.com/80072600/213763229-8e7bc940-ac37-462c-b755-2c6ebd063221.png"/>
        </p>
    </details>

- **Command**: `/request`.\
Sends a request or feedback to the dev.
    <details>
        <summary>Show example.</summary>
        <p>
            <img src="https://user-images.githubusercontent.com/80072600/213763233-b40acb55-53d6-4916-a145-6b7d95c1e15d.png"/>
        </p>
    </details>

- **Command**: `/help`.\
Displays a help message. If used in conjunction with a command name, `notes` or `specifics` a different message wil be displayed.
    <details>
        <summary>Show example.</summary>
        <p>
            <img src="https://user-images.githubusercontent.com/80072600/213763210-50dce454-5195-467a-8284-cdd6acdf9da5.png"/>
            <img src="https://user-images.githubusercontent.com/80072600/213763215-8dba5d56-5060-4e8f-9808-090bb1517484.png"/>
        </p>
    </details>

## Usage notes.

- **`/` commands can be substituted with direct mentions if preferred.**
    - Doing so will enable the use of shorthand commands.
        - Example: `@Baiken f sol 2k` same as `/frames sol 2k`.
        - Example: `@Baiken h ky 6p` same as `/hitboxes ky 6p`.
        - Example: `@Baiken m leo` same as `/moves leo`.
        - Example: `@Baiken a chipp` same as `/aliases chipp`.

- **All searching is case insensitive.**
  - All names, nicknames, moves and aliases are case agnostic.
  - Example: `/hitboxes ky dp` = `/hitboxes KY dP`.

- **Character searching.**
  - Characters can be found either using a part of their name, or any of the [nicknames](https://github.com/yakiimoninja/baiken/blob/main/data/nicknames.json) that exist.
  - Example: `/frames Happy Chaos cs` = `/frames happy cs` = `/frames hc cs`.

- **Move searching.**
   - Moves can be found either using a part of their name, their input, or any of the aliases that exist.
      - Example: `/frames Anji Needles` = `/frames Anji 236HP` = `/frames Anji ichi`.
   - Charged moves can be found with or without the use of `[]`.
      - Example `/frames may 46S` = `/frames may [4]6S`.
   - For a fully charged dust attack the alias `5D!` can be used instead.
      - Example: `/frames chipp 5D!`.

## Character specifics.
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
  - The above is not always the case depending on the special move and alias used.
  - For Level 1 `Thunderbird`: `/frames gold D1`.
  - For Level 2 `Thunderbird`: `/frames gold D2`.
  - For Level 3 `Thunderbird`: `/frames gold D3`.
  - See `/aliases gold` for more info on his aliases.

- **For Testament's different Grave Reaper versions use as shown.**
  - Regular version: `/frames testament 236S`.
  - Partially charged version: `/frames testament 236S!`.
  - Fully charged version: `/frames testament 236S!!`.
#
### Help support the project.
[![](https://img.shields.io/static/v1?label=Sponsor&message=%E2%9D%A4&logo=GitHub&color=%23fe8e86)](https://github.com/sponsors/yakiimoninja)
### Baiken artwork: [@gogalking](https://twitter.com/gogalking/status/1307199393607553024).
