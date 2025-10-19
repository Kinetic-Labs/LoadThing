{
  # Configure the proxy
  proxy = {
    # Your application (preferably local network)
    target = "https://example.com";
    port = 443;
    path = "/";
  };

  # LoadThing's config
  load_thing = {
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
