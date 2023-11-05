import { cac } from 'cac'
import { command } from "../index.js"

const cli = cac();
cli.command('install [version]', 'install node version').action((version, options) => {
  command("install", version)
})

cli.help()
cli.version('0.0.0')

cli.parse()