-- Your SQL goes here
CREATE TABLE "users" (
    "id" TEXT NOT NULL PRIMARY KEY,
    "email" TEXT NOT NULL,
    "name" TEXT NOT NULL,
    "gender" BOOLEAN NOT NULL,
    "age" INTEGER NOT NULL
);

CREATE UNIQUE INDEX "users_email_key" ON "users"("email");