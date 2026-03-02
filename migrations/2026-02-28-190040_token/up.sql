-- Your SQL goes here

CREATE TABLE "token"(
	"id" INTEGER NOT NULL PRIMARY KEY,
	"expires_at" INTEGER NOT NULL,
	"expires_in" INTEGER NOT NULL,
	"token_type" TEXT NOT NULL,
	"refresh_token" TEXT NOT NULL,
	"access_token" TEXT NOT NULL
);

