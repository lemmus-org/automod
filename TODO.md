# TODOs

## General Improvements
* Proper logging
  * Should support both text and JSON output
* Base Docker image
  * For quick deployments and PR testing
* PR validation
* Unit tests
* Documentation
* API Pagination
* Cron-syntax scheduling for plugins
  * Haven't found a well-maintained solution
* Support debug/dry-run mode for validation testing

## Plugins
* Post scheduler
  * Could also support managing featured post rotations
* Post URL allowlist
  * Remove any posts linking to URLs that aren't allowed
* Registration applications
* Private message
  * Add support for notifying admins whenever a user sends it a message.
  * Add support for common moderator actions.

## Ideas
* Implement support for better notifications
  * e.g. PMs, Discord, Matrix, email, etc...