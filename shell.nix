{ pkgs ? import <nixpkgs> {} }:
   let
     mozilla-overlay = import (builtins.fetchTarball "https://github.com/mozilla/nixpkgs-mozilla/archive/master.tar.gz");
     pkgs = import <nixpkgs> { overlays = [ mozilla-overlay ]; };
     rustNightly = (pkgs.latest.rustChannels.nightly.rust.override {
       extensions = [ "rust-src" ];
     });
   in
pkgs.mkShell {
  nativeBuildInputs = [
    rustNightly
    pkgs.pkg-config
    pkgs.postgresql  # PostgreSQL server and client tools
  ];

  buildInputs = [
    pkgs.openssl
    pkgs.postgresql.lib  # PostgreSQL library for linking
  ];

  # PostgreSQL environment variables
  PGDATA = "./.postgres";  # Database storage directory
  PGPORT = "5432";         # PostgreSQL port
  PGUSER = "dev";
  PGPASSWORD = "secretpassword";
  PGDB = "levels";

  # Enable SQLx offline mode
  SQLX_OFFLINE = "true";
  # Set environment variables for OpenSSL
  PKG_CONFIG_PATH = "${pkgs.openssl.dev}/lib/pkgconfig";
  OPENSSL_DIR = "${pkgs.openssl.dev}";
  OPENSSL_LIB_DIR = "${pkgs.openssl.out}/lib";
  OPENSSL_INCLUDE_DIR = "${pkgs.openssl.dev}/include";

  shellHook = ''
    echo "Rust nightly environment ready"
    echo "SQLX_OFFLINE mode is enabled"
    
    # Create the runtime directory PostgreSQL expects
    export PGRUN=$(mktemp -d)
    mkdir -p $PGRUN
    
    # Initialize PostgreSQL database if it doesn't exist
    if [ ! -d "$PGDATA" ]; then
      echo "Initializing PostgreSQL database..."
      initdb --auth=trust --no-locale --encoding=UTF8
      
      # Configure PostgreSQL to use our custom runtime directory
      echo "unix_socket_directories = '$PGRUN'" >> $PGDATA/postgresql.conf
      echo "PostgreSQL database initialized"
    fi
    
    # Start PostgreSQL when entering the shell
    pg_ctl start -o "-k $PGRUN" -l "$PGDATA/postgres.log"

    # Create user and database if they don't exist
    echo "Setting up PostgreSQL user and database..."
    psql -h $PGRUN -d postgres -U $(whoami) -c "CREATE USER $PGUSER WITH PASSWORD '$PGPASSWORD';"
    psql -h $PGRUN -d postgres -U $(whoami) -c "CREATE DATABASE $PGDB WITH OWNER $PGUSER;" 

    # Set default environment variables for database connections
    export DATABASE_URL="postgresql://$PGUSER:$PGPASSWORD@localhost:$PGPORT/$PGDB"

    echo "PostgreSQL is running on port $PGPORT"
    echo "Database '$(basename "$PWD")' is available"
    echo "Socket directory: $PGRUN"
    
    # Function to stop PostgreSQL when exiting the shell
    function stop_postgres() {
      echo "Stopping PostgreSQL..."
      pg_ctl stop -o "-k $PGRUN"
      rm -rf $PGRUN
    }
    trap stop_postgres EXIT
  '';
}
