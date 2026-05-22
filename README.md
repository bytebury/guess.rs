# [guess.rs](https://guesser.up.railway.app/)

> Proudly built in the [🍕 state](https://portal.ct.gov/).

## About
Tired of planning poker apps with limitations, ads, or high prices, we built our own simplified tool. It focuses on straightforward estimates, helping teams concentrate and fostering thoughtful discussions to unlock deeper work insights.

Try it out at https://guesser.up.railway.app.

## Local development

```sh
./dev.sh
```

This generates a `.env` (see `.env.example` for reference) and a local SQLite DB under `db/`, then starts the server with `cargo watch`.

## Deploy to Railway

The app ships with everything needed to deploy on Railway as a single service backed by SQLite on a Railway Volume. SQLite + a small volume is the cheapest option — no separate managed database to pay for.

1. **Create a new Railway project** from this repo. Railway will detect `railway.json` and `Dockerfile` and use the Dockerfile builder.
2. **Attach a Volume to the service** with mount path `/data`. This is where the SQLite file lives across deploys.
3. **Set environment variables** in the service (Variables tab):
   - `DATABASE_URL=sqlite:///data/database.db`
   - `APP_NAME=Guess`
   - `APP_WEBSITE_URL=https://<your-railway-domain>`
   - `JWT_SECRET=<something secret>`
   - `COOKIE_URL=<your-railway-domain>`
   - `GOOGLE_CLIENT_ID`, `GOOGLE_CLIENT_SECRET`, `GOOGLE_CALLBACK_URL` (if using Google auth)
   - `STRIPE_SECRET`, `STRIPE_WEBHOOK_SECRET`, `STRIPE_PRICE_ID` (if using Stripe)

   `PORT` is injected by Railway automatically — don't set it.
4. **Deploy.** The app auto-creates the SQLite file and runs migrations on startup.

### Why SQLite on a volume?
Railway charges for compute time and volume storage. A managed Postgres instance is a second always-on service. For a small/medium app, SQLite + WAL on a persistent volume costs less and is plenty fast. If you outgrow it, swap `DATABASE_URL` for a Postgres URL and migrate the schema — `sqlx` supports both, though the queries here are SQLite-flavored and would need review.

## Attributions
* https://weirder.earth/@dzuk/102711286545743862 (Icons)
