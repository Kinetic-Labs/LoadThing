{
  # Configure the proxy
  proxy = {
    # Your application (preferably local network)
    target = "http://127.0.0.1";

    # Port to use (80 is default for http)
    port = 5000;

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

    # Time the response of each request
    time = true;
  };
}
