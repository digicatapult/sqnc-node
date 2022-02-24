const pino = require('pino')
const env = require('./env')

const logger = pino(
  {
    name: process.env.LOG_SERVICE_NAME || 'API',
    level: env.LOG_LEVEL,
    redact: {
      paths: ['USER_URI', '[*].USER_URI'],
      censor: (args) => {
        if (args === '' || args === null || args === undefined) {
          return '[EMPTY]'
        } else {
          return '[REDACTED]'
        }
      },
    },
  },
  process.stdout
)

logger.debug('Env: %j', env)

module.exports = logger
