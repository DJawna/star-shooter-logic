BEGIN TRANSACTION;
CREATE TABLE IF NOT EXISTS "global_resolution_settings" (
	"settingname"	TEXT NOT NULL,
	"value"	INTEGER NOT NULL CHECK(value>=0),
	"defaultValue"	INTEGER NOT NULL CHECK(defaultValue>=0),
	PRIMARY KEY("settingname")
);
CREATE TABLE IF NOT EXISTS "global_audiosettings" (
	"audioSetting"	TEXT,
	"value"	INTEGER CHECK(value<=100 and value>=0),
	"defaultValue"	INTEGER CHECK(value<=100 and value>=0),
	PRIMARY KEY("audioSetting")
);
INSERT INTO "global_resolution_settings" ("settingname","value","defaultValue") VALUES ('resX',800,800),
 ('resY',600,600);
INSERT INTO "global_audiosettings" ("audioSetting","value","defaultValue") VALUES ('SoundEffectsVolume',100,100),
 ('musicVolume',100,100),
 ('masterVolume',100,100);
COMMIT;
