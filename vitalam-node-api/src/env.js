const envalid = require('envalid')
const dotenv = require('dotenv')

if (process.env.NODE_ENV === 'test') {
  dotenv.config({ path: 'test/test.env' })
} else {
  dotenv.config({ path: '.env' })
}

const vars = envalid.cleanEnv(process.env, {
  API_HOST: envalid.host({ devDefault: 'localhost' }),
  API_PORT: envalid.port({ default: 9944 }),
  LOG_LEVEL: envalid.str({ default: 'info', devDefault: 'debug' }),
  LEGACY_METADATA_KEY: envalid.str({ default: '' }),
  METADATA_KEY_LENGTH: envalid.num({ default: 32 }),
  METADATA_VALUE_LITERAL_LENGTH: envalid.num({ default: 32 }),
  PROCESS_IDENTIFIER_LENGTH: envalid.num({ default: 32 }),
})

module.exports = {
  ...vars,
}
