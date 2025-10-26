## Overview
This is the reference implementation of the api 302.
This api has three objects: user, location and trust
any user can add trusted people to their list of trustees, but every information is opt in.
Users can add information to help others (trusted people and authorities) help.

This server has no own frontend.
It is just the public api component, used for different events.

## Important components
rust, axum, sqlite/sqlx

## Roadmap
- implement more fine control on the kind of information, each person may see
