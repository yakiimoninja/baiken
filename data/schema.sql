PRAGMA foreign_keys = ON;

CREATE TABLE IF NOT EXISTS "characters" (
	"id" INTEGER NOT NULL UNIQUE,
	"character" TEXT NOT NULL UNIQUE,
	PRIMARY KEY ("id")
);

CREATE TABLE IF NOT EXISTS "nicknames" (
	"id" INTEGER NOT NULL UNIQUE,
	"character_id" INTEGER NOT NULL,
	"nickname" TEXT NOT NULL,
	PRIMARY KEY ("id"),
	FOREIGN KEY ("character_id") REFERENCES "characters"("id")
	ON UPDATE NO ACTION ON DELETE NO ACTION,
    UNIQUE ("character_id", "nickname")
);

CREATE TABLE IF NOT EXISTS "moves" (
	"id" INTEGER NOT NULL UNIQUE,
	"character_id" INTEGER NOT NULL,
	"input" TEXT NOT NULL,
	"name" TEXT,
	"damage" TEXT,
	"guard" TEXT,
	"startup" TEXT,
	"active" TEXT,
	"recovery" TEXT,
	"on_hit" TEXT,
	"on_block" TEXT,
	"level" TEXT,
	"counter" TEXT,
	"move_type" TEXT,
	"risc_gain" TEXT,
	"risc_loss" TEXT,
	"wall_damage" TEXT,
	"input_tension" TEXT,
	"chip_ratio" TEXT,
	"otg_ratio" TEXT,
	"scaling" TEXT,
	"invincibility" TEXT,
	"cancel" TEXT,
	"caption" TEXT,
	"notes" TEXT,
	"image" TEXT,
	PRIMARY KEY ("id"),
	FOREIGN KEY ("character_id") REFERENCES "characters"("id")
	ON UPDATE NO ACTION ON DELETE NO ACTION,
    UNIQUE ("character_id", "input")
);

CREATE TABLE IF NOT EXISTS "hitboxes" (
	"id" INTEGER NOT NULL UNIQUE,
    "move_id" INTEGER NOT NULL,
	"hitbox" TEXT,
	"hitbox_caption" TEXT,
	PRIMARY KEY ("id"),
    FOREIGN KEY ("move_id") REFERENCES "moves"("id")
	ON UPDATE NO ACTION ON DELETE NO ACTION,
    UNIQUE ("move_id", "hitbox")
);

CREATE TABLE IF NOT EXISTS "aliases" (
	"id" INTEGER NOT NULL UNIQUE,
    "move_id" INTEGER NOT NULL,
	"alias" TEXT NOT NULL,
	PRIMARY KEY ("id"),
    FOREIGN KEY ("move_id") REFERENCES "moves"("id")
	ON UPDATE NO ACTION ON DELETE NO ACTION,
    UNIQUE ("move_id", "alias")
);

CREATE TABLE IF NOT EXISTS "info" (
	"id" INTEGER NOT NULL UNIQUE,
	"character_id" INTEGER NOT NULL UNIQUE,
	"defense" TEXT,
	"guts" TEXT,
	"guard_balance" TEXT,
	"prejump" TEXT,
	"umo" TEXT,
	"forward_dash" TEXT,
	"backdash" TEXT,
	"backdash_duration" TEXT,
	"backdash_invincibility" TEXT,
	"backdash_airborne" TEXT,
	"backdash_distance" TEXT,
	"jump_duration" TEXT,
	"jump_height" TEXT,
	"high_jump_duration" TEXT,
	"high_jump_height" TEXT,
	"earliest_iad" TEXT,
	"ad_duration" TEXT,
	"ad_distance" TEXT,
	"abd_duration" TEXT,
	"abd_distance" TEXT,
	"movement_tension" TEXT,
	"jump_tension" TEXT,
	"airdash_tension" TEXT,
	"walk_speed" TEXT,
	"back_walk_speed" TEXT,
	"dash_initial_speed" TEXT,
	"dash_acceleration" TEXT,
	"dash_friction" TEXT,
	"jump_gravity" TEXT,
	"high_jump_gravity" TEXT,
	PRIMARY KEY ("id"),
	FOREIGN KEY ("character_id") REFERENCES "characters"("id")
	ON UPDATE NO ACTION ON DELETE NO ACTION
);
