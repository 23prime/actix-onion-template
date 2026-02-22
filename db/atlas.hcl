env "local" {
  url = getenv("DATABASE_URL")
  src = "file://schema.sql"
  dev = getenv("DEV_DATABASE_URL")
}

env "test" {
  url = getenv("TEST_DATABASE_URL")
  src = "file://schema.sql"
  dev = getenv("DEV_DATABASE_URL")
}
