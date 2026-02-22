env "local" {
  url = getenv("DATABASE_URL")
  migration {
    dir = "file://db/migrations"
  }
}
