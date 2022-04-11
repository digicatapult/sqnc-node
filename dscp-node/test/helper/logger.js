const pino = require('pino')

const logger = pino(
  {
    name: 'API',
    level: process.env.LOG_LEVEL,
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

module.exports = logger
