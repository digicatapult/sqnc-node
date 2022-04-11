const envalid = require('envalid')
const dotenv = require('dotenv')
const path = require('path')

if (process.env.NODE_ENV === 'test') {
  dotenv.config({ path: path.resolve(__dirname, '../test/test.env') })
} else {
  dotenv.config({ path: path.resolve(__dirname, '../.env') })
}

const vars = envalid.cleanEnv(process.env, {
  METADATA_KEY_LENGTH: envalid.num({ default: 32 }),
  METADATA_VALUE_LITERAL_LENGTH: envalid.num({ default: 32 }),
  PROCESS_IDENTIFIER_LENGTH: envalid.num({ default: 32 }),
})

module.exports = vars
