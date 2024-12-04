# Deployer

<div align="left">
    <img src="https://img.shields.io/badge/Rust-DDA484?logo=Rust&logoColor=white" />
</div>

Don't waste your time on deploying your projects! <br />
Deployer will check your repository's branch for new commits
and automatically pull your project into the specified directory.

<br/>

Note: Supports only projects from GitHub (will be fixed in the future)!
Note 2: In the config file, use only global paths to directories.

## Documentation

### Configuration

Firstly you want to start with creating and modifying configuration file for Deployer. <br />
To generate new configuration file template, use the following command:

```Bash
deployer config /path/to/config
```

### Make it up and running

Once you have written the configuration file, you can run Deployer with this command:

```Bash
deployer run /path/to/config
```

Deployer will check your repository for new commits every 60 seconds.

## Example `deployer-config.jsonc`

This is an example configuration `jsonc` file.  
Note that if project already **is** in the `build_dir`, it
would `rm -rf` that project.

```jsonc
{
  // the "https://" part is optional
  "repository": "https://github.com/Makefolder/deployer",
  "branch": "main",
  "token": "tokentokenmysweettoken",
  "pull_dir": "/usr/meykfolduh/var/my-pulls",
  "sys_svc_dir": "/lib/systemd/system",
  "services": [
    {
      "name": "backend",
      "svc_filename": "backend.service",
      "build_dir": "/usr/meykfolduh/var/production",
      // What if your project has multiple backends/main files
      // such as micro-services. Those are located on different
      // directories.
      // Example: my-repo/backend/v2
      "custom_dir": "backend/v2", // nullable
      "svc_file_contents": [
        "[Unit]",
        "Description=Your service description.",
        "",
        "[Service]",
        "And so on..."
      ]
    },
    {
      "name": "backend",
      "svc_filename": "backend.service",
      "build_dir": "/usr/meykfolduh/var/production",
      // my-repo/file-server
      "custom_dir": "file-server",
      "svc_file_contents": [
        "[Unit]",
        "Description=Some FS description.",
        "",
        "[Service]",
        "And so on..."
      ]
    }
  ]
}
```
