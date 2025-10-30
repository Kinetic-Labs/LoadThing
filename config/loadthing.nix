{
  # Configure the proxy
  proxy = {
    # Your application (preferably local network)
    target = "https://neko.thoq.dev";

    # Port to use (443 is default for https)
    port = 443;

    # Path to fetch ('/' is the root of page)
    path = "/";
  };

  # LoadThing's web server config
  web = {
    # Port to serve app on
    port = 9595;

    # Hostname to serve app on
    hostname = "127.0.0.1";
  };

  # Configure extra features
  features = {
    # Log each request
    log = true;
  };
}
