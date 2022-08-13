# Shortify

Shortify is my successor to [SimpleShortener](https://github.com/randomairborne/simpleshortener). It uses Cloudflare KV instead of a SQL database, so you can run it at the Serverless™ Edge™ Cloud™ and Scale™ easily.  It doesn't have a panel, you control it through the Workers KV dashboard.
<br>
Please note during setup that you must change the account ID, kv namespace, and all keys must be of the format `/path/to/thing` or `/` and values must be full urls.