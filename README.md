# Order Matching Exchange
Simple order matching engine built in Rust using Redis and PostgreSQL as data store.

## Concepts
Engine parses a contract config. It will then generate a `Matcher` thread for each contract which wait until they get orders passed in from the main thread.

Each matcher is synchronous so no queueing is required.