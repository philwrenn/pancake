CREATE TABLE "open_preferences"
(
    "id"      INTEGER,
    "search"  TEXT   NOT NULL,
    "browser_key" TEXT    NOT NULL,
    "exact"   INTEGER NOT NULL DEFAULT 0,
    PRIMARY KEY ("id" AUTOINCREMENT)
);