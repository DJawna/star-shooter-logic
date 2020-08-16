BEGIN TRANSACTION;
CREATE TABLE IF NOT EXISTS "global_controls" (
	"inputName"	TEXT NOT NULL,
	"value"	INTEGER NOT NULL,
	"defaultValue"	INTEGER NOT NULL,
	PRIMARY KEY("inputName"),
	FOREIGN KEY("defaultValue") REFERENCES "global_keymappings"("keyCode"),
	FOREIGN KEY("value") REFERENCES "global_keymappings"("keyCode")
);
CREATE TABLE IF NOT EXISTS "global_keymappings" (
	"keyCode"	INTEGER NOT NULL,
	"keyName"	TEXT NOT NULL,
	PRIMARY KEY("keyCode")
);
CREATE TABLE IF NOT EXISTS "global_audiosettings" (
	"audioSetting"	TEXT NOT NULL,
	"value"	INTEGER NOT NULL CHECK(value<=100 and value>=0),
	"defaultValue"	INTEGER NOT NULL CHECK(defaultValue<=100 and defaultValue>=0),
	PRIMARY KEY("audioSetting")
);
CREATE TABLE IF NOT EXISTS "global_resolution_settings" (
	"settingname"	TEXT NOT NULL,
	"value"	INTEGER NOT NULL CHECK(value>=0),
	"defaultValue"	INTEGER NOT NULL CHECK(defaultValue>=0),
	PRIMARY KEY("settingname")
);
CREATE TABLE IF NOT EXISTS "global_screens" (
	"screen_id"	INTEGER NOT NULL,
	"screen_name"	TEXT NOT NULL,
	PRIMARY KEY("screen_id")
);
CREATE TABLE IF NOT EXISTS "global_gui_elements" (
	"screen_id"	INTEGER NOT NULL,
	"id"	INTEGER NOT NULL,
	FOREIGN KEY("screen_id") REFERENCES "global_screen"("screen_id"),
	PRIMARY KEY("screen_id","id")
);
INSERT INTO "global_controls" ("inputName","value","defaultValue") VALUES ('up',1,1),
 ('left',2,2),
 ('right',3,3),
 ('down',4,4),
 ('fire',5,5),
 ('special_item',6,6);
INSERT INTO "global_keymappings" ("keyCode","keyName") VALUES (1,'cursor_up'),
 (2,'cursor_left'),
 (3,'cursor_right'),
 (4,'cursor_down'),
 (5,'space'),
 (6,'control');
INSERT INTO "global_audiosettings" ("audioSetting","value","defaultValue") VALUES ('SoundEffectsVolume',100,100),
 ('musicVolume',100,100),
 ('masterVolume',100,100);
INSERT INTO "global_resolution_settings" ("settingname","value","defaultValue") VALUES ('resX',800,800),
 ('resY',600,600);
COMMIT;
