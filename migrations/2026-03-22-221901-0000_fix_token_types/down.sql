-- Revert types to 4-byte integers
ALTER TABLE "token" ALTER COLUMN "expires_at" TYPE INT4;
ALTER TABLE "token" ALTER COLUMN "expires_in" TYPE INT4;

-- Remove the Primary Key
ALTER TABLE "token" DROP CONSTRAINT token_pkey;
