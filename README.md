cndk8 - pronounced Syndicate :D 
# bottle2telegram

Throw a message in a bottle to your telegram account

## Architecture

Application spawns a webservice and bot is a separate service, both start together and are stopped together.

### Everything is a plugin

- Plugins are separated in two components, web and bot functionality, ipc communication is defined with a json schema (possibly), plugins are self contained and should be plug and play, an application restart is required to make them work.

## Features

- Receive text via post, send to telegram, markdown is supported
  - Save in database
- Receive links via post, send to telegram
  - Save in database later (make configurable)
- When it's a twitter account, download files to hard drive
- Receive files, send to telegram
  - Build drag and drop for file upload
  - List uploaded files for a user
- Rate limiting is built in for unauthenticated requests.
