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
INSERT INTO "global_keymappings" ("keyName","keyCode") VALUES ('cursor_up',1),
 ('cursor_left',2),
 ('cursor_right',3),
 ('cursor_down',4),
 ('space',5),
 ('control',6);
INSERT INTO "global_audiosettings" ("audioSetting","value","defaultValue") VALUES ('SoundEffectsVolume',100,100),
 ('musicVolume',100,100),
 ('masterVolume',100,100);
INSERT INTO "global_resolution_settings" ("settingname","value","defaultValue") VALUES ('resX',800,800),
 ('resY',600,600);
COMMIT;
