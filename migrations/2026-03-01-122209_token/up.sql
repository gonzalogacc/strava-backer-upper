-- Your SQL goes here

ALTER TABLE "token" DROP COLUMN "id";
ALTER TABLE "token" ADD COLUMN "id" INT8 NOT NULL;

