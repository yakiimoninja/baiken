PRAGMA foreign_keys = ON;
-- regex /[a-zA-Z0-9]+/g
-- select * from characters where instr(lower(character), lower('?1'));
-- select * from move_data right join characters on move_data.character_id=characters.id where move_data.character_id=1 order by id;
-- case insensiitive column indexing query performance increase OPEN LINK WITH vi'gx 
-- 'https://www.youtube.com/watch?v=Wd5WWVx3aRE&t=456s'

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
	"character_id" INTEGER NOT NULL,
	"input" TEXT NOT NULL,
	"hitbox" TEXT,
	"hitbox_caption" TEXT,
	PRIMARY KEY ("id"),
	FOREIGN KEY ("character_id", "input") REFERENCES "moves"("character_id", "input")
	ON UPDATE NO ACTION ON DELETE NO ACTION,
    UNIQUE ("character_id", "input")
);

CREATE TABLE IF NOT EXISTS "aliases" (
	"id" INTEGER NOT NULL UNIQUE,
	"character_id" INTEGER NOT NULL,
	"input" TEXT NOT NULL,
	"alias" TEXT NOT NULL,
	PRIMARY KEY ("id"),
	FOREIGN KEY ("character_id", "input") REFERENCES "moves"("character_id", "input")
	ON UPDATE NO ACTION ON DELETE NO ACTION,
    UNIQUE ("character_id", "input")
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

INSERT INTO characters (character)
VALUES
('ABA'),
('Anji Mito'),
('Asuka R'),
('Axl Low'),
('Baiken'),
('Bedman'),
('Bridget'),
('Chipp Zanuff'),
('Elphelt Valentine'),
('Faust'),
('Giovanna'),
('Goldlewis Dickinson'),
('Happy Chaos'),
('Ino'),
('Jacko'),
('Johnny'),
('Ky Kiske'),
('Leo Whitefang'),
('May'),
('Millia Rage'),
('Nagoriyuki'),
('Potemkin'),
('Queen Dizzy'),
('Ramlethal Valentine'),
('Sin Kiske'),
('Slayer'),
('Sol Badguy'),
('Testament'),
('Zato1');

INSERT INTO nicknames (character_id, nickname)
VALUES
((SELECT characters.id FROM characters WHERE characters.character = 'ABA'), 'A.B.A'),
((SELECT characters.id FROM characters WHERE characters.character = 'ABA'), 'ABA'),
((SELECT characters.id FROM characters WHERE characters.character = 'ABA'), 'Paracelsus'),
((SELECT characters.id FROM characters WHERE characters.character = 'Anji Mito'), 'Anji Mito'),
((SELECT characters.id FROM characters WHERE characters.character = 'Anji Mito'), 'AM'),
((SELECT characters.id FROM characters WHERE characters.character = 'Anji Mito'), 'Lanji'),
((SELECT characters.id FROM characters WHERE characters.character = 'Asuka R'), 'Asuka R'),
((SELECT characters.id FROM characters WHERE characters.character = 'Asuka R'), 'That Man'),
((SELECT characters.id FROM characters WHERE characters.character = 'Asuka R'), 'Oscar'),
((SELECT characters.id FROM characters WHERE characters.character = 'Axl Low'), 'AL'),
((SELECT characters.id FROM characters WHERE characters.character = 'Axl Low'), 'Brit'),
((SELECT characters.id FROM characters WHERE characters.character = 'Axl Low'), 'British'),
((SELECT characters.id FROM characters WHERE characters.character = 'Baiken'), 'Bacon'),
((SELECT characters.id FROM characters WHERE characters.character = 'Bedman'), 'Bedman?'),
((SELECT characters.id FROM characters WHERE characters.character = 'Bedman'), 'Bedwoman'),
((SELECT characters.id FROM characters WHERE characters.character = 'Bedman'), 'Bed'),
((SELECT characters.id FROM characters WHERE characters.character = 'Bedman'), 'Delilah'),
((SELECT characters.id FROM characters WHERE characters.character = 'Bridget'), 'Brisket'),
((SELECT characters.id FROM characters WHERE characters.character = 'Chipp Zanuff'), 'Chipp Zanuff'),
((SELECT characters.id FROM characters WHERE characters.character = 'Chipp Zanuff'), 'CZ'),
((SELECT characters.id FROM characters WHERE characters.character = 'Chipp Zanuff'), 'Ninja'),
((SELECT characters.id FROM characters WHERE characters.character = 'Elphelt Valentine'), 'EV'),
((SELECT characters.id FROM characters WHERE characters.character = 'Elphelt Valentine'), 'Die'),
((SELECT characters.id FROM characters WHERE characters.character = 'Faust'), 'Laust'),
((SELECT characters.id FROM characters WHERE characters.character = 'Giovanna'), 'Gio'),
((SELECT characters.id FROM characters WHERE characters.character = 'Goldlewis Dickinson'), 'Goldick Lewison'),
((SELECT characters.id FROM characters WHERE characters.character = 'Goldlewis Dickinson'), 'GD'),
((SELECT characters.id FROM characters WHERE characters.character = 'Goldlewis Dickinson'), 'GL'),
((SELECT characters.id FROM characters WHERE characters.character = 'Happy Chaos'), 'HC'),
((SELECT characters.id FROM characters WHERE characters.character = 'Happy Chaos'), 'Asshole'),
((SELECT characters.id FROM characters WHERE characters.character = 'Ino'), 'I-no'),
((SELECT characters.id FROM characters WHERE characters.character = 'Ino'), 'Witch'),
((SELECT characters.id FROM characters WHERE characters.character = 'Jacko'), 'Jack-O'),
((SELECT characters.id FROM characters WHERE characters.character = 'Jacko'), 'Jack-O Valentine'),
((SELECT characters.id FROM characters WHERE characters.character = 'Johnny'), 'Lean'),
((SELECT characters.id FROM characters WHERE characters.character = 'Ky Kiske'), 'KK'),
((SELECT characters.id FROM characters WHERE characters.character = 'Ky Kiske'), 'Kyle'),
((SELECT characters.id FROM characters WHERE characters.character = 'Leo Whitefang'), 'LW'),
((SELECT characters.id FROM characters WHERE characters.character = 'Leo Whitefang'), 'Gorilla'),
((SELECT characters.id FROM characters WHERE characters.character = 'May'), 'Totsugeki'),
((SELECT characters.id FROM characters WHERE characters.character = 'Millia Rage'), 'MR'),
((SELECT characters.id FROM characters WHERE characters.character = 'Nagoriyuki'), 'Vampire'),
((SELECT characters.id FROM characters WHERE characters.character = 'Potemkin'), 'Pot'),
((SELECT characters.id FROM characters WHERE characters.character = 'Queen Dizzy'), 'QD'),
((SELECT characters.id FROM characters WHERE characters.character = 'Ramlethal Valentine'), 'RV'),
((SELECT characters.id FROM characters WHERE characters.character = 'Ramlethal Valentine'), 'Borgar'),
((SELECT characters.id FROM characters WHERE characters.character = 'Sin Kiske'), 'SK'),
((SELECT characters.id FROM characters WHERE characters.character = 'Slayer'), 'Dandy'),
((SELECT characters.id FROM characters WHERE characters.character = 'Sol Badguy'), 'SB'),
((SELECT characters.id FROM characters WHERE characters.character = 'Sol Badguy'), 'Frederick'),
((SELECT characters.id FROM characters WHERE characters.character = 'Sol Badguy'), 'Helios'),
((SELECT characters.id FROM characters WHERE characters.character = 'Sol Badguy'), 'Sol Goodman'),
((SELECT characters.id FROM characters WHERE characters.character = 'Testament'), 'Testie'),
((SELECT characters.id FROM characters WHERE characters.character = 'Zato1'), 'Zato-1'),
((SELECT characters.id FROM characters WHERE characters.character = 'Zato1'), 'Zato One'),
((SELECT characters.id FROM characters WHERE characters.character = 'Zato1'), 'Zato 1'),
((SELECT characters.id FROM characters WHERE characters.character = 'Zato1'), 'Eddie');
