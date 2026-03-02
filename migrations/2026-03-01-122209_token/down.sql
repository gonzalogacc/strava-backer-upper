-- This file should undo anything in `up.sql`

ALTER TABLE "token" DROP COLUMN "id";
ALTER TABLE "token" ADD COLUMN "id" INT4 NOT NULL;

