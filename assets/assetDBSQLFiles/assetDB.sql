BEGIN TRANSACTION;
CREATE TABLE IF NOT EXISTS "Tiles" (
	"ID"	INTEGER NOT NULL,
	"TextureID"	INTEGER NOT NULL,
	"TextureX"	BLOB NOT NULL,
	"TextureY"	INTEGER NOT NULL,
	"TextureWidth"	INTEGER NOT NULL,
	"TextureHeight"	INTEGER NOT NULL,
	"CanFlyOver"	INTEGER NOT NULL,
	"width"	INTEGER NOT NULL,
	"height"	INTEGER NOT NULL,
	"TilemapID"	INTEGER NOT NULL,
	PRIMARY KEY("ID","TilemapID")
);
CREATE TABLE IF NOT EXISTS "TileMap" (
	"ID"	INTEGER NOT NULL,
	"LevelID"	INTEGER NOT NULL,
	FOREIGN KEY("LevelID") REFERENCES "global_Levels"("id"),
	PRIMARY KEY("ID","LevelID")
);
CREATE TABLE IF NOT EXISTS "global_Levels" (
	"id"	INTEGER NOT NULL,
	"Name"	TEXT NOT NULL,
	PRIMARY KEY("id")
);
CREATE TABLE IF NOT EXISTS "global_gui_elements" (
	"screen_id"	INTEGER NOT NULL,
	"id"	INTEGER NOT NULL,
	PRIMARY KEY("screen_id","id"),
	FOREIGN KEY("screen_id") REFERENCES "global_screen"("screen_id")
);
CREATE TABLE IF NOT EXISTS "global_screens" (
	"screen_id"	INTEGER NOT NULL,
	"screen_name"	TEXT NOT NULL,
	PRIMARY KEY("screen_id")
);
CREATE TABLE IF NOT EXISTS "global_resolution_settings" (
	"settingname"	TEXT NOT NULL,
	"value"	INTEGER NOT NULL CHECK(value>=0),
	"defaultValue"	INTEGER NOT NULL CHECK(defaultValue>=0),
	PRIMARY KEY("settingname")
);
CREATE TABLE IF NOT EXISTS "global_audiosettings" (
	"audioSetting"	TEXT NOT NULL,
	"value"	INTEGER NOT NULL CHECK(value<=100 and value>=0),
	"defaultValue"	INTEGER NOT NULL CHECK(defaultValue<=100 and defaultValue>=0),
	PRIMARY KEY("audioSetting")
);
CREATE TABLE IF NOT EXISTS "global_keymappings" (
	"keyCode"	INTEGER NOT NULL,
	"keyName"	TEXT NOT NULL,
	PRIMARY KEY("keyCode")
);
CREATE TABLE IF NOT EXISTS "global_controls" (
	"inputName"	TEXT NOT NULL,
	"value"	INTEGER NOT NULL,
	"defaultValue"	INTEGER NOT NULL,
	FOREIGN KEY("value") REFERENCES "global_keymappings"("keyCode"),
	FOREIGN KEY("defaultValue") REFERENCES "global_keymappings"("keyCode"),
	PRIMARY KEY("inputName")
);
INSERT INTO "global_screens" ("screen_id","screen_name") VALUES (1,'Start'),
 (2,'Highscore'),
 (3,'Continue'),
 (4,'OptionsMenu'),
 (5,'ConfirmNewResolution'),
 (6,'Level'),
 (7,'Pause');
INSERT INTO "global_resolution_settings" ("settingname","value","defaultValue") VALUES ('resX',800,800),
 ('resY',600,600);
INSERT INTO "global_audiosettings" ("audioSetting","value","defaultValue") VALUES ('SoundEffectsVolume',100,100),
 ('musicVolume',100,100),
 ('masterVolume',100,100);
INSERT INTO "global_keymappings" ("keyCode","keyName") VALUES (1,'cursor_up'),
 (2,'cursor_left'),
 (3,'cursor_right'),
 (4,'cursor_down'),
 (5,'space'),
 (6,'control');
INSERT INTO "global_controls" ("inputName","value","defaultValue") VALUES ('up',1,1),
 ('left',2,2),
 ('right',3,3),
 ('down',4,4),
 ('fire',5,5),
 ('special_item',6,6);
COMMIT;
