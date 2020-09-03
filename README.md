<h1 align="center">
  <img src="assets/screenshot.png?raw=true">
<br/>
Rakaly Desktop
</h1>

**A WIP prototype desktop GUI for Rakaly functionality**

Not ready for consumption

Goals:

- **Lightweight**: Rakaly Desktop must use a minimum of resources (CPU and RAM) so as to not impact gameplay.
- **Cross platform**: Any platform that can play EU4 can use Rakaly Desktop
- **Service**: The app should be able to tie into a windows service so that it can start on boot

Right now there are two subcommands implemented:

- `run`: Watches the configured directory (or if not configured, the current working directory) for EU4 file changes and then upload to an endpoint. If the EU4 file is a zip file, it is uploaded verbatim, else text files are gzipped before upload.
- `gui`: playground for the same thing but in GUI form.
