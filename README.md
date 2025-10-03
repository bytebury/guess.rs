# crust 🍕

> Proudly built in the [🍕 state](https://portal.ct.gov/).

Build like bytebury. The official template that we use for our websites, skip
all the bootstrapping. We are not suggesting that this is
the right way to create an application, this is just _our way_ and we're sharing
with everybody. We favor simplicity in developer
experience over processes and tools. Hence, running locally is a single command:
`./dev.sh` and deploying is done automatically through
GHA. By keeping local development simple, as well as our architecture, we can
move fast &mdash; relying on Rust's speed to match on the
server. It's how we built [Fun Banking](https://fun-banking.com),
and continue to build more.

## Some Background

This is the template that we use at bytebury. Our primary stack is [Axum](https://github.com/tokio-rs/axum),
[Askama](https://github.com/askama-rs/askama),
[HTMX](https://github.com/bigskysoftware/htmx), and [SQLite](https://sqlite.org/)
through [SQLx](https://github.com/launchbadge/sqlx). We run all of our servers
on [DigitalOcean](https://www.digitalocean.com/) on
various server sizes, so you'll notice some deployment workflows for
DigitalOcean (feel free to change that to your liking).
We use [Stripe](https://stripe.com) as our payment partner and
[Google](https://google.com) for our OAuth (extensible).

## Getting Started

> [!NOTE]
> Running `./dev` will run the application in watch mode for you as well as run
> any pending migrations!

1. Clone the repository `git clone git@github.com:bytebury/crust.git`
2. Run the development server `cd ./crust && ./dev.sh` in your terminal

This will run all of your migrations as well as generate a `.env` file in your
root directory. Open it up and change the environment variables to your liking.
After that, you should be ready to start development.

## Creating a Migration

> [!NOTE]
> You will need sqlx installed locally to create migrations.

```sh
sqlx migrate add create_my_table
```

## Listen to Stripe Events

You can use the Stripe CLI to listen to the webhooks locally. To do that,
you'll need to download the Stripe CLI and then run the following
command in a separate terminal:

```sh
stripe listen --forward-to localhost:8080/webhooks/stripe
```

## IP Services

We use [IP2Location LITE](https://lite.ip2location.com/database-download) for
our IP services. If you plan on following suite, know that they require
you to attribute them by adding the following HTML to your website.

```html
<p>
  {{ shared.app_info.name }} uses the IP2Location LITE database for
  <a class="underline" href="https://lite.ip2location.com" target="_blank">IP geolocation</a>.
</p>
```

## RBAC

At this time, we deal with Role Based Access Control (RBAC) through the `Can`
trait. This trait has a singular function that you can implement like so:

```rs
impl Can<Resource> for User {
  fn can(&self, action: Action, resource: &Resource) -> bool {
    match self.role {
      Role::Admin => true, // Admins are super-users.
      Role::User => match action {
        Action::Read => true,
        Action::Create => true,
        Action::Update | Action::Delete => resource.user_id == self.id,
      }
    }
  }
}

// Usage:

if user.can(Action::Delete, &article) {
  // success
}

if user.cannot(Action::Delete, &article) {
  // uh-oh you can't do that.
}
```

We decided to do this due to simplicity in our applications. We almost never
need granular access through the database &mdash; most of the times, roles
suffice for our use-cases.

## Creating a Server from Scratch

When you are creating a server from scratch, you'll typically need to configure
your Nginx, or at least, we do at bytebury. So, we've included a script that
does the set up we use: SSL + HTTP/2 enabled via `server-setup.sh`. This is
a one-time command. Once you have your server already configured, you won't
need this again. However, we've included it so it can streamline those who use
it.

## Our Philosophy

* Focus on what matters &mdash; off-load often
* Be close with the browser &mdash; it probably already does it
* Simple cloud architecture &mdash; your server can handle it
* Simple dev experience &mdash; doesn't mean "cool"
* If people don't ask for it &mdash; they probably don't want it
