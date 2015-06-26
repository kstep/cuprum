Nginx configuration example:

```nginx
    server {
        listen 9898;

        root /home/kstep/git/cuprum/client/dest;

        location / {
            try_files $uri @backend;
        }

        location @backend {
            include scgi_params;
            scgi_pass localhost:9000;
        }
    }
```
